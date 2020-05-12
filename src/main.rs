mod plotting;
mod ui;
mod data;

use gio::prelude::*;
use gtk::{Application};
use ui::{build_ui};

fn main() {
    use data::{Data};
    let data = Data::from_files(vec![String::from("test.dat")]);

    let time = 3.0;

    let data_slice = data.at_time(time);

    let application = Application::new(
        Some("org.muninn"),
        gio::ApplicationFlags::empty(),
    ).expect("Failed to initialize GTK application");

    application.connect_activate(move |app| {
        build_ui(app, data_slice.clone());
    });

    application.run(&[]);
}