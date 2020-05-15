use crate::data::DataSlice;

use plotters::prelude::*;

/// Take a vector of vectors of points, and plot them into an SVG file, returned as a String
// pub fn plot_data_slice_to_svg(data_slice: Vec<Vec<(f64,f64)>>) -> String
pub fn plot_data_slice_to_svg(data_slice: &DataSlice) -> String
{
    let (_filenames, data) = data_slice;

    // Parameters
    let point_size = 2; // Point Size
    let colors = vec![BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, YELLOW];
    let n_colors = colors.len();
    let padding = 0.05;

    // Figure out drawing area
    let mut xmaxs = Vec::new();
    let mut xmins = Vec::new();
    let mut ymaxs = Vec::new();
    let mut ymins = Vec::new();
    for series in data {
        xmaxs.push(series.iter().max_by(|(x1,_),(x2,_)| x1.partial_cmp(x2).unwrap()).unwrap().0);
        xmins.push(series.iter().min_by(|(x1,_),(x2,_)| x1.partial_cmp(x2).unwrap()).unwrap().0);
        ymaxs.push(series.iter().max_by(|(_,y1),(_,y2)| y1.partial_cmp(y2).unwrap()).unwrap().1);
        ymins.push(series.iter().min_by(|(_,y1),(_,y2)| y1.partial_cmp(y2).unwrap()).unwrap().1);
    }

    let xmin = *xmins.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
    let xmax = *xmaxs.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
    let ymin = *ymins.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
    let ymax = *ymaxs.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

    let dx = xmax - xmin;
    let dy = ymax - ymin;

    let xmin = xmin - padding * dx;
    let xmax = xmax + padding * dx;
    let ymin = ymin - padding * dy;
    let ymax = ymax + padding * dy;

    // Prepare empty plot
    let mut svg_string = String::new();
    {
        let root = SVGBackend::with_string(&mut svg_string, (600, 400)).into_drawing_area();
        // let root = SVGBackend::new("0.svg", (600,400)).into_drawing_area();
        root.fill(&WHITE).expect("Failed to fill canvas");
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(20)
            .y_label_area_size(50)
            .build_ranged(xmin..xmax, ymin..ymax)
            .expect("Failed to build chart");

        chart.configure_mesh().draw().expect("Failed to draw mesh");

        // plot each point vector seperately
        for (i, points) in data.iter().enumerate() {
            let color = &colors[i%n_colors];
            chart.draw_series(LineSeries::new(points.clone(), color))
                .expect("Failed to draw line");
            chart.draw_series(PointSeries::of_element(
                points.clone(),
                point_size,
                color,
                &|coord, size, style| {
                    return EmptyElement::at(coord)
                        + Circle::new((0,0), size, style.filled())
                }
                )).expect("Failed to draw points");
        }
    }    
    // Return "file"
    svg_string
}

/// Create a vector of f64's with equidistant values
fn float_range(xmin: f64, xmax: f64, n: usize) -> Vec<f64> {
    let d = xmax - xmin;
    let dx = d / ((n-1) as f64);
    let xs = (0..n).map(|x| x as f64 * dx + xmin).collect();
    xs
}

/// Create a test plot with 3 lines as an svg string
pub fn test_plot() -> String {
    let xs = float_range(-1.0, 1.0, 100);
    let points: Vec<(f64,f64)> = xs.iter().map(|&x| (x, (x*10.0).sin())).collect();
    let points2: Vec<(f64,f64)> = xs.iter().map(|&x| (x, 2.0*x)).collect();
    let points3: Vec<(f64,f64)> = xs.iter().map(|&x| (x, -2.0*x)).collect();
    let data = (vec![String::from("1"), String::from("2"), String::from("3")], vec![points, points2, points3]);
    let svg_string = plot_data_slice_to_svg(&data);
    // println!("{:?}", file);

    // { // write string into file
    //     use std::path::Path;
    //     use std::fs::File;
    //     use std::io::prelude::*;

    //     let file_path = Path::new("src/0.svg");
    //     let mut file = File::create(&file_path).expect("Failed to create file");
    //     file.write_all(svg_string.as_bytes()).expect("Failed to write file");
    // }

    return svg_string
}