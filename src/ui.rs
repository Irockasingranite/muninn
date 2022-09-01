use gdk::{ModifierType, ScrollDirection, EventScroll};
use glib::timeout_add_local;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use gtk::{Application, ApplicationWindow, Builder, Button, DrawingArea, Entry, EventBox, FileChooserDialog, Image, SpinButton, ToggleButton, Viewport};
use gtk::ResponseType;
use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use glib::clone;

use crate::state::{State, PlotStatus};
use crate::data::Data;
use crate::plotting::PlotRange;


use gdk_pixbuf::{Pixbuf, PixbufLoader};
fn pixbuf_from_string(s: &str) -> Pixbuf {
    let loader = PixbufLoader::new();
    loader.write(s.as_bytes()).expect("Failed to load string into Pixbuf");
    loader.close()
        .expect("Failed to close PixBufLoader");
    loader.pixbuf().unwrap()
}

pub fn build_ui(application: &Application, state_cell: Rc<RefCell<State>>) {
    let glade_src = include_str!("../layout.glade");
    let builder = Builder::from_string(glade_src);

    // Main Window setup
    let window: ApplicationWindow = builder.object("main_app_window")
        .expect("Failed to get appWindow");
    window.set_application(Some(application));
    window.set_default_width(1200);
    window.set_default_height(800);

    // Play/Pause button setup
    let _play_pause_button = setup_play_pause_button(builder.clone(), state_cell.clone());

    // Time entry setup
    let current_time_entry = setup_time_entry(builder.clone(), state_cell.clone());

    // First button setup
    let (_first_button, _previous_button, _next_button, _last_button) = setup_navigation_buttons(builder.clone(), state_cell.clone(), current_time_entry.buffer());

    // Update interval spinbutton setup
    let _update_interval_spinbutton = setup_interval_spinbutton(builder.clone(), state_cell.clone());

    // Timestep interval spinbutton setup
    let _timestep_interval_spinbutton = setup_timestep_interval_spinbutton(builder.clone(), state_cell.clone());

    // Autoscale toggle setup
    let (autoscale_x_toggle, autoscale_y_toggle) = setup_autoscale_toggles(builder.clone(), state_cell.clone());

    // Logscale toggle setup
    let (_logscale_x_toggle, _logscale_y_toggle) = setup_logscale_toggles(builder.clone(), state_cell.clone());

    // Line toggle setup
    let (_line_toggle, _point_toggle, _color_toggle) = setup_style_toggles(builder.clone(), state_cell.clone());

    // Plot range entry setup
    let (x_min_entry, x_max_entry, y_min_entry, y_max_entry) = setup_plot_range_entries(builder.clone(), state_cell.clone(), (autoscale_x_toggle, autoscale_y_toggle));

    // Plot Image setup
    let plot_area = setup_plot_area(builder.clone(), state_cell.clone());

    // Load button setup
    let _load_button = setup_load_button(builder.clone(), state_cell.clone(), window.clone());

    // Save button setup
    let _save_button = setup_save_button(builder.clone(), state_cell.clone(), window.clone());

    // export_gnuplot_button setup
    let _export_gnuplot_button = setup_export_gnuplot_button(builder.clone(), state_cell.clone(), window.clone());

    // Custom update routine (called every 10 ms)
    let status_mutex = Arc::new(Mutex::new(PlotStatus::Idle));
    let state_clone = state_cell;
    let time_entry_clone = current_time_entry;
    let x_min_entry_clone = x_min_entry;
    let x_max_entry_clone = x_max_entry;
    let y_min_entry_clone = y_min_entry;
    let y_max_entry_clone = y_max_entry;
    let plot_area_clone = plot_area.clone();
    timeout_add_local(Duration::from_millis(10), move || {

        // Depending on the plotting status, do different things
        let mut status_locked = status_mutex.lock().unwrap();
        match &*status_locked {
            PlotStatus::Working => {
                // If a plot is currently in the making, don't do anything
                return Continue(true);
            },
            PlotStatus::Finished(svg_string_option) => {
                // If a new plot has been finished, show it
                if let Some((s, r)) = svg_string_option {
                    // Update the plot itself
                    let buf = pixbuf_from_string(&s);
                    state_clone.borrow_mut().plot_image_pixbuf = Some(buf.clone());

                    // Update range entries
                    let (plot_range_x, plot_range_y) = r;
                    let (x_min, x_max) = plot_range_x.get();
                    let (y_min, y_max) = plot_range_y.get();

                    // Pick formatting depending on the actual value
                    // switch to scientific notation for small and large values
                    let x_min_entry_text = if (x_min.abs() < 1.0e-3  || x_min.abs() > 1.0e3) && x_min != 0.0 {
                        format!("{:.3e}", x_min)
                    } else {
                        format!("{:.3}", x_min)
                    };

                    let x_max_entry_text = if (x_max.abs() < 1.0e-3  || x_max.abs() > 1.0e3) && x_max != 0.0 {
                        format!("{:.3e}", x_max)
                    } else {
                        format!("{:.3}", x_max)
                    };

                    let y_min_entry_text = if (y_min.abs() < 1.0e-3  || y_min.abs() > 1.0e3) && y_min != 0.0 {
                        format!("{:.3e}", y_min)
                    } else {
                        format!("{:.3}", y_min)
                    };

                    let y_max_entry_text = if (y_max.abs() < 1.0e-3  || y_max.abs() > 1.0e3) && y_max != 0.0 {
                        format!("{:.3e}", y_max)
                    } else {
                        format!("{:.3}", y_max)
                    };

                    x_min_entry_clone.buffer().set_text(x_min_entry_text.as_str());
                    x_max_entry_clone.buffer().set_text(x_max_entry_text.as_str());
                    y_min_entry_clone.buffer().set_text(y_min_entry_text.as_str());
                    y_max_entry_clone.buffer().set_text(y_max_entry_text.as_str());
                    
                    // Also update state based on results of plot generation
                    state_clone.borrow_mut().plot_image_string = Some(s.clone());
                    state_clone.borrow_mut().plot_range_x_actual = *plot_range_x;
                    state_clone.borrow_mut().plot_range_y_actual = *plot_range_y;
                    state_clone.borrow_mut().update_needed = false;

                    // Redraw the plot itself
                    plot_area_clone.queue_draw();
                }

                // Reset plot status
                *status_locked = PlotStatus::Idle;
            },
            PlotStatus::Idle => {
                // If no plot is being worked on, advance the animation
                let is_playing = state_clone.borrow().is_playing;
                if is_playing {
                    state_clone.borrow_mut().advance_animation();
                    let time = state_clone.borrow().current_time;
                    time_entry_clone.buffer().set_text(format!("{:.3}", time).as_str());
                }

                // If a new plot is needed, spawn a task to create it
                let update_needed = state_clone.borrow().update_needed;
                if update_needed {
                    *status_locked = PlotStatus::Working;
                    let plotting = state_clone.borrow_mut().request_plot(Arc::clone(&status_mutex));
                    // If no data is loaded, no plotting task was created
                    if !plotting {
                        *status_locked = PlotStatus::Idle;
                    }
                }

            }
        }

        Continue(true)
    });

    window.show_all();
}

