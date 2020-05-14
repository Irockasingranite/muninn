use crate::data::{Data, DataSlice};
use std::time::{Instant};

#[derive(Clone)]
pub struct State {
    pub current_step: usize, // currently displayed timestep
    pub current_time: f64, // time value of displayed timestep
    pub n_steps: usize, // total number of timesteps in loaded data
    pub times: Vec<f64>, // time values for all steps
    pub update_interval: i32, // in ms
    pub timestep_interval: usize, // allows skipping timesteps
    loaded_data: Option<Data>, // Currently loaded dataset
    current_slice: Option<DataSlice>, // Slice for current timestep
    pub is_playing: bool, // Whether the plot is being animated
    last_step_made_at: Instant, // time when last frame was rendered
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
        }
    }

    pub fn load_data(&mut self, data: Data) {
        let mut all_times = Vec::new();
        for ts in data.timeseries.iter() {
            let mut t = ts.times.clone();
            all_times.append(&mut t);
        }
        all_times.sort_by(|x, y| x.partial_cmp(y).unwrap());
        all_times.dedup();
        let current_time = *all_times.first().expect("No time steps in data");

        self.n_steps = all_times.len();
        self.times = all_times;
        self.current_time = current_time;
        self.current_slice = Some(data.at_time(current_time));
        self.loaded_data = Some(data);
    }

    pub fn from_data(data: Data) -> State {
        let mut state = State::new();
        state.load_data(data);
        state
    }

    pub fn advance_animation(&mut self) {
        let now = Instant::now();
        let time_since_last_step = now.duration_since(self.last_step_made_at);
        if time_since_last_step.as_millis() > self.update_interval as u128 {
            // advance one or more steps
            let target_step = std::cmp::min(self.current_step + self.timestep_interval, self.n_steps-1);
            let target_time = self.times[target_step];
            self.current_slice = Some(self.loaded_data.as_ref().unwrap().at_time(target_time));
            self.current_step = target_step;
            self.current_time = target_time;
            self.last_step_made_at = Instant::now();
            println!("Making Step! t = {}", self.current_time);
        }
    }

    pub fn jump_to_time(&mut self, time: f64) {
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
            let target_step = self.times.iter().rposition(|t| target_time >= *t).unwrap();
            self.current_step = target_step;
            self.current_slice = Some(d.at_time(target_time));
            self.current_time = target_time;
        }
    }
}