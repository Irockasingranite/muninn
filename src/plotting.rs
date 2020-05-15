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
