mod plotting;
mod ui;
mod data;
mod state;

use glib::{clone};
use gtk::prelude::*;
use gtk::{Application};
use ui::{build_ui};

use std::env;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let application = Application::new(
        Some("org.muninn"),
        gio::ApplicationFlags::empty(),
    );
    
    let args: Vec<String> = env::args().collect();

    use data::{Data};
    use state::{State};

    let data = Data::from_files(args);
    let state = match data {
        Some(d) => State::from_data(d),
        None => State::new(),
    };

    let state_cell = Rc::new(RefCell::new(state));
    application.connect_activate(clone!(@weak state_cell => move |app| {
        build_ui(app, state_cell);
    }));

    application.run();
}