mod plotting;
mod ui;

use gio::prelude::*;
use gtk::{Application};
use ui::{build_ui};

fn main() {

    let application = Application::new(
        Some("org.muninn"),
        gio::ApplicationFlags::empty(),
    ).expect("Failed to initialize GTK application");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&[]);
}