fn setup_plot_area(builder: Builder, state_cell: Rc<RefCell<State>>) -> gtk::DrawingArea {
    let plot_area: DrawingArea = builder.object("plot_area")
        .expect("Failed to get plot_area");

    plot_area.connect_draw(clone!(@strong state_cell => move |_, cr| {
        // Draw the plot itself
        if let Some(buf) = &state_cell.borrow().plot_image_pixbuf {
            cr.set_source_pixbuf(&buf, 0.0, 0.0);
        }
        cr.paint().expect("Failed to paint plot image pixbuf");

        // If the left mouse button is being held, draw a selection box
        if state_cell.borrow().mouse_state.left_button_held {
            // Using white as source and difference as the operator effectively inverts colors
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.set_operator(gdk::cairo::Operator::Difference);
            cr.set_line_width(1.0);

            let (x_start, y_start) = state_cell.borrow().mouse_state.drag_start;
            let (x_end, y_end) = state_cell.borrow().mouse_state.position;
            cr.rectangle(x_start, y_start, x_end - x_start, y_end - y_start);
            cr.stroke().expect("Failed to draw selection box");
        }

        gtk::Inhibit(false)
    }));

    // The eventbox lets us capture events like clicking and scrolling
    let plot_area_event_box: EventBox = builder.object("plot_area_event_box")
        .expect("Failed to get plot_area_event_box");
    
    // Handle mouse button press
    plot_area_event_box.add_events(gdk::EventMask::BUTTON_PRESS_MASK);
    plot_area_event_box.connect_button_press_event(clone!(@strong state_cell,
                                                          @strong plot_area => move |_, event| {
        if event.button() == 1 { // Left Mouse Button
            state_cell.borrow_mut().mouse_state.left_button_held = true;
            state_cell.borrow_mut().mouse_state.drag_start = event.position();
            plot_area.queue_draw();
        } else if event.button() == 2 || event.button() == 3 { // Middle or Right Mouse Button
            state_cell.borrow_mut().mouse_state.middle_right_button_held = true;
            state_cell.borrow_mut().mouse_state.drag_start = event.position();
        }

        Inhibit(false)
    }));

    // Handle mouse button release
    plot_area_event_box.add_events(gdk::EventMask::BUTTON_RELEASE_MASK);
    let x_min_entry: Entry = builder.object("x_min_entry")
        .expect("Failed to get x_min_entry");
    let x_max_entry: Entry = builder.object("x_max_entry")
        .expect("Failed to get x_max_entry");
    let y_min_entry: Entry = builder.object("y_min_entry")
        .expect("Failed to get y_min_entry");
    let y_max_entry: Entry = builder.object("y_max_entry")
        .expect("Failed to get y_max_entry");
    let autoscale_x_toggle: ToggleButton = builder.object("autoscale_x_toggle")
        .expect("Failed to get autoscale_x_toggle");
    let autoscale_y_toggle: ToggleButton = builder.object("autoscale_y_toggle")
        .expect("Failed to get autoscale_y_toggle");

    plot_area_event_box.connect_button_release_event(clone!(@strong state_cell,
                                                            @strong plot_area,
                                                            @strong x_min_entry,
                                                            @strong x_max_entry,
                                                            @strong y_min_entry,
                                                            @strong y_max_entry,
                                                            @strong autoscale_x_toggle,
                                                            @strong autoscale_y_toggle => move |_, event| {
        if event.button() == 1 { // Left Mouse Button
            state_cell.borrow_mut().mouse_state.left_button_held = false;
            plot_area.queue_draw();

            // Calculate new plot ranges
            let drag_start = state_cell.borrow().mouse_state.drag_start;
            let drag_end = event.position();

            if drag_start != drag_end {

                let (x_range_new, y_range_new) = plot_range_from_selection(drag_start, drag_end, state_cell.clone());

                // Disable autoscale toggles
                autoscale_x_toggle.set_active(false);
                autoscale_y_toggle.set_active(false);

                // Change plot settings 
                state_cell.borrow_mut().plot_settings.plot_range_x = x_range_new;
                state_cell.borrow_mut().plot_settings.plot_range_y = y_range_new;

                // Update range entries
                let (x_min, x_max) = x_range_new.get();
                let (y_min, y_max) = y_range_new.get();
                x_min_entry.set_text(&format!("{:.3}", x_min));
                x_max_entry.set_text(&format!("{:.3}", x_max));
                y_min_entry.set_text(&format!("{:.3}", y_min));
                y_max_entry.set_text(&format!("{:.3}", y_max));
                
                // Make sure a new plot is rendered
                state_cell.borrow_mut().update_needed = true;
            }
        } else if event.button() == 2 || event.button() == 3 { // Middle or Right Mouse Button
            state_cell.borrow_mut().mouse_state.middle_right_button_held = false;
        }

        Inhibit(false)
    }));

    // Handle mouse movement
    plot_area_event_box.add_events(gdk::EventMask::POINTER_MOTION_MASK);
    plot_area_event_box.connect_motion_notify_event(clone!(@strong state_cell,
                                                            @strong plot_area,
                                                            @strong x_min_entry,
                                                            @strong x_max_entry,
                                                            @strong y_min_entry,
                                                            @strong y_max_entry,
                                                            @strong autoscale_x_toggle,
                                                            @strong autoscale_y_toggle => move |_, event| {
        
        state_cell.borrow_mut().mouse_state.position = event.position();
        
        // Update selection rectangle
        if state_cell.borrow().mouse_state.left_button_held {
            plot_area.queue_draw();
        }

        // Pan the plot
        if state_cell.borrow().mouse_state.middle_right_button_held {
            let (xi, yi) = event.position();
            let (xp, yp) = plot_coords_from_image_coords(xi, yi, state_cell.clone());
            let (xi_last, yi_last) = state_cell.borrow().mouse_state.last_position;
            let (xp_last, yp_last) = plot_coords_from_image_coords(xi_last, yi_last, state_cell.clone());

            let delta_x = xp - xp_last;
            let delta_y = yp - yp_last;

            let (x_min, x_max) = state_cell.borrow().plot_range_x_actual.get();
            let (y_min, y_max) = state_cell.borrow().plot_range_y_actual.get();

            let x_min_new = x_min - delta_x;
            let x_max_new = x_max - delta_x;
            let y_min_new = y_min - delta_y;
            let y_max_new = y_max - delta_y;

            autoscale_x_toggle.set_active(false);
            autoscale_y_toggle.set_active(false);

            state_cell.borrow_mut().plot_settings.plot_range_x = PlotRange::Fixed((x_min_new, x_max_new));
            state_cell.borrow_mut().plot_settings.plot_range_y = PlotRange::Fixed((y_min_new, y_max_new));

            x_min_entry.set_text(&format!("{:.3}", x_min_new));
            x_max_entry.set_text(&format!("{:.3}", x_max_new));
            y_min_entry.set_text(&format!("{:.3}", y_min_new));
            y_max_entry.set_text(&format!("{:.3}", y_max_new));

            state_cell.borrow_mut().update_needed = true;
        }

        state_cell.borrow_mut().mouse_state.last_position = event.position();

        Inhibit(false)
    }));

    // Handle mouse wheel scrolling
    plot_area_event_box.add_events(gdk::EventMask::SCROLL_MASK);
    plot_area_event_box.connect_scroll_event(clone!(@strong state_cell,
                                                            @strong plot_area,
                                                            @strong x_min_entry,
                                                            @strong x_max_entry,
                                                            @strong y_min_entry,
                                                            @strong y_max_entry,
                                                            @strong autoscale_x_toggle,
                                                            @strong autoscale_y_toggle => move |_, event| {
        let scroll_factor = 0.05;
        let (x_range, y_range) = plot_range_after_scroll(state_cell.clone(), event, scroll_factor);

        if x_range != state_cell.borrow().plot_settings.plot_range_x {
            autoscale_x_toggle.set_active(false);
            state_cell.borrow_mut().plot_settings.plot_range_x = x_range;
        }

        if y_range != state_cell.borrow().plot_settings.plot_range_y {
            autoscale_y_toggle.set_active(false);
            state_cell.borrow_mut().plot_settings.plot_range_y = y_range;
        }

        state_cell.borrow_mut().update_needed = true;

        Inhibit(false)
    }));

    // The viewport is responsible for sizing the area
    let plot_area_viewport: Viewport = builder.object("plot_area_viewport")
        .expect("Failed to get plot_area_viewport");
    plot_area_viewport.connect_size_allocate(clone!(@weak state_cell => move |_, allocation| {
        let width = allocation.width() as u32;
        let height = allocation.height() as u32;
        let (current_width, current_height) = state_cell.borrow().plot_area_size;
        if (width, height) != (current_width, current_height) {
            state_cell.borrow_mut().plot_area_size = (width, height);
            state_cell.borrow_mut().update_needed = true;
        }
    }));

    plot_area
}

