use std::error::Error;
use std::fmt;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type Time = f64;

pub type Point = (f64, f64);
pub type DataLine = Vec<Point>;

#[derive(Clone)]
pub struct DataSlice {
    pub time: f64,
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

#[derive(Debug)]
pub struct DatafileReadError {
    filename: String,
}

impl fmt::Display for DatafileReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not a valid datafile: {}", self.filename)
    }
}

impl Error for DatafileReadError {}

#[derive(Clone)]
pub struct Data {
    pub dataslices: Vec<DataSlice>,
    pub start_time: f64,
    pub end_time: f64,
}

impl Data {
    fn new() -> Data {
        Data{
            dataslices: Vec::new(),
            start_time: 0.0,
            end_time: 0.0,
        }
    }

    pub fn from_files(filenames: Vec<String>) -> Option<Data> {
        let mut data = Data::new();

        // Collect data from all files, tagged with their time values
        let mut time_line_pairs = Vec::new();
        for filename in filenames {
            if let Ok(mut pairs) = read_datalines_from_file(&filename) {
                time_line_pairs.append(&mut pairs);
            }
        }

        // No data in files means no data for structure
        if time_line_pairs.is_empty() {
            return None;
        }

        // Sort all datalines by time
        time_line_pairs.sort_by(|(t1, _), (t2, _)| t1.partial_cmp(t2).unwrap());

        // Extract first and last time in data
        data.start_time = time_line_pairs.first().unwrap().0;
        data.end_time = time_line_pairs.last().unwrap().0;

        // Collect datalines by their time
        for (t, l) in time_line_pairs.into_iter() {
            // If a slice already exists and the time matches, add line to that
            // slice. Otherwise create a new one.
            if let Some(slice) = data.dataslices.last_mut() {
                if slice.time == t {
                    slice.datalines.push(l);
                } else {
                    data.dataslices.push(DataSlice {
                        time: t,
                        datalines: vec![l]
                    });
                }
            } else {
                data.dataslices.push(DataSlice {
                    time: t,
                    datalines: vec![l]
                });
            }
        }

        if data.dataslices.is_empty() {
            None
        } else {
            Some(data)
        }
    }

    pub fn at_time(&self, time: Time) -> DataSlice {
        let index_option = self.dataslices.iter().position(|s| s.time > time);
        if let Some(index) = index_option {
            self.dataslices[index as usize].clone()
        } else {
            self.dataslices.last().unwrap().clone()
        }
    }

    pub fn times(&self) -> Vec<Time> {
        self.dataslices.iter().map(|s| s.time).collect()
    }
}

fn read_datalines_from_file(filename: &str) -> Result<Vec<(Time, DataLine)>> {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut datalines: Vec<DataLine> = Vec::new();
    let mut times: Vec<f64> = Vec::new();

    for l in reader.lines() {
        let line = l?;
        if line.is_empty() {
            continue;
        }
        if line.starts_with('\"') {
            if line.starts_with("\"Time = ") {
                let time_str = line.get(8..).unwrap();
                let time = time_str.parse::<f64>().unwrap();
                times.push(time);
                // Begin a new dataline
                datalines.push(DataLine::new());
            }
          // Add points to the latest dataline  
        } else if let Some(dataline) = datalines.last_mut() {
            let mut words = line.trim().split_whitespace();
            if let (Ok(x), Ok(y)) = (words.next().unwrap().parse::<f64>(),
                                     words.next().unwrap().parse::<f64>()) {
                dataline.push((x,y));
            }
        }
    }

    // After reading all datalines, zip them together with their time values
    let mut time_line_pairs: Vec<(Time, DataLine)> = Vec::new();
    for (t, l) in times.iter().zip(datalines) {
        time_line_pairs.push((*t, l));
    }

    Ok(time_line_pairs)
}