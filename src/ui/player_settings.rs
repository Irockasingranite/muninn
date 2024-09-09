use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct PlayerSettingsModel {
    pub update_interval_ms: u64,
    pub step_size: usize,
}

impl Default for PlayerSettingsModel {
    fn default() -> Self {
        Self {
            update_interval_ms: 100,
            step_size: 1,
        }
    }
}

#[derive(Debug)]
pub enum PlayerSettingsMsg {
    SetUpdateInterval(u64),
    SetStepSize(usize),
}

#[relm4::component(pub)]
impl SimpleComponent for PlayerSettingsModel {
    type Init = ();
    type Input = PlayerSettingsMsg;
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 5,
            set_valign: gtk::Align::Center,

            gtk::Label::new(Some("Update interval [ms]")) {},

            #[name = "update_interval_selector"]
            gtk::SpinButton {
                set_digits: 0,
                set_adjustment: &gtk::Adjustment::builder()
                    .lower(10.0)
                    .upper(10000.0)
                    .step_increment(10.0)
                    .value(100.0)
                    .build(),

                connect_value_changed[sender, update_interval_selector] => move |s| {
                    sender.input(PlayerSettingsMsg::SetUpdateInterval(s.value() as u64));
                }
            },

            gtk::Label::new(Some("Timestep interval")) {},

            #[name = "timestep_interval_selector"]
            gtk::SpinButton {
                set_digits: 0,
                set_max_width_chars: 3,
                set_adjustment: &gtk::Adjustment::builder()
                    .lower(1.0)
                    .upper(1000000.0)
                    .step_increment(1.0)
                    .value(1.0)
                    .build(),

                connect_value_changed[sender, timestep_interval_selector] => move |s| {
                    sender.input(PlayerSettingsMsg::SetStepSize(s.value() as usize));
                }
            },
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self::default();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            PlayerSettingsMsg::SetUpdateInterval(interval) => {
                self.update_interval_ms = interval;
            }
            PlayerSettingsMsg::SetStepSize(step_size) => {
                self.step_size = step_size;
            }
        }
    }
}
