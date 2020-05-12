mod plotting;
use plotting::{plot_points_to_svg};

fn float_range(xmin: f64, xmax: f64, n: usize) -> Vec<f64> {
    let d = xmax - xmin;
    let dx = d / ((n-1) as f64);
    let xs = (0..n).map(|x| x as f64 * dx + xmin).collect();
    xs
}

fn test_plot() -> String {
    let xs = float_range(-1.0, 1.0, 100);
    let points: Vec<(f64,f64)> = xs.iter().map(|&x| (x, (x*10.0).sin())).collect();
    let points2: Vec<(f64,f64)> = xs.iter().map(|&x| (x, 2.0*x)).collect();
    let points3: Vec<(f64,f64)> = xs.iter().map(|&x| (x, -2.0*x)).collect();
    let data = vec![points, points2, points3];
    let svg_string = plot_points_to_svg(data);
    // println!("{:?}", file);

    // { // write string into file
    //     use std::path::Path;
    //     use std::fs::File;
    //     use std::io::prelude::*;

    //     let file_path = Path::new("1.svg");
    //     let mut file = File::create(&file_path).expect("Failed to create file");
    //     file.write_all(svg_string.as_bytes()).expect("Failed to write file");
    // }

    return svg_string
}

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Image};
use gdk_pixbuf::{PixbufLoader, PixbufLoaderExt};

fn main() {

    let application = Application::new(
        Some("org.muninn"),
        gio::ApplicationFlags::empty(),
    ).expect("Failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Muninn");
        window.set_default_size(350, 70);

        let svg_string = test_plot();
        let loader = PixbufLoader::new();
        loader.write(svg_string.as_bytes()).expect("Failed to load SVG string to Pixbuf");
        loader.close().expect("Failed to close PixbufLoader");


        let image = Image::new_from_pixbuf(loader.get_pixbuf().as_ref());
        window.add(&image);

        window.show_all();
    });

    application.run(&[]);
}