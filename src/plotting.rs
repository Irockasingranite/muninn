use crate::data::{DataSlice, Point};

use plotters::prelude::*;

pub type Range = (f64, f64);

#[derive(Debug, Clone, Copy)]
pub enum PlotRange {
    Auto,
    Fixed(Range),
}

#[derive(Debug, Clone)]
pub struct PlotSettings {
    pub plot_range_x: PlotRange,
    pub plot_range_y: PlotRange,
    pub draw_lines: bool,
    pub draw_points: bool,
    pub use_color: bool,
    pub use_logscale_x: bool,
    pub use_logscale_y: bool,
}

impl PlotSettings {
    pub fn new() -> PlotSettings {
        PlotSettings {
            plot_range_x: PlotRange::Auto,
            plot_range_y: PlotRange::Auto,
            draw_lines: true,
            draw_points: true,
            use_color: true,
            use_logscale_x: false,
            use_logscale_y: false,
        }
    }
}

fn point_in_area(p: &Point, x_range: &Range, y_range: &Range) -> bool {
    let (x,y) = p;
    let (xmin, xmax) = x_range;
    let (ymin, ymax) = y_range;

    (x >= xmin) & (x <= xmax) & (y >= ymin) & (y <= ymax)
}

/// Takes a line piece and two ranges, and moves the first and last points of the piece
/// to end exactly where the line crosses the boundary of the area specified by the ranges
fn truncate_line_segment(line_segment: &(Point, Point), x_range: &Range, y_range: &Range) -> Point {
    let (x_min, x_max) = *x_range;
    let (y_min, y_max) = *y_range;

    let ((x0, y0), (x1, y1)) = *line_segment;

    // check and modify in x-direction
    let x_bound = if x0 < x_min {
        Some(x_min) // piece crosses lower bound
    } else if x0 > x_max {
        Some(x_max) // piece crosses upper bound
    } else {
        None
    };

    let (x2, y2) = if let Some(b) = x_bound {
        let dx = x1 - x0;
        let dist_to_bound = b - x0;
        let scale_factor = dist_to_bound / dx;
        (x0 + scale_factor * (x1 - x0), y0 + scale_factor * (y1 - y0))
    } else {
        (x0, y0)
    };

    // check and modify in y-direction
    let y_bound = if y2 < y_min {
        Some(y_min) // piece crosses lower bound
    } else if y2 > y_max {
        Some(y_max) // piece crosses upper bound
    } else {
        None
    };

    if let Some(b) = y_bound {
        let dy = y1 - y2;
        let dist_to_bound = b - y2;
        let scale_factor = dist_to_bound / dy;
        return (x2 + scale_factor * (x1 - x2),
                y2 + scale_factor * (y1 - y2));
    }
    (x2, y2)
}

/// Takes a line of data points and returns a vector of lines that lie within the specified area
fn truncate_line(line: &[Point], x_range: &Range, y_range: &Range) -> Vec<Vec<Point>> {
    if line.is_empty() {
        return Vec::new();
    }

    // Determine where to split the line (each time it crosses the area border)
    let mut split_indices: Vec<usize> = Vec::new();
    for (i, p) in line.iter().enumerate().skip(1) {
        let current_in_area = point_in_area(p, x_range, y_range);
        let previous_in_area = point_in_area(&line[i-1], x_range, y_range);
        if current_in_area ^ previous_in_area { // logical XOR, means we crossed either into or out of the area
            split_indices.push(i);
        }
    }
    let n_splits = split_indices.len();

    // If no splits were detected, either all or none of the line is in the area
    if n_splits == 0 {
        if point_in_area(&line[0], x_range, y_range) {
            return vec![line.to_owned()];
        } else {
            return Vec::new();
        }
    }

    // Split line into pieces at the determined indices
    // (the pieces overlap to make sure each contains points on both sides of the edge)
    let mut line_pieces: Vec<&[Point]> = Vec::new();

    // TODO: make the next part more idiomatic with iterators instead of the C-style loop
    // (peakable iterator for accessing next index?)

    // first piece:
    let first_piece = &line[0..split_indices[0]+1];
    line_pieces.push(first_piece);
    // middle pieces:
    for i in 0..n_splits-1 {
        let current_split = split_indices[i];
        let next_split = split_indices[i+1];
        let line_piece = &line[current_split-1..next_split+1];
        line_pieces.push(line_piece);
    }
    // last piece:
    let last_piece = &line[split_indices[n_splits-1]-1..];
    line_pieces.push(last_piece);
    
    // Determine which pieces to discard and keep the rest
    let skip_index = match point_in_area(&line_pieces[0][0], x_range, y_range) {
        true  => 0,
        false => 1,
    };
    let mut selected_pieces = Vec::new();
    for piece in line_pieces.iter().skip(skip_index).step_by(2) {
        selected_pieces.push(piece.to_vec());
    }

    // Finally, modify the ends of each piece to make them end exactly on the area border
    for piece in selected_pieces.iter_mut() {
        // treat first point
        piece[0] = truncate_line_segment(&(piece[0], piece[1]), x_range, y_range);
        // treat last point
        let length = piece.len();
        piece[length-1] = truncate_line_segment(&(piece[length-1], piece[length-2]), x_range, y_range);
    }    

    selected_pieces
}