fn setup_play_pause_button(builder: Builder, state_cell: Rc<RefCell<State>>) -> Button {
    let play_pause_button: Button = builder.object("play_pause_button")
        .expect("Failed to get play_pause_button");

    let play_icon: Image = builder.object("play_icon")
        .expect("Failed to get play_icon");
    let pause_icon: Image = builder.object("pause_icon")
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

    play_pause_button
}

fn setup_time_entry(builder: Builder, state_cell: Rc<RefCell<State>>) -> gtk::Entry {
    let current_time_entry: Entry = builder.object("current_time_entry")
        .expect("Failed to get current_time_entry");
    let entry_buffer = current_time_entry.buffer();
    current_time_entry.connect_activate(clone!(@weak state_cell,
                                               @strong entry_buffer as buf
                                               => move |_| {
        let text = buf.text();
        if let Ok(t) = text.parse::<f64>() {
            let new_time = state_cell.borrow_mut().jump_to_time(t);
            if let Some(t) = new_time {
                buf.set_text(format!("{:.3}", t).as_str());
            }
        }
    }));
    let time = state_cell.borrow().current_time;
    entry_buffer.set_text(format!("{:.3}", time).as_str());

    current_time_entry
}

fn setup_navigation_buttons(builder: Builder, state_cell: Rc<RefCell<State>>, time_buffer: gtk::EntryBuffer) -> (Button, Button, Button, Button) {
    let first_button: Button = builder.object("first_button")
        .expect("Failed to get first_button");
    first_button.connect_clicked(clone!(@strong time_buffer,
                                        @weak state_cell => move |_| {
        let time = state_cell.borrow_mut().go_to_first_step();
        if let Some(t) = time {
            time_buffer.set_text(format!("{:.3}", t).as_str());
        }
    }));   

    // Previous button setup
    let previous_button: Button = builder.object("previous_button")
        .expect("Failed to get previous_button");
    previous_button.connect_clicked(clone!(@strong time_buffer,
                                           @weak state_cell => move |_| {
        let time = state_cell.borrow_mut().go_to_previous_step();
        if let Some(t) = time {
            time_buffer.set_text(format!("{:.3}", t).as_str());
        }
    }));

    // Next button setup
    let next_button: Button = builder.object("next_button")
        .expect("Failed to get next_button");
    next_button.connect_clicked(clone!(@strong time_buffer,
                                       @weak state_cell => move |_| {
        let time = state_cell.borrow_mut().go_to_next_step();
        if let Some(t) = time {
            time_buffer.set_text(format!("{:.3}", t).as_str());
        }
    }));

    // Last button setup
    let last_button: Button = builder.object("last_button")
        .expect("Failed to get last_button");
    last_button.connect_clicked(clone!(@strong time_buffer,
                                       @weak state_cell => move |_| {
        let time = state_cell.borrow_mut().go_to_last_step();
        if let Some(t) = time {
            time_buffer.set_text(format!("{:.3}",t).as_str());
        }
    }));

    (first_button, previous_button, next_button, last_button)
}

