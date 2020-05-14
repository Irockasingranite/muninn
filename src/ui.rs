#![allow(unused_variables)] // remove this later

use gtk::{Application, ApplicationWindow, Builder, Button, Entry, Image};
use gtk::{TextBufferExt};
use gtk::prelude::*;
use std::rc::{Rc};
use std::cell::{RefCell};
use glib::{clone};

use crate::state::{State};

use gdk_pixbuf::{Pixbuf, PixbufLoader, PixbufLoaderExt};
fn pixbuf_from_string(s: &str) -> Pixbuf {
    let loader = PixbufLoader::new();
    loader.write(s.as_bytes()).expect("Failed to load string into Pixbuf");
    loader.close()
        .expect("Failed to close PixBufLoader");
    loader.get_pixbuf().unwrap()
}

pub fn build_ui(application: &Application, state_cell: Rc<RefCell<State>>) {
    let glade_src = include_str!("../layout.glade");
    let builder = Builder::new_from_string(glade_src);

    // Main Window setup
    let window: ApplicationWindow = builder.get_object("main_app_window")
        .expect("Failed to get appWindow");
    window.set_application(Some(application));

    // Plot Image setup
    let image: Image = builder.get_object("plot_image")
        .expect("Failed to get plot_image");

    // Play/Pause button setup
    let play_pause_button: Button = builder.get_object("play_pause_button")
        .expect("Failed to get play_pause_button");

    let play_icon: Image = builder.get_object("play_icon")
        .expect("Failed to get play_icon");
    let pause_icon: Image = builder.get_object("pause_icon")
        .expect("Failed to get pause_icon");

    play_pause_button.connect_clicked(clone!(@weak state_cell,
                                             @strong play_pause_button,
                                             @strong play_icon,
                                             @strong pause_icon => move |_| {
        let ref mut is_playing = state_cell.borrow_mut().is_playing;
        match *is_playing {
            true => {
                *is_playing = false;
                play_pause_button.set_image(Some(&play_icon));
            },
            false => {
                *is_playing = true;
                play_pause_button.set_image(Some(&pause_icon));
            },
        }
        println!("is_playing: {}", is_playing);
    }));

    // Time entry setup
    let current_time_entry: Entry = builder.get_object("current_time_entry")
        .expect("Failed to get current_time_entry");
    let current_time_entry_buffer = current_time_entry.get_buffer();
    current_time_entry.connect_activate(clone!(@weak state_cell,
                                               @strong current_time_entry_buffer as buf
                                               => move |_| {
        let text = buf.get_text();
        match text.parse::<f64>() {
            Ok(t) => {
                state_cell.borrow_mut().jump_to_time(t);
            },
            Err(_) => (),
        }
    }));

    window.show_all();
}