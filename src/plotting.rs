use crate::data::{DataSlice, Point};

use plotters::prelude::*;

type Range = (f64, f64);

#[derive(Clone)]
pub enum PlotRange {
    Auto,
    Fixed(Range, Range),
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
    return (x2, y2);
}

/// Takes a line of data points and returns a vector of lines that lie within the specified area
fn truncate_line(line: &Vec<Point>, x_range: &Range, y_range: &Range) -> Vec<Vec<Point>> {
    if line.len() == 0 {
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
            return vec![line.clone()];
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
pub fn plot_data_slice_to_svg(data_slice: &DataSlice, plot_range: &PlotRange, image_size: &(u32, u32)) -> String
{
    let (_filenames, data) = data_slice;

    // Parameters
    let point_size = 2; // Point Size
    let colors = vec![BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, YELLOW];
    let n_colors = colors.len();
    let padding = 0.05;

    // Figure out drawing area
    let ((xmin, xmax), (ymin, ymax)) = match *plot_range {
        PlotRange::Fixed(x_range, y_range) => (x_range, y_range),
        PlotRange::Auto => {
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
            ((xmin, xmax), (ymin, ymax))
        }
    };

    // Filter data to work around plotters bug
    // For point series: Remove any points outside the plotting area
    // For line series: Move last and first points along connecting lines
    //                  to coincide with plotting area border    

    let mut point_data: Vec<Vec<Point>> = Vec::new();
    for line in data {
        let filtered_line: Vec<Point> = line.iter().filter(|(x,y)| {
            *x >= xmin && *x <= xmax && *y >= ymin && *y <= ymax
        }).map(|p| *p).collect();
        point_data.push(filtered_line);
    }
    let mut line_data: Vec<Vec<Vec<Point>>> = Vec::new();
    for line in data.iter() {
        let filtered_line = truncate_line(&line, &(xmin, xmax), &(ymin, ymax));
        // let filtered_line = vec![line.clone()];
        line_data.push(filtered_line);
    }

    // Prepare empty plot
    let mut svg_string = String::new();
    let (width, height) = *image_size;
    {
        let root = SVGBackend::with_string(&mut svg_string, (width, height)).into_drawing_area();
        root.fill(&WHITE).expect("Failed to fill canvas");
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(20)
            .y_label_area_size(50)
            .build_ranged(xmin..xmax, ymin..ymax)
            .expect("Failed to build chart");

        chart.configure_mesh().draw().expect("Failed to draw mesh");

        // plot each point vector seperately
        // line:
        for (i, line_segments) in line_data.iter().enumerate() {
            let color = &colors[i%n_colors];
            for segment in line_segments {
                chart.draw_series(LineSeries::new(segment.clone(), color))
                    .expect("Failed to draw line");
            }
        }
        // points:
        for (i, points) in point_data.iter().enumerate() {
            let color = &colors[i%n_colors];
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