fn setup_interval_spinbutton(builder: Builder, state_cell: Rc<RefCell<State>>) -> SpinButton {
    let update_interval_spinbutton: SpinButton = builder.object("update_interval_spinbutton")
        .expect("Failed to get update_interval_spinbutton");
    let update_interval_spinbutton_adjustment = update_interval_spinbutton.adjustment();
    update_interval_spinbutton_adjustment.connect_value_changed(clone!(@strong update_interval_spinbutton,
                                                                       @weak state_cell => move |_| {
        let value = update_interval_spinbutton.value();
        state_cell.borrow_mut().update_interval = value as i32;
    }));

    update_interval_spinbutton
}

fn setup_timestep_interval_spinbutton(builder: Builder, state_cell: Rc<RefCell<State>>) -> SpinButton {
    let timestep_interval_spinbutton: SpinButton = builder.object("timestep_interval_spinbutton")
        .expect("Failed to get timestep_interval_spinbutton");
    let timestep_interval_spinbutton_adjustment = timestep_interval_spinbutton.adjustment();
    timestep_interval_spinbutton_adjustment.connect_value_changed(clone!(@strong timestep_interval_spinbutton,
                                                         @weak state_cell => move |_| {
        let value = timestep_interval_spinbutton.value();
        state_cell.borrow_mut().timestep_interval = value as usize;
    }));

    timestep_interval_spinbutton   
}