/// Take a vector of vectors of points, and plot them into an SVG file, returned as a String
pub fn plot_data_slice_to_svg(data_slice: &DataSlice, plot_settings: &PlotSettings, image_size: &(u32, u32)) -> (String, (PlotRange, PlotRange))
{
    let data = &data_slice.datalines;

    // Parameters
    let point_size = 2; // Point Size
    let colors = match plot_settings.use_color {
        true => vec![BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, YELLOW],
        false => vec![BLACK],
    };
    let n_colors = colors.len();
    let x_padding = 0.0;
    let y_padding = 0.02;

    // Figure out drawing area
    let (mut xmin, mut xmax) = match plot_settings.plot_range_x {
        PlotRange::Fixed(x_range) => x_range,
        PlotRange::Auto => {
            let mut xmaxs = Vec::new();
            let mut xmins = Vec::new();
            for series in data {
                if !series.is_empty() {
                    xmaxs.push(series.iter().max_by(|(x1,_),(x2,_)| x1.partial_cmp(x2).unwrap()).unwrap().0);
                    xmins.push(series.iter().min_by(|(x1,_),(x2,_)| x1.partial_cmp(x2).unwrap()).unwrap().0);
                }
            }

            let xmin = if !xmins.is_empty() {
                *xmins.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap()
            } else {
                0.0
            };
            let xmax = if !xmaxs.is_empty() {
                *xmaxs.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap()
            } else {
                0.0
            };

            (xmin, xmax)
        }
    };


    let (mut ymin, mut ymax) = match plot_settings.plot_range_y {
        PlotRange::Fixed(y_range) => y_range,
        PlotRange::Auto => {
            // Filter the points to only consider those in the specified x-range
            let mut points: Vec<Point> = Vec::new();
            for series in data {
                points.extend(series.iter().filter(|(x, _y)| x >= &xmin && x <= &xmax ));
            }

            let ymin = if !points.is_empty() {
                points.iter().min_by(|(_,y1),(_,y2)| y1.partial_cmp(y2).unwrap()).unwrap().1
            } else {
                0.0
            };
            let ymax = if !points.is_empty() {
                points.iter().max_by(|(_,y1),(_,y2)| y1.partial_cmp(y2).unwrap()).unwrap().1
            } else {
                0.0
            };

            (ymin, ymax)
        }
    };

    if plot_settings.use_logscale_x {
        if xmin <= 0.0 {
            xmin = 1.0e-10;
        }
        let dx = xmax.log(10.0) - xmin.log(10.0);
        xmin = 10.0_f64.powf(xmin.log(10.0) - x_padding * dx);
        xmax = 10.0_f64.powf(xmax.log(10.0) + x_padding * dx);
    } else {
        let dx = xmax - xmin;
        xmin -= x_padding * dx;
        xmax += x_padding * dx;
    }

    if xmax == xmin {
        xmin -= 0.05;
        xmax += 0.05;
    }
    if plot_settings.use_logscale_y {
        if ymin <= 0.0 {
            ymin = 1.0e-10;
        }
        let dy = ymax.log(10.0) - ymin.log(10.0);
        ymin = 10.0_f64.powf(ymin.log(10.0) - y_padding * dy);
        ymax = 10.0_f64.powf(ymax.log(10.0) + y_padding * dy);
    } else {
        let dy = ymax - ymin;
        ymin -= y_padding * dy;
        ymax += y_padding * dy;
    }

    if ymax == ymin {
        ymin -= 0.05;
        ymax += 0.05;
    }

    let plotted_ranges = (PlotRange::Fixed((xmin, xmax)), PlotRange::Fixed((ymin, ymax)));

    // Filter data to work around plotters bug
    // For point series: Remove any points outside the plotting area
    // For line series: Move last and first points along connecting lines
    //                  to coincide with plotting area border    

    let mut point_data: Vec<Vec<Point>> = Vec::new();
    if plot_settings.draw_points {
        for line in data {
            let filtered_line: Vec<Point> = line.iter().filter(|(x,y)| {
                *x >= xmin && *x <= xmax && *y >= ymin && *y <= ymax
            }).copied().collect();
            point_data.push(filtered_line);
        }
    }

    let mut line_data: Vec<Vec<Vec<Point>>> = Vec::new();
    if plot_settings.draw_lines {
        for line in data.iter() {
            let filtered_line = truncate_line(&line, &(xmin, xmax), &(ymin, ymax));
            line_data.push(filtered_line);
        }
    }

    let x_range = xmin..xmax;
    let y_range = ymin..ymax;
    let logscale_settings = (plot_settings.use_logscale_x, plot_settings.use_logscale_y);

    let mut svg_string = String::new();    
    {
        let root = SVGBackend::with_string(&mut svg_string, *image_size).into_drawing_area();
        root.fill(&WHITE).expect("Failed to fill canvas");

        /* Ugly code duplication, but I can't figure out how to work around not being able to make
           trait objects out of plotters::coord::AsRangedCoord... */
        match logscale_settings {
            (false, false) => {
                let mut chart = ChartBuilder::on(&root)
                                .x_label_area_size(20)
                                .y_label_area_size(50)
                                .build_ranged(x_range, y_range)
                                .expect("Failed to build chart");

                chart.configure_mesh().draw().expect("Failed to draw mesh");

                // plot each point vector seperately
                // line:
                if plot_settings.draw_lines {
                    for (i, line_segments) in line_data.iter().enumerate() {
                        let color = &colors[i%n_colors];
                        for segment in line_segments {
                            chart.draw_series(LineSeries::new(segment.clone(), color))
                                .expect("Failed to draw line");
                        }
                    }
                }
                // points:
                if plot_settings.draw_points {  
                    for (i, points) in point_data.iter().enumerate() {
                        let color = &colors[i%n_colors];
                        chart.draw_series(PointSeries::of_element(
                            points.clone(),
                            point_size,
                            color,
                            &|coord, size, style| {
                                EmptyElement::at(coord)
                                    + Circle::new((0,0), size, style.filled())
                            }
                            )).expect("Failed to draw points");
                    }
                }
            },
            (false, true) => {
                let mut chart = ChartBuilder::on(&root)
                                .x_label_area_size(20)
                                .y_label_area_size(50)
                                .build_ranged(x_range, LogRange(y_range))
                                .expect("Failed to build chart");

                chart.configure_mesh().draw().expect("Failed to draw mesh");

                // plot each point vector seperately
                // line:
                if plot_settings.draw_lines {
                    for (i, line_segments) in line_data.iter().enumerate() {
                        let color = &colors[i%n_colors];
                        for segment in line_segments {
                            chart.draw_series(LineSeries::new(segment.clone(), color))
                                .expect("Failed to draw line");
                        }
                    }
                }
                // points:
                if plot_settings.draw_points {  
                    for (i, points) in point_data.iter().enumerate() {
                        let color = &colors[i%n_colors];
                        chart.draw_series(PointSeries::of_element(
                            points.clone(),
                            point_size,
                            color,
                            &|coord, size, style| {
                                EmptyElement::at(coord)
                                    + Circle::new((0,0), size, style.filled())
                            }
                            )).expect("Failed to draw points");
                    }
                }                
            },
            (true, false) => {
                let mut chart = ChartBuilder::on(&root)
                                .x_label_area_size(20)
                                .y_label_area_size(50)
                                .build_ranged(LogRange(x_range), y_range)
                                .expect("Failed to build chart");

                chart.configure_mesh().draw().expect("Failed to draw mesh");

                // plot each point vector seperately
                // line:
                if plot_settings.draw_lines {
                    for (i, line_segments) in line_data.iter().enumerate() {
                        let color = &colors[i%n_colors];
                        for segment in line_segments {
                            chart.draw_series(LineSeries::new(segment.clone(), color))
                                .expect("Failed to draw line");
                        }
                    }
                }
                // points:
                if plot_settings.draw_points {  
                    for (i, points) in point_data.iter().enumerate() {
                        let color = &colors[i%n_colors];
                        chart.draw_series(PointSeries::of_element(
                            points.clone(),
                            point_size,
                            color,
                            &|coord, size, style| {
                                EmptyElement::at(coord)
                                    + Circle::new((0,0), size, style.filled())
                            }
                            )).expect("Failed to draw points");
                    }
                }
            },
            (true, true) => {
                let mut chart = ChartBuilder::on(&root)
                                .x_label_area_size(20)
                                .y_label_area_size(50)
                                .build_ranged(LogRange(x_range), LogRange(y_range))
                                .expect("Failed to build chart");

                chart.configure_mesh().draw().expect("Failed to draw mesh");

                // plot each point vector seperately
                // line:
                if plot_settings.draw_lines {
                    for (i, line_segments) in line_data.iter().enumerate() {
                        let color = &colors[i%n_colors];
                        for segment in line_segments {
                            chart.draw_series(LineSeries::new(segment.clone(), color))
                                .expect("Failed to draw line");
                        }
                    }
                }
                // points:
                if plot_settings.draw_points {  
                    for (i, points) in point_data.iter().enumerate() {
                        let color = &colors[i%n_colors];
                        chart.draw_series(PointSeries::of_element(
                            points.clone(),
                            point_size,
                            color,
                            &|coord, size, style| {
                                EmptyElement::at(coord)
                                    + Circle::new((0,0), size, style.filled())
                            }
                            )).expect("Failed to draw points");
                    }
                }
            },
        }

    }

    // Return "file" and actual range
    (svg_string, plotted_ranges)
}
