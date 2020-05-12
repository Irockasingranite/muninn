use crate::data::DataSlice;
use gtk::{Application, ApplicationWindow, Builder, Button, Image};
use gtk::prelude::*;

use crate::plotting::{test_plot};

use gdk_pixbuf::{Pixbuf, PixbufLoader, PixbufLoaderExt};
fn pixbuf_from_string(s: &str) -> Pixbuf {
    let loader = PixbufLoader::new();
    loader.write(s.as_bytes()).expect("Failed to load string into Pixbuf");
    loader.close()
        .expect("Failed to close PixBufLoader");
    loader.get_pixbuf().unwrap()
}

pub fn build_ui(application: &Application, data_slice: DataSlice) {
    let glade_src = include_str!("../layout.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("appWindow")
        .expect("Failed to get appWindow");
    window.set_application(Some(application));

    let image: Image = builder.get_object("plotImage")
        .expect("Failed to get plotImage");

    // let pixbuf = image.get_pixbuf().unwrap();

    let test_plot_button: Button = builder.get_object("test_plot_button")
        .expect("Failed to get test_plot_button");

    let image_clone = image.clone();
    test_plot_button.connect_clicked(move |_| {
        let svg_string = test_plot();
        let buf = pixbuf_from_string(&svg_string);
        image_clone.set_from_pixbuf(Some(&buf));
    });

    let t3_button: Button = builder.get_object("t3_button")
        .expect("Failed to get t3_button");

    let image_clone = image.clone();
    t3_button.connect_clicked(move |_| {
        use crate::plotting::{plot_data_slice_to_svg};
        let svg_string = plot_data_slice_to_svg(data_slice.clone());
        let buf = pixbuf_from_string(&svg_string);
        image_clone.set_from_pixbuf(Some(&buf));
    });

    window.show_all();
}