fn setup_autoscale_toggles(builder: Builder, state_cell: Rc<RefCell<State>>) -> (ToggleButton, ToggleButton) {
    let autoscale_x_toggle: ToggleButton = builder.object("autoscale_x_toggle")
        .expect("Failed to get autoscale_x_toggle");
    autoscale_x_toggle.connect_toggled(clone!(@strong autoscale_x_toggle,
                                            @weak state_cell => move |_| {
        let checked = autoscale_x_toggle.is_active();
        if checked {
            state_cell.borrow_mut().plot_settings.plot_range_x = PlotRange::Auto;
            state_cell.borrow_mut().update_needed = true;
        } else {
            let mut state = state_cell.borrow_mut();
            state.plot_settings.plot_range_x = state.plot_range_x_actual;
        }
    }));
    autoscale_x_toggle.set_active(true);

    // Autoscale toggle setup
    let autoscale_y_toggle: ToggleButton = builder.object("autoscale_y_toggle")
        .expect("Failed to get autoscale_y_toggle");
    autoscale_y_toggle.connect_toggled(clone!(@strong autoscale_y_toggle,
                                            @weak state_cell => move |_| {
        let checked = autoscale_y_toggle.is_active();
        if checked {
            state_cell.borrow_mut().plot_settings.plot_range_y = PlotRange::Auto;
            state_cell.borrow_mut().update_needed = true;
        } else {
            let mut state = state_cell.borrow_mut();
            state.plot_settings.plot_range_y = state.plot_range_y_actual;
        }
    }));
    autoscale_y_toggle.set_active(true);

    (autoscale_x_toggle, autoscale_y_toggle)
}

fn setup_logscale_toggles(builder: Builder, state_cell: Rc<RefCell<State>>) -> (ToggleButton, ToggleButton) {
    let logscale_x_toggle: ToggleButton = builder.object("logscale_x_toggle")
        .expect("Failed to get logscale_x_toggle");
    logscale_x_toggle.connect_toggled(clone!(@strong logscale_x_toggle,
                                             @weak state_cell => move |_| {
        state_cell.borrow_mut().plot_settings.use_logscale_x = logscale_x_toggle.is_active();
        state_cell.borrow_mut().update_needed = true;
    }));
    logscale_x_toggle.set_active(false);

    // Logscale y toggle setup
    let logscale_y_toggle: ToggleButton = builder.object("logscale_y_toggle")
        .expect("Failed to get logscale_x_toggle");
    logscale_y_toggle.connect_toggled(clone!(@strong logscale_y_toggle,
                                             @weak state_cell => move |_| {
        state_cell.borrow_mut().plot_settings.use_logscale_y = logscale_y_toggle.is_active();
        state_cell.borrow_mut().update_needed = true;
    }));
    logscale_y_toggle.set_active(false);

    (logscale_x_toggle, logscale_y_toggle)
}

fn setup_style_toggles(builder: Builder, state_cell: Rc<RefCell<State>>) -> (ToggleButton, ToggleButton, ToggleButton) {
    let line_toggle: ToggleButton = builder.object("line_toggle")
        .expect("Failed to get line_toggle");
    line_toggle.connect_toggled(clone!(@strong line_toggle,
                                       @weak state_cell => move |_| {
        state_cell.borrow_mut().plot_settings.draw_lines = line_toggle.is_active();
        state_cell.borrow_mut().update_needed = true;
    }));
    line_toggle.set_active(true);

    // Point toggle setup
    let point_toggle: ToggleButton = builder.object("point_toggle")
        .expect("Failed to get point_toggle");
    point_toggle.connect_toggled(clone!(@strong point_toggle,
                                        @weak state_cell => move |_| {
        state_cell.borrow_mut().plot_settings.draw_points = point_toggle.is_active();
        state_cell.borrow_mut().update_needed = true;
    }));
    point_toggle.set_active(true);

    let color_toggle: ToggleButton = builder.object("color_toggle")
        .expect("Failed to get color_toggle");
    color_toggle.connect_toggled(clone!(@strong color_toggle,
                                        @weak state_cell => move |_| {
        state_cell.borrow_mut().plot_settings.use_color = color_toggle.is_active();
        state_cell.borrow_mut().update_needed = true;
    }));
    color_toggle.set_active(true);

    (line_toggle, point_toggle, color_toggle)
}

