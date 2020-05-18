use gtk::{Application, ApplicationWindow, Builder, Button, Entry, Image};
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
    let plot_image: Image = builder.get_object("plot_image")
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
                let new_time = state_cell.borrow_mut().jump_to_time(t);
                if let Some(t) = new_time {
                    buf.set_text(format!("{:.3}", t).as_str());
                }
            },
            Err(_) => (),
        }
    }));
    let time = state_cell.borrow().current_time;
    current_time_entry_buffer.set_text(format!("{:.3}", time).as_str());

    // First button setup
    let first_button: Button = builder.get_object("first_button")
        .expect("Failed to get first_button");
    first_button.connect_clicked(clone!(@strong current_time_entry_buffer,
                                        @weak state_cell => move |_| {
        let time = state_cell.borrow_mut().go_to_first_step();
        if let Some(t) = time {
            current_time_entry_buffer.set_text(format!("{:.3}", t).as_str());
        }
    }));   

    // Previous button setup
    let previous_button: Button = builder.get_object("previous_button")
        .expect("Failed to get previous_button");
    previous_button.connect_clicked(clone!(@strong current_time_entry_buffer,
                                           @weak state_cell => move |_| {
        let time = state_cell.borrow_mut().go_to_previous_step();
        if let Some(t) = time {
            current_time_entry_buffer.set_text(format!("{:.3}", t).as_str());
        }
    }));

    // Next button setup
    let next_button: Button = builder.get_object("next_button")
        .expect("Failed to get next_button");
    next_button.connect_clicked(clone!(@strong current_time_entry_buffer,
                                       @weak state_cell => move |_| {
        let time = state_cell.borrow_mut().go_to_next_step();
        if let Some(t) = time {
            current_time_entry_buffer.set_text(format!("{:.3}", t).as_str());
        }
    }));

    // Last button setup
    let last_button: Button = builder.get_object("last_button")
        .expect("Failed to get last_button");
    last_button.connect_clicked(clone!(@strong current_time_entry_buffer,
                                       @weak state_cell => move |_| {
        let time = state_cell.borrow_mut().go_to_last_step();
        if let Some(t) = time {
            current_time_entry_buffer.set_text(format!("{:.3}",t).as_str());
        }
    }));

    // Custom update routine (called every 10 ms)
    let state_clone = state_cell.clone();
    let current_time_entry_buffer_clone = current_time_entry_buffer.clone();
    let plot_image_clone = plot_image.clone();
    timeout_add(10, move || {
        let time = state_clone.borrow().current_time;
        
        // Animation of state
        let is_playing = state_clone.borrow().is_playing;
        if is_playing {
            state_clone.borrow_mut().advance_animation();
            current_time_entry_buffer_clone.set_text(format!("{:.3}", time).as_str());
        }

        // Update plot if necessary
        let plotted_time = state_clone.borrow_mut().plotted_time;
        if plotted_time != Some(time) {
            let svg_string = state_clone.borrow_mut().update_plot();
            if let Some(s) = svg_string {
                let buf = pixbuf_from_string(&s);
                plot_image_clone.set_from_pixbuf(Some(&buf));
            }

        }

        return Continue(true);
    });

    window.show_all();
}