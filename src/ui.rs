use gtk::{Application, ApplicationWindow, Builder, Button, Box, Image};
use gtk::prelude::*;

use crate::plotting::{test_plot};

pub fn build_ui(application: &Application) {
    let glade_src = include_str!("../layout.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("appWindow")
        .expect("Failed to get appWindow");
    window.set_application(Some(application));

    let image: Image = builder.get_object("plotImage")
        .expect("Failed to get plotImage");

    // let pixbuf = image.get_pixbuf().unwrap();

    let button: Button = builder.get_object("button")
        .expect("Failed to get button");

    button.connect_clicked(move |_| {
        use gdk_pixbuf::{PixbufLoader, PixbufLoaderExt};
        let loader = PixbufLoader::new();
        let svg_string = test_plot();
        loader.write(svg_string.as_bytes())
            .expect("Failed to write svg to loader");
        loader.close().expect("Failed to close loader");
        let buf = loader.get_pixbuf();
        image.set_from_pixbuf(buf.as_ref());
    });

    window.show_all();
}