fn setup_plot_range_entries(builder: Builder, state_cell: Rc<RefCell<State>>, toggles: (ToggleButton, ToggleButton)) -> (Entry, Entry, Entry, Entry) {
    let (autoscale_x_toggle, autoscale_y_toggle) = toggles;
    
    let x_min_entry: Entry = builder.object("x_min_entry")
        .expect("Failed to get x_min_entry");
    
    let x_max_entry: Entry = builder.object("x_max_entry")
        .expect("Failed to get x_max_entry");
    
    let y_min_entry: Entry = builder.object("y_min_entry")
        .expect("Failed to get y_min_entry");
    
    let y_max_entry: Entry = builder.object("y_max_entry")
        .expect("Failed to get y_max_entry");

    x_min_entry.connect_activate(clone!(@strong x_min_entry,
                                        @strong x_max_entry,
                                        @strong y_min_entry,
                                        @strong y_max_entry,
                                        @strong autoscale_x_toggle as x_toggle,
                                        @strong autoscale_y_toggle as y_toggle,
                                        @weak state_cell => move |_| {
        update_ranges(&x_min_entry, &x_max_entry, &y_min_entry, &y_max_entry, &x_toggle, &y_toggle, state_cell);
    }));

    x_max_entry.connect_activate(clone!(@strong x_min_entry,
                                        @strong x_max_entry,
                                        @strong y_min_entry,
                                        @strong y_max_entry,
                                        @strong autoscale_x_toggle as x_toggle,
                                        @strong autoscale_y_toggle as y_toggle,
                                        @weak state_cell => move |_| {
        update_ranges(&x_min_entry, &x_max_entry, &y_min_entry, &y_max_entry, &x_toggle, &y_toggle, state_cell);
    }));

    y_min_entry.connect_activate(clone!(@strong x_min_entry,
                                        @strong x_max_entry,
                                        @strong y_min_entry,
                                        @strong y_max_entry,
                                        @strong autoscale_x_toggle as x_toggle,
                                        @strong autoscale_y_toggle as y_toggle,
                                        @weak state_cell => move |_| {
        update_ranges(&x_min_entry, &x_max_entry, &y_min_entry, &y_max_entry, &x_toggle, &y_toggle, state_cell);
    }));

    y_max_entry.connect_activate(clone!(@strong x_min_entry,
                                        @strong x_max_entry,
                                        @strong y_min_entry,
                                        @strong y_max_entry,
                                        @strong autoscale_x_toggle as x_toggle,
                                        @strong autoscale_y_toggle as y_toggle,
                                        @weak state_cell => move |_| {
        update_ranges(&x_min_entry, &x_max_entry, &y_min_entry, &y_max_entry, &x_toggle, &y_toggle, state_cell);
    }));

    (x_min_entry, x_max_entry, y_min_entry, y_max_entry)
}

fn setup_load_button(builder: Builder, state_cell: Rc<RefCell<State>>, window: ApplicationWindow) -> Button {
    let load_button: Button = builder.object("load_button")
        .expect("Failed to get load_button");
    load_button.connect_clicked(clone!(@strong window,
                                       @weak state_cell => move |_| {
        let file_chooser_dialog = FileChooserDialog::new(Some("Load Data"), Some(&window), gtk::FileChooserAction::Open);
        file_chooser_dialog.add_button("Cancel", ResponseType::Cancel);
        file_chooser_dialog.add_button("Open", ResponseType::Accept);
        file_chooser_dialog.set_select_multiple(true);
        file_chooser_dialog.connect_response(move |d,r| {
            if let ResponseType::Accept = r {
                let filenames = d.filenames();
                let filenames: Vec<String> = filenames.iter().map(|pb| pb.as_path().display().to_string()).collect();
                if let Some(data) = Data::from_files(filenames) {
                    state_cell.borrow_mut().load_data(data);
                }
            }
        });
        file_chooser_dialog.run();
        file_chooser_dialog.hide();
    }));

    load_button
}

fn setup_save_button(builder: Builder, state_cell: Rc<RefCell<State>>, window: ApplicationWindow) -> Button {
    let save_plot_button: Button = builder.object("save_plot_button")
        .expect("Failed to get save_button");
    save_plot_button.connect_clicked(clone!(@strong window,
                                            @weak state_cell => move |_| {

        let file_chooser_dialog = FileChooserDialog::new(Some("Save Plot"), Some(&window), gtk::FileChooserAction::Save);
        file_chooser_dialog.add_button("Cancel", ResponseType::Cancel);
        file_chooser_dialog.add_button("Save", ResponseType::Accept);
        file_chooser_dialog.connect_response(move |d, r| {
            if let ResponseType::Accept = r {
                if let Some(filename) = d.filename() {
                    let filename = filename.as_path().display().to_string();     
                    if let Some(svg_string) = state_cell.borrow().plot_image_string.clone() {
                        std::fs::write(filename, svg_string).expect("Failed to write file");
                    }
                }
            }
        });
        file_chooser_dialog.run();
        file_chooser_dialog.hide(); 
    }));

    save_plot_button
}

fn setup_export_gnuplot_button(builder: Builder, state_cell: Rc<RefCell<State>>, window: ApplicationWindow) -> Button {
    let export_gnuplot_button: Button = builder.object("export_gnuplot_button")
        .expect("Failed to get export_gnuplot_button");
    export_gnuplot_button.connect_clicked(clone!(@strong window,
                                                 @weak state_cell => move |_| {
        let file_chooser_dialog = FileChooserDialog::new(Some("Export as"), Some(&window), gtk::FileChooserAction::Save);
        file_chooser_dialog.add_button("Cancel", ResponseType::Cancel);
        file_chooser_dialog.add_button("Save", ResponseType::Accept);
        file_chooser_dialog.connect_response(move |d, r| {
            if let ResponseType::Accept = r {
                if let Some(filename) = d.filename() {
                    let filename = filename.as_path().display().to_string();
                    if let Some(dataslice) = &state_cell.borrow().current_slice {
                        let gnuplot_string = dataslice.to_string_gnuplot();
                        std::fs::write(filename, gnuplot_string).expect("Failed to write file");
                    }
                }
            }
        });

        file_chooser_dialog.run();
        file_chooser_dialog.hide();
    }));

    export_gnuplot_button
}

