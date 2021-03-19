mod timeseries;
pub use timeseries::{TimeSeries, DataLine, Point};

#[derive(Clone)]
pub struct DataSlice {
    pub filenames: Vec<String>,
    pub datalines: Vec<DataLine>,
}

impl DataSlice {
    pub fn to_string_gnuplot(&self) -> String {
        let mut gnuplot_string = String::new();

        for dataline in &self.datalines {
            for point in dataline {
                gnuplot_string.push_str(&format!("{:.15e}\t{:.15e}\n", point.0, point.1));
            }
            gnuplot_string.push_str("\n\n");
        }

        gnuplot_string
    }
}

#[derive(Clone)]
pub struct Data {
    pub filenames: Vec<String>,
    pub timeseries: Vec<TimeSeries>,
    pub start_time: f64,
    pub end_time: f64,
}

impl Data {
    fn new() -> Data {
        Data{
            filenames: Vec::new(),
            timeseries: Vec::new(),
            start_time: 0.0,
            end_time: 0.0,
        }
    }

    pub fn from_files(filenames: Vec<String>) -> Option<Data> {
        let mut data = Data::new();
        let mut start_time = std::f64::MAX;
        let mut end_time = std::f64::MIN;

        for filename in filenames {
            if let Ok(ts) = TimeSeries::from_file(&filename) {
                if ts.start_time < start_time {
                    start_time = ts.start_time;
                }
                if ts.end_time > end_time {
                    end_time = ts.end_time;
                }
                data.filenames.push(filename);
                data.timeseries.push(ts);
            }
        }

        if data.timeseries.is_empty() {
            return None;
        }

        data.start_time = start_time;
        data.end_time = end_time;

        if data.timeseries.is_empty() {
            None
        } else {
            Some(data)
        }
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

        DataSlice{
            filenames,
            datalines: lines,}
    }
}