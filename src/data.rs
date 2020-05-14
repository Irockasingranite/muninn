mod timeseries;
pub use timeseries::{TimeSeries, DataLine, Point};

pub type DataSlice = (Vec<String>, Vec<DataLine>);

#[derive(Clone)]
pub struct Data {
    pub filenames: Vec<String>,
    pub timeseries: Vec<TimeSeries>,
}

impl Data {
    pub fn new() -> Data {
        Data{
            filenames: Vec::new(),
            timeseries: Vec::new(),
        }
    }

    pub fn from_files(filenames: Vec<String>) -> Data {
        let mut data = Data::new();

        for filename in filenames {
            if let Ok(ts) = TimeSeries::from_file(&filename) {
                data.filenames.push(filename);
                data.timeseries.push(ts);
            }
        }

        data
    }

    pub fn at_time(&self, time: f64) -> DataSlice {
        let mut filenames = Vec::new();
        let mut lines = Vec::new();

        for (i, ts) in self.timeseries.iter().enumerate() {
            if let Some(line) = ts.at_time(time) {
                lines.push(line.clone());
                filenames.push(self.filenames[i].clone());
            }
        }

        (filenames, lines)
    }
}