fn update_ranges(x_min_entry: &Entry,
                 x_max_entry: &Entry,
                 y_min_entry: &Entry,
                 y_max_entry: &Entry,
                 x_toggle: &ToggleButton,
                 y_toggle: &ToggleButton,
                 state_cell: Rc<RefCell<State>>) -> () {

    let (x_min_old, x_max_old) = state_cell.borrow().plot_range_x_actual.get();
    let (y_min_old, y_max_old) = state_cell.borrow().plot_range_y_actual.get();

    let x_min_text = x_min_entry.buffer().text();
    let x_max_text = x_max_entry.buffer().text();
    let y_min_text = y_min_entry.buffer().text();
    let y_max_text = y_max_entry.buffer().text();

    let x_min_new = match x_min_text.parse::<f64>() {
        Ok(min) => min,
        Err(_) => x_min_old,
    };

    let x_max_new = match x_max_text.parse::<f64>() {
        Ok(max) => max,
        Err(_) => x_max_old,
    };

    let x_range_new = PlotRange::Fixed((x_min_new, x_max_new));

    let y_min_new = match y_min_text.parse::<f64>() {
        Ok(min) => min,
        Err(_) => y_min_old,
    };

    let y_max_new = match y_max_text.parse::<f64>() {
        Ok(max) => max,
        Err(_) => y_max_old,
    };

    let y_range_new = PlotRange::Fixed((y_min_new, y_max_new));

    let x_min_old_text = format!("{:.3}", x_min_old);
    let x_max_old_text = format!("{:.3}", x_max_old);
    let y_min_old_text = format!("{:.3}", y_min_old);
    let y_max_old_text = format!("{:.3}", y_max_old);

    let x_changed = (x_min_text != x_min_old_text) || (x_max_text != x_max_old_text);
    let y_changed = (y_min_text != y_min_old_text) || (y_max_text != y_max_old_text);

    println!("x_changed = {}", x_changed);
    println!("y_changed = {}", y_changed);

    x_toggle.set_active(!x_changed);
    y_toggle.set_active(!y_changed);

    state_cell.borrow_mut().plot_settings.plot_range_x = x_range_new;
    state_cell.borrow_mut().plot_settings.plot_range_y = y_range_new;
    
    state_cell.borrow_mut().update_needed = true;
}

fn plot_range_from_selection(image_pos_1: (f64, f64),
                             image_pos_2: (f64, f64),
                             state_cell: Rc<RefCell<State>>) -> (PlotRange, PlotRange) {
    // CAUTION: Image coordinates are measured from the top, plot coordinates from the bottom!

    // Image coordinates of the selection
    let (xi1, yi1) = image_pos_1;
    let (xi2, yi2) = image_pos_2;
    let xi_min = xi1.min(xi2);
    let xi_max = xi1.max(xi2);
    let yi_min = yi1.min(yi2);
    let yi_max = yi1.max(yi2);

    if let None = &state_cell.borrow().plot_image_pixbuf {
        return (PlotRange::Auto, PlotRange::Auto)
    };

    let (xp_min_new, yp_min_new) = plot_coords_from_image_coords(xi_min, yi_max, state_cell.clone());
    let (xp_max_new, yp_max_new) = plot_coords_from_image_coords(xi_max, yi_min, state_cell.clone());

    let x_range = PlotRange::Fixed((xp_min_new, xp_max_new));
    let y_range = PlotRange::Fixed((yp_min_new, yp_max_new));

    (x_range, y_range)
}

