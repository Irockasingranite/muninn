mod plotting;
mod ui;
mod data;
mod state;

use glib::{clone};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application};
use ui::{build_ui};

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let application = Application::new(
        Some("org.muninn"),
        gio::ApplicationFlags::empty(),
    ).expect("Failed to initialize GTK application");
    

    use data::{Data};
    use state::{State};
    let data = Data::from_files(vec![String::from("test.dat")]);
    let state = State::from_data(data);

    let state_cell = Rc::new(RefCell::new(state));
    application.connect_activate(clone!(@weak state_cell => move |app| {
        build_ui(app, state_cell);
    }));

    let state_clone = state_cell.clone();
    timeout_add(10, move || {
        let is_playing = state_clone.borrow().is_playing;
        if is_playing {
            state_clone.borrow_mut().advance_animation();
        }
        return Continue(true);
    });

    application.run(&[]);
}