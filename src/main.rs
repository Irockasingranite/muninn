mod plotting;
mod ui;
mod data;
mod state;

use glib::{clone};
use gtk::prelude::*;
use gtk::{Application};
use ui::{build_ui};

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let application = Application::new(
        Some("org.muninn"),
        gio::ApplicationFlags::HANDLES_OPEN,
    );

    use state::{State};
    let state = State::new();

    let state_cell = Rc::new(RefCell::new(state));
    application.connect_activate(clone!(@weak state_cell => move |app| {
        build_ui(app, state_cell);
    }));

    use data::{Data};
    application.connect_open(clone!(@weak state_cell => move |app, files, _hint| {
        let mut filenames = Vec::new();
        for file in files {
            let filename = file.path()
                .expect("Error accessing file")
                .as_path().display().to_string();
            filenames.push(filename);
        }
        if let Some(data) = Data::from_files(filenames) {
            state_cell.borrow_mut().load_data(data);
        }

        app.activate();
    }));

    application.run();
}