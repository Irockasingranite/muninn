use crate::data::{Data, DataSlice};
use crate::plotting::{PlotRange, PlotSettings};
use std::time::Instant;
use std::sync::{Arc, Mutex};
use gdk_pixbuf::Pixbuf;

pub enum PlotStatus {
    Idle,
    Working,
    Finished(Option<(String, (PlotRange, PlotRange))>),
}

pub struct State {
    pub current_step: usize, // currently displayed timestep
    pub current_time: f64, // time value of displayed timestep
    pub n_steps: usize, // total number of timesteps in loaded data
    pub times: Vec<f64>, // time values for all steps
    pub update_interval: i32, // in ms
    pub timestep_interval: usize, // allows skipping timesteps
    pub loaded_data: Option<Data>, // Currently loaded dataset
    pub current_slice: Option<DataSlice>, // Slice for current timestep
    pub is_playing: bool, // Whether the plot is being animated
    last_step_made_at: Instant, // time when last frame was rendered
    pub update_needed: bool, // whether the current image needs to be updated
    pub plot_settings: PlotSettings,
    pub plot_range_x_actual: PlotRange,
    pub plot_range_y_actual: PlotRange,
    pub plot_area_size: (u32, u32),
    pub plot_image_string: Option<String>,
    pub plot_image_pixbuf: Option<Pixbuf>,
    pub mouse_state: MouseState,
}

impl State {
    pub fn new() -> State {
        State {
            current_step: 0,
            current_time: 0.0,
            n_steps: 0,
            times: Vec::new(),
            update_interval: 100,
            timestep_interval: 1,
            loaded_data: None,
            current_slice: None,
            is_playing: false,
            last_step_made_at: Instant::now(),
            update_needed: true,
            // plot_range_setting: PlotRange::Auto,
            plot_settings: PlotSettings::new(),
            plot_range_x_actual: PlotRange::Auto,
            plot_range_y_actual: PlotRange::Auto,
            plot_area_size: (600,400),
            plot_image_string: None,
            plot_image_pixbuf: None,
            mouse_state: MouseState::new(),
        }
    }

    pub fn load_data(&mut self, data: Data) {
        let times = data.times();
        
        let current_time = match times.first() {
            Some(t) => *t,
            None => 0.0,
        };

        self.n_steps = times.len();
        self.times = times;
        self.current_time = current_time;
        self.current_slice = Some(data.at_time(current_time));
        self.loaded_data = Some(data);
    }

    pub fn advance_animation(&mut self) {
        let now = Instant::now();
        let time_since_last_step = now.duration_since(self.last_step_made_at);
        if time_since_last_step.as_millis() > self.update_interval as u128 {
            // advance one or more steps
            let target_step = std::cmp::min(self.current_step + self.timestep_interval, self.n_steps-1);
            self.go_to_step(target_step);
            self.last_step_made_at = Instant::now();
        }
    }

    pub fn jump_to_time(&mut self, time: f64) -> Option<f64> {
        if let Some(d) = &self.loaded_data {
            // clamp target time to available data
            let mut target_time = time;
            if target_time < d.start_time {
                target_time = d.start_time;
            }
            if target_time > d.end_time {
                target_time = d.end_time;
            }

            // find correct target step
            // We also 'correct' target_time to where we actually jump to
            let target_step = self.times.iter().rposition(|t| target_time >= *t).unwrap();

            if target_step < self.times.len() {
                let time_before_target = self.times[target_step];
                let time_after_target = self.times[target_step+1];
                target_time = if (target_time - time_before_target) < (time_after_target - target_time) {
                    self.times[target_step]
                } else {
                    self.times[target_step+1]
                }
            }

            self.current_step = target_step;
            self.current_slice = Some(d.at_time(target_time));
            self.current_time = target_time;
            self.update_needed = true;
            return Some(target_time);
        }
        None
    }

    /// Set state to a specified time steps. Returns time at that step. Returns None if no steps are loaded.
    pub fn go_to_step(&mut self, step: usize) -> Option<f64> {
        if let Some(d) = &self.loaded_data {
            let mut target_step = step;
            if target_step >= self.n_steps {
                target_step = self.n_steps - 1;
            }
            self.current_step = target_step;
            let target_time = self.times[target_step];
            self.current_slice = Some(d.at_time(target_time));
            self.current_time = target_time;
            self.update_needed = true;
            Some(target_time)
        } else {
            None
        }
    }

    /// Set state to the first timestep in the loaded dlinesata. Returns the time on that step. Returns None if no steps are loaded.
    pub fn go_to_first_step(&mut self) -> Option<f64> {
        self.go_to_step(0)
    }

    /// Set state to previous timestep in the loaded data. Returns the time on that step. Returns None if no steps are loaded.
    pub fn go_to_previous_step(&mut self) -> Option<f64> {
        let target_step = if self.current_step > 0 {
            self.current_step - 1
        } else {
            0
        };
        self.go_to_step(target_step)
    }

    /// Set state to next timestep in the loaded data. Returns the time on that step. Returns None if no steps are loaded.
    pub fn go_to_next_step(&mut self) -> Option<f64> {
        let step = self.current_step + 1;
        self.go_to_step(step)
    }

    /// Set state to the last timestep in the loaded data. Returns the time on that step. Returns None if no steps are loaded.
    pub fn go_to_last_step(&mut self) -> Option<f64> {
        let target_step = if self.n_steps > 0 {
            self.n_steps - 1
        } else {
            0
        };
        self.go_to_step(target_step)
    }

    pub fn request_plot(&mut self, status_mutex: Arc<Mutex<PlotStatus>>) -> bool {
        use crate::plotting::plot_data_slice_to_svg;
        if let Some(d) = &self.loaded_data {
            let s = match &self.current_slice {
                None => {
                    d.at_time(self.current_time)
                },
                Some(s) => s.clone()
            };

            let settings = self.plot_settings.clone();
            let size = self.plot_area_size.clone();

            rayon::spawn(move || {
                {
                    let mut status_locked = status_mutex.lock().unwrap();
                    *status_locked = PlotStatus::Working;
                }
                let (string, ranges) = plot_data_slice_to_svg(&s, &settings, &size);
                let mut status_locked = status_mutex.lock().unwrap();
                *status_locked = PlotStatus::Finished(Some((string, ranges)));
            });
            return true;
        }
        false
    }
}

pub struct MouseState {
    pub position: (f64, f64),
    pub last_position: (f64, f64),
    pub left_button_held: bool,
    pub middle_right_button_held: bool,
    pub drag_start: (f64, f64),
}

impl MouseState {
    pub fn new() -> Self {
        MouseState {
            position: (0.0, 0.0),
            last_position: (0.0, 0.0),
            left_button_held: false,
            middle_right_button_held: false,
            drag_start: (0.0, 0.0),
        }
    }
}