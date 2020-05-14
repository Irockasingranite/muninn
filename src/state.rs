use crate::data::{Data, DataSlice};

#[derive(Clone)]
pub struct State {
    pub current_step: usize, // currently displayed timestep
    pub current_time: f64, // time value of displayed timestep
    pub n_steps: usize, // total number of timesteps in loaded data
    pub times: Vec<f64>, // time values for all steps
    pub update_interval: i32, // in ms
    pub timestep_interval: i32, // allows skipping timesteps
    loaded_data: Option<Data>, // Currently loaded dataset
    current_slice: Option<DataSlice>, // Slice for current timestep
    pub is_playing: bool, // Whether the plot is being animated
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
}