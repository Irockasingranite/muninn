use gtk::{Application, ApplicationWindow, Builder, Button, Entry, EventBox, FileChooserDialog, Image, SpinButton, ToggleButton, Viewport};
use gtk::ResponseType;
use gtk::prelude::*;
use std::rc::{Rc};
use std::cell::{RefCell};
use glib::{clone};

use crate::state::{State};
use crate::data::{Data};
use crate::plotting::{PlotRange};

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
    let plot_image_event_box: EventBox = builder.get_object("plot_image_event_box")
        .expect("Failed to get plot_image_event_box");
    plot_image_event_box.add_events(gdk::EventMask::SCROLL_MASK);
    plot_image_event_box.connect_scroll_event(move |_, event| {
        let (_x, _y) = event.get_position();
        // println!("scroll at ({}, {})!", x, y);
        Inhibit(false)
    });
    let plot_image_viewport: Viewport = builder.get_object("plot_image_viewport")
        .expect("Failed to get plot_image_viewport");
    plot_image_viewport.connect_size_allocate(clone!(@weak state_cell => move |_, allocation| {
        let width = allocation.width as u32;
        let height = allocation.height as u32;
        let (current_width, current_height) = state_cell.borrow().plot_image_size;
        if (width, height) != (current_width, current_height) {
            state_cell.borrow_mut().plot_image_size = (width, height);
            state_cell.borrow_mut().update_needed = true;
        }
    }));

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
        if state_cell.borrow().loaded_data.is_none() {
            return;
        }
        let is_playing = &mut state_cell.borrow_mut().is_playing;
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
        if let Ok(t) = text.parse::<f64>() {
            let new_time = state_cell.borrow_mut().jump_to_time(t);
            if let Some(t) = new_time {
                buf.set_text(format!("{:.3}", t).as_str());
            }
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

    // Update interval spinbutton setup
    let update_interval_spinbutton: SpinButton = builder.get_object("update_interval_spinbutton")
        .expect("Failed to get update_interval_spinbutton");
    let update_interval_spinbutton_adjustment = update_interval_spinbutton.get_adjustment();
    update_interval_spinbutton_adjustment.connect_value_changed(clone!(@strong update_interval_spinbutton,
                                                                       @weak state_cell => move |_| {
        let value = update_interval_spinbutton.get_value();
        state_cell.borrow_mut().update_interval = value as i32;
    }));

    // Timestep interval spinbutton setup
    let timestep_interval_spinbutton: SpinButton = builder.get_object("timestep_interval_spinbutton")
        .expect("Failed to get timestep_interval_spinbutton");
    let timestep_interval_spinbutton_adjustment = timestep_interval_spinbutton.get_adjustment();
    timestep_interval_spinbutton_adjustment.connect_value_changed(clone!(@strong timestep_interval_spinbutton,
                                                         @weak state_cell => move |_| {
        let value = timestep_interval_spinbutton.get_value();
        state_cell.borrow_mut().timestep_interval = value as usize;
    }));

    // Autoscale toggle setup
    let autoscale_toggle: ToggleButton = builder.get_object("autoscale_toggle")
        .expect("Failed to get autoscale_toggle");
    autoscale_toggle.connect_toggled(clone!(@strong autoscale_toggle,
                                            @weak state_cell => move |_| {
        let checked = autoscale_toggle.get_active();
        if checked {
            state_cell.borrow_mut().plot_settings.plot_range = PlotRange::Auto;
            state_cell.borrow_mut().update_needed = true;
        } else {
            let mut state = state_cell.borrow_mut();
            state.plot_settings.plot_range = state.plot_range_actual;
        }
    }));
    autoscale_toggle.set_active(true);

    // Logscale x toggle setup
    let logscale_x_toggle: ToggleButton = builder.get_object("logscale_x_toggle")
        .expect("Failed to get logscale_x_toggle");
    logscale_x_toggle.connect_toggled(clone!(@strong logscale_x_toggle,
                                             @weak state_cell => move |_| {
        state_cell.borrow_mut().plot_settings.use_logscale_x = logscale_x_toggle.get_active();
        state_cell.borrow_mut().update_needed = true;
    }));
    logscale_x_toggle.set_active(false);

    // Logscale y toggle setup
    let logscale_y_toggle: ToggleButton = builder.get_object("logscale_y_toggle")
        .expect("Failed to get logscale_x_toggle");
    logscale_y_toggle.connect_toggled(clone!(@strong logscale_y_toggle,
                                             @weak state_cell => move |_| {
        state_cell.borrow_mut().plot_settings.use_logscale_y = logscale_y_toggle.get_active();
        state_cell.borrow_mut().update_needed = true;
    }));
    logscale_y_toggle.set_active(false);    

    // Line toggle setup
    let line_toggle: ToggleButton = builder.get_object("line_toggle")
        .expect("Failed to get line_toggle");
    line_toggle.connect_toggled(clone!(@strong line_toggle,
                                       @weak state_cell => move |_| {
        state_cell.borrow_mut().plot_settings.draw_lines = line_toggle.get_active();
        state_cell.borrow_mut().update_needed = true;
    }));
    line_toggle.set_active(true);

    // Point toggle setup
    let point_toggle: ToggleButton = builder.get_object("point_toggle")
        .expect("Failed to get point_toggle");
    point_toggle.connect_toggled(clone!(@strong point_toggle,
                                        @weak state_cell => move |_| {
        state_cell.borrow_mut().plot_settings.draw_points = point_toggle.get_active();
        state_cell.borrow_mut().update_needed = true;
    }));
    point_toggle.set_active(true);

    let color_toggle: ToggleButton = builder.get_object("color_toggle")
        .expect("Failed to get color_toggle");
    color_toggle.connect_toggled(clone!(@strong color_toggle,
                                        @weak state_cell => move |_| {
        state_cell.borrow_mut().plot_settings.use_color = color_toggle.get_active();
        state_cell.borrow_mut().update_needed = true;
    }));
    color_toggle.set_active(true);

    // x_min entry setup
    let x_min_entry: Entry = builder.get_object("x_min_entry")
        .expect("Failed to get x_min_entry");
    let x_min_entry_buffer = x_min_entry.get_buffer();
    x_min_entry.connect_activate(clone!(@strong x_min_entry_buffer as buf,
                                        @strong autoscale_toggle,
                                        @weak state_cell => move |_| {
        let text = buf.get_text();
        if let Ok(x_min_new) = text.parse::<f64>() {
            let ((_x_min, x_max), (y_min, y_max)) = match state_cell.borrow().plot_range_actual {
                PlotRange::Fixed(x_range,y_range) => (x_range, y_range),
                PlotRange::Auto => ((0.0, 1.0), (0.0, 1.0)),
            };
            autoscale_toggle.set_active(false);
            state_cell.borrow_mut().plot_settings.plot_range = PlotRange::Fixed((x_min_new, x_max), (y_min, y_max));
            state_cell.borrow_mut().update_needed = true;
        }
    }));

    // x_max entry setup
    let x_max_entry: Entry = builder.get_object("x_max_entry")
        .expect("Failed to get x_max_entry");
    let x_max_entry_buffer = x_max_entry.get_buffer();
    x_max_entry.connect_activate(clone!(@strong x_max_entry_buffer as buf,
                                        @strong autoscale_toggle,
                                        @weak state_cell => move |_| {
        let text = buf.get_text();
        if let Ok(x_max_new) = text.parse::<f64>() {
            let ((x_min, _x_max), (y_min, y_max)) = match state_cell.borrow().plot_range_actual {
                PlotRange::Fixed(x_range,y_range) => (x_range, y_range),
                PlotRange::Auto => ((0.0, 1.0), (0.0, 1.0)),
            };
            autoscale_toggle.set_active(false);
            state_cell.borrow_mut().plot_settings.plot_range = PlotRange::Fixed((x_min, x_max_new), (y_min, y_max));
            state_cell.borrow_mut().update_needed = true;
        }
    }));

    // y_min entry setup
    let y_min_entry: Entry = builder.get_object("y_min_entry")
        .expect("Failed to get y_min_entry");
    let y_min_entry_buffer = y_min_entry.get_buffer();
    y_min_entry.connect_activate(clone!(@strong y_min_entry_buffer as buf,
                                        @strong autoscale_toggle,
                                        @weak state_cell => move |_| {
        let text = buf.get_text();
        if let Ok(y_min_new) = text.parse::<f64>() {
            let ((x_min, x_max), (_y_min, y_max)) = match state_cell.borrow().plot_range_actual {
                PlotRange::Fixed(x_range,y_range) => (x_range, y_range),
                PlotRange::Auto => ((0.0, 1.0), (0.0, 1.0)),
            };
            autoscale_toggle.set_active(false);
            state_cell.borrow_mut().plot_settings.plot_range = PlotRange::Fixed((x_min, x_max), (y_min_new, y_max));
            state_cell.borrow_mut().update_needed = true;
        }
    }));

    // y_max entry setup
    let y_max_entry: Entry = builder.get_object("y_max_entry")
        .expect("Failed to get y_max_entry");
    let y_max_entry_buffer = y_max_entry.get_buffer();
    y_max_entry.connect_activate(clone!(@strong y_max_entry_buffer as buf,
                                        @strong autoscale_toggle,
                                        @weak state_cell => move |_| {
        let text = buf.get_text();
        if let Ok(y_max_new) = text.parse::<f64>() {
            let ((x_min, x_max), (y_min, _y_max)) = match state_cell.borrow().plot_range_actual {
                PlotRange::Fixed(x_range,y_range) => (x_range, y_range),
                PlotRange::Auto => ((0.0, 1.0), (0.0, 1.0)),
            };
            autoscale_toggle.set_active(false);
            state_cell.borrow_mut().plot_settings.plot_range = PlotRange::Fixed((x_min, x_max), (y_min, y_max_new));
            state_cell.borrow_mut().update_needed = true;
        }
    }));

    // file_chooser_dialog setup
    let file_chooser_dialog: FileChooserDialog = builder.get_object("file_chooser_dialog")
        .expect("Failed to get file_chooser_dialog");
    file_chooser_dialog.add_button("Cancel", ResponseType::Cancel);
    file_chooser_dialog.add_button("Open", ResponseType::Accept);
    file_chooser_dialog.connect_response(clone!(@weak state_cell => move |d,r| {
        if let ResponseType::Accept = r {
            let filenames = d.get_filenames();
            let filenames: Vec<String> = filenames.iter().map(|pb| pb.as_path().display().to_string()).collect();
            if let Some(data) = Data::from_files(filenames) {
                state_cell.borrow_mut().load_data(data);
            }
        }
    }));
    let load_button: Button = builder.get_object("load_button")
        .expect("Failed to get load_button");
    load_button.connect_clicked(move |_| {
        file_chooser_dialog.run();
        file_chooser_dialog.hide();
    });

    // Custom update routine (called every 10 ms)
    let state_clone = state_cell;
    let current_time_entry_buffer_clone = current_time_entry_buffer;
    let x_min_entry_buffer_clone = x_min_entry_buffer;
    let x_max_entry_buffer_clone = x_max_entry_buffer;
    let y_min_entry_buffer_clone = y_min_entry_buffer;
    let y_max_entry_buffer_clone = y_max_entry_buffer;
    let plot_image_clone = plot_image;
    timeout_add(10, move || {
        
        // Animation of state
        let is_playing = state_clone.borrow().is_playing;
        if is_playing {
            state_clone.borrow_mut().advance_animation();
            let time = state_clone.borrow().current_time;
            current_time_entry_buffer_clone.set_text(format!("{:.3}", time).as_str());
        }

        // Update plot if necessary
        let update_needed = state_clone.borrow().update_needed;
        if update_needed {
            let svg_string = state_clone.borrow_mut().update_plot();
            if let Some((s, r)) = svg_string {
                // Update the plot itself
                let buf = pixbuf_from_string(&s);
                plot_image_clone.set_from_pixbuf(Some(&buf));

                // Update range entries
                let (x_min, x_max, y_min, y_max) = match r {
                    PlotRange::Fixed((x_min, x_max), (y_min, y_max)) 
                        => (x_min, x_max, y_min, y_max),
                    PlotRange::Auto => (0.0, 1.0, 0.0, 1.0),
                };
                x_min_entry_buffer_clone.set_text(format!("{:.3}", x_min).as_str());
                x_max_entry_buffer_clone.set_text(format!("{:.3}", x_max).as_str());
                y_min_entry_buffer_clone.set_text(format!("{:.3}", y_min).as_str());
                y_max_entry_buffer_clone.set_text(format!("{:.3}", y_max).as_str());
            }

        }

        Continue(true)
    });

    window.show_all();
}