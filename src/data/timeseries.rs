#![allow(dead_code)] // remove this later

pub type Point = (f64, f64);
pub type DataLine = Vec<Point>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct TimeSeries {
    times: Vec<f64>,
    start_time: f64,
    end_time: f64,
    data_lines: Vec<DataLine>,
}

impl TimeSeries {
    pub fn new() -> TimeSeries {
        TimeSeries{
            times: Vec::new(),
            start_time: 0.0,
            end_time: 0.0,
            data_lines: Vec::new(),
        }
    }

    /// Read in a file and parse it into a valid TimeSeries
    pub fn from_file(filename: &str) -> Result<TimeSeries> {
        use std::fs::File;
        use std::io::BufReader;
        use std::io::prelude::*;

        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut data_lines: Vec<DataLine> = Vec::new();
        let mut times: Vec<f64> = Vec::new();

        for l in reader.lines() {
            let line = l?;
            if line.is_empty() {
                continue;
            }
            if line.starts_with("\"") {
                if line.starts_with("\"Time = ") {
                    let time_str = line.get(8..).unwrap();
                    let time = time_str.parse::<f64>().unwrap();
                    times.push(time);
                    data_lines.push(DataLine::new());
                }
            }
            else  {
                if let Some(dataline) = data_lines.last_mut() {
                    let mut words = line.trim().split(" ");
                    let x = words.next().unwrap().parse::<f64>().unwrap();
                    let y = words.next().unwrap().parse::<f64>().unwrap();
                    let point = (x, y);
                    dataline.push(point);
                }
            }
        }       

        let start_time = *times.iter().min_by(|&x, &y| x.partial_cmp(y).unwrap()).unwrap();
        let end_time = *times.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        let series = TimeSeries{
            times,
            start_time: start_time,
            end_time: end_time,
            data_lines,
        };

        Ok(series)
    }

    pub fn at_time(&self, time: f64) -> Option<&DataLine> {
        if time > self.end_time || time < self.start_time {
            None
        } else {
            let index = self.times.iter().rposition(|t| time > *t).unwrap();

            Some(&self.data_lines[index as usize])
        }
    }
}