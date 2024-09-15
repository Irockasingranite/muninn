use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

#[derive(Debug, Clone)]
pub struct PlotSettingsModel {
    pub x_range_min: f64,
    pub x_range_max: f64,
    pub y_range_min: f64,
    pub y_range_max: f64,
    pub autoscale_x: bool,
    pub autoscale_y: bool,
    pub logscale_x: bool,
    pub logscale_y: bool,
}

#[derive(Debug)]
pub enum PlotSettingsInput {
    XRangeMinChanged(f64),
    XRangeMaxChanged(f64),
    YRangeMinChanged(f64),
    YRangeMaxChanged(f64),
}

#[derive(Debug)]
pub enum PlotSettingsOutput {
    PlotSettingsChanged(PlotSettingsModel),
}

#[relm4::component(pub)]
impl SimpleComponent for PlotSettingsModel {
    type Init = ();
    type Input = PlotSettingsInput;
    type Output = PlotSettingsOutput;

    view! {
        gtk::Box {
            add_css_class: "toolbar",

            // x-range entry
            gtk::Label::new(Some("x-range:")),
            gtk::Entry {
                    set_max_width_chars: 8,
                    set_valign: gtk::Align::Center,
                    // Display current setting
                    #[watch]
                    set_buffer: &gtk::EntryBuffer::builder()
                            .text(format!("{:.3}", model.x_range_min))
                        .build(),
                    // Send update message on enter press
                    connect_activate[sender] => move |entry| {
                        let text = entry.buffer().text();
                        if let Ok(value) = text.parse() {
                            sender.input(PlotSettingsInput::XRangeMinChanged(value));
                        }
                    },
            },
            gtk::Label::new(Some(" - ")),
            gtk::Entry {
                    set_max_width_chars: 8,
                    set_valign: gtk::Align::Center,
                    // Display current setting
                    set_buffer: &gtk::EntryBuffer::builder()
                            .text(format!("{:.3}", model.x_range_max))
                        .build(),
                    // Send update message on enter press
                    connect_activate[sender] => move |entry| {
                        let text = entry.buffer().text();
                        if let Ok(value) = text.parse() {
                            sender.input(PlotSettingsInput::XRangeMaxChanged(value));
                        }
                    },
            },

            gtk::Separator {},

            // y-range entry
            gtk::Label::new(Some("y-range:")),
            gtk::Entry {
                    set_max_width_chars: 8,
                    set_valign: gtk::Align::Center,
                    // Display current setting
                    set_buffer: &gtk::EntryBuffer::builder()
                            .text(format!("{:.3}", model.y_range_min))
                        .build(),
                    // Send update message on enter press
                    connect_activate[sender] => move |entry| {
                        let text = entry.buffer().text();
                        if let Ok(value) = text.parse() {
                            sender.input(PlotSettingsInput::YRangeMinChanged(value));
                        }
                    },
            },
            gtk::Label::new(Some(" - ")),
            gtk::Entry {
                    set_max_width_chars: 8,
                    set_valign: gtk::Align::Center,
                    // Display current setting
                    set_buffer: &gtk::EntryBuffer::builder()
                            .text(format!("{:.3}", model.y_range_max))
                        .build(),
                    // Send update message on enter press
                    connect_activate[sender] => move |entry| {
                        let text = entry.buffer().text();
                        if let Ok(value) = text.parse() {
                            sender.input(PlotSettingsInput::YRangeMaxChanged(value));
                        }
                    },
            },

            gtk::Separator {},

            gtk::Label::new(Some("Autoscale:")),
            gtk::ToggleButton {
                set_label: "x",
                set_active: model.autoscale_x,
            },
            gtk::ToggleButton {
                set_label: "y",
                set_active: model.autoscale_y,
            },

            gtk::Separator {},

            gtk::Label::new(Some("Logscale:")),
            gtk::ToggleButton {
                set_label: "x",
                set_active: model.logscale_x,
            },
            gtk::ToggleButton {
                set_label: "y",
                set_active: model.logscale_y,
            },
        },
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            x_range_min: 0.0,
            x_range_max: 1.0,
            y_range_min: 0.0,
            y_range_max: 1.0,
            autoscale_x: false,
            autoscale_y: false,
            logscale_x: false,
            logscale_y: false,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            PlotSettingsInput::XRangeMinChanged(value) => {
                self.x_range_min = value;
                sender
                    .output(PlotSettingsOutput::PlotSettingsChanged(self.clone()))
                    .unwrap();
            }
            PlotSettingsInput::XRangeMaxChanged(value) => {
                self.x_range_max = value;
                sender
                    .output(PlotSettingsOutput::PlotSettingsChanged(self.clone()))
                    .unwrap();
            }
            PlotSettingsInput::YRangeMinChanged(value) => {
                self.y_range_min = value;
                sender
                    .output(PlotSettingsOutput::PlotSettingsChanged(self.clone()))
                    .unwrap();
            }
            PlotSettingsInput::YRangeMaxChanged(value) => {
                self.y_range_max = value;
                sender
                    .output(PlotSettingsOutput::PlotSettingsChanged(self.clone()))
                    .unwrap();
            }
        }
    }
}