fn plot_range_after_scroll(state_cell: Rc<RefCell<State>>, event: &EventScroll, scroll_factor: f64) -> (PlotRange, PlotRange) {
    let shift_pressed = event.state().contains(ModifierType::SHIFT_MASK);
    let ctrl_pressed = event.state().contains(ModifierType::CONTROL_MASK);
    let alt_pressed = event.state().contains(ModifierType::MOD1_MASK);

    let (xi, yi) = event.position();
    let (xp, yp) = plot_coords_from_image_coords(xi, yi, state_cell.clone());

    let x_range_old = state_cell.borrow().plot_settings.plot_range_x;
    let y_range_old = state_cell.borrow().plot_settings.plot_range_y;

    let (x_min, x_max) = state_cell.borrow().plot_range_x_actual.get();
    let (y_min, y_max) = state_cell.borrow().plot_range_y_actual.get();

    let mut x_changed = false;
    let mut y_changed = false;

    let mut x_min_new = x_min;
    let mut x_max_new = x_max;
    let mut y_min_new = y_min;
    let mut y_max_new = y_max;


    if ctrl_pressed { // zooming
        let dx_plus = x_max - xp;
        let dx_minus = xp - x_min;
        let dy_plus = y_max - yp;
        let dy_minus = yp - y_min;

        match event.direction() {
            ScrollDirection::Up => {
                if !alt_pressed {
                    x_min_new = x_min + 2.0 * scroll_factor * dx_minus;
                    x_max_new = x_max - 2.0 * scroll_factor * dx_plus;
                    x_changed = true;
                }
                if !shift_pressed {
                    y_min_new = y_min + 2.0 * scroll_factor * dy_minus;
                    y_max_new = y_max - 2.0 * scroll_factor * dy_plus;
                    y_changed = true;
                }
            },
            ScrollDirection::Down => {
                if !alt_pressed {
                    x_min_new = x_min - 2.0 * scroll_factor * dx_minus;
                    x_max_new = x_max + 2.0 * scroll_factor * dx_plus;
                    x_changed = true;
                }
                if !shift_pressed {
                    y_min_new = y_min - 2.0 * scroll_factor * dy_minus;
                    y_max_new = y_max + 2.0 * scroll_factor * dy_plus;
                    y_changed = true;
                }
            },
            ScrollDirection::Right => {
                if !shift_pressed {
                    x_min_new = x_min + 2.0 * scroll_factor * dx_minus;
                    x_max_new = x_max - 2.0 * scroll_factor * dx_plus;
                    x_changed = true;
                }
                if !alt_pressed {
                    y_min_new = y_min + 2.0 * scroll_factor * dy_minus;
                    y_max_new = y_max - 2.0 * scroll_factor * dy_plus;
                    y_changed = true;
                }
            },
            ScrollDirection::Left => {
                if !shift_pressed {
                    x_min_new = x_min + 2.0 * scroll_factor * dx_minus;
                    x_max_new = x_max - 2.0 * scroll_factor * dx_plus;
                    x_changed = true;
                }
                if !alt_pressed {
                    y_min_new = y_min + 2.0 * scroll_factor * dy_minus;
                    y_max_new = y_max - 2.0 * scroll_factor * dy_plus;
                    y_changed = true;
                }
            },
            _ => {},
        }
    } else { // scrolling
        let dx = x_max - x_min;
        let dy = y_max - y_min;

        match event.direction() {
            ScrollDirection::Up => {
                if shift_pressed {
                    x_min_new = x_min - scroll_factor * dx;
                    x_max_new = x_max - scroll_factor * dx;
                    x_changed = true;    
                } else {
                    y_min_new = y_min + scroll_factor * dy;
                    y_max_new = y_max + scroll_factor * dy;
                    y_changed = true;
                }
            },
            ScrollDirection::Down => {
                if shift_pressed {
                    x_min_new = x_min + scroll_factor * dx;
                    x_max_new = x_max + scroll_factor * dx;
                    x_changed = true;    
                } else {
                    y_min_new = y_min - scroll_factor * dy;
                    y_max_new = y_max - scroll_factor * dy;
                    y_changed = true;
                }
            },
            ScrollDirection::Right => {
                if shift_pressed {
                    y_min_new = y_min - scroll_factor * dy;
                    y_max_new = y_max - scroll_factor * dy;
                    y_changed = true;   
                } else {
                    x_min_new = x_min + scroll_factor * dx;
                    x_max_new = x_max + scroll_factor * dx;
                    x_changed = true;
                }
            },
            ScrollDirection::Left => {
                if shift_pressed {
                    y_min_new = y_min + scroll_factor * dy;
                    y_max_new = y_max + scroll_factor * dy;
                    y_changed = true;   
                } else {
                    x_min_new = x_min - scroll_factor * dx;
                    x_max_new = x_min - scroll_factor * dx;
                    x_changed = true;
                }
            },
            _ => {},
        }
    }

    let x_range_new = if x_changed {
        PlotRange::Fixed((x_min_new, x_max_new))
    } else {
        x_range_old
    };

    let y_range_new = if y_changed {
        PlotRange::Fixed((y_min_new, y_max_new))
    } else {
        y_range_old
    };

    (x_range_new, y_range_new)
}

fn plot_coords_from_image_coords(xi: f64, yi: f64, state_cell: Rc<RefCell<State>>) -> (f64, f64) {
    // Extents of the full plot image
    let (image_width, image_height) = if let Some(buf) = &state_cell.borrow().plot_image_pixbuf {
        (buf.width() as f64, buf.height() as f64)
    } else {
        return (0.0, 0.0)
    };

    // Images generated by plotters have margins on the left and bottom
    const MARGIN_LEFT: f64 = 50.0;
    const MARGIN_BOTTOM: f64 = 21.0;

    // Adjust image coordinates and extents to take margins into account
    // Also clip selection to the plot area
    let image_width = image_width - MARGIN_LEFT;
    let image_height = image_height - MARGIN_BOTTOM;
    let xi = (xi - MARGIN_LEFT).max(0.0);
    let yi = yi.max(0.0);

    // Current plot ranges
    let (xp_min, xp_max) = state_cell.borrow().plot_range_x_actual.get();
    let (yp_min, yp_max) = state_cell.borrow().plot_range_y_actual.get();

    let log_x = state_cell.borrow().plot_settings.use_logscale_x;
    let log_y = state_cell.borrow().plot_settings.use_logscale_y;

    let plot_width = if log_x {
        xp_max.log(10.0) - xp_min.log(10.0)
    } else {
        xp_max - xp_min
    };

    let plot_height = if log_y {
        yp_max.log(10.0) - yp_min.log(10.0)
    } else {
        yp_max - yp_min
    };

    // Map image coordinates to plot coordinates
    // Here, y_max and y_min have opposite meanings between image and plot coordinates
    let xp = if log_x {
        10.0_f64.powf((xi / image_width) * plot_width + xp_min.log(10.0))
    } else {
        (xi / image_width) * plot_width + xp_min
    };

    let yp = if log_y {
        10.0_f64.powf((1.0 - (yi / image_height)) * plot_height + yp_min.log(10.0))
    } else {
        (1.0 - (yi / image_height)) * plot_height + yp_min
    };

    (xp, yp)
}