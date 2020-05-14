#![allow(unused_variables)] // remove this later

use gtk::{Application, ApplicationWindow, Builder, Button, Image};
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

    let window: ApplicationWindow = builder.get_object("main_app_window")
        .expect("Failed to get appWindow");
    window.set_application(Some(application));

    let image: Image = builder.get_object("plot_image")
        .expect("Failed to get plot_image");

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
    window.show_all();
}