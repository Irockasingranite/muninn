use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct PlayerModel {
    pub is_playing: bool,
    update_interval: u32,
    step_size: usize,
}

#[derive(Debug)]
pub enum PlayerMsg {
    Play,
    Pause,
    FirstStep,
    PreviousStep,
    NextStep,
    LastStep,
    SetUpdateInterval(u32),
    SetStepSize(usize),
}

#[relm4::component(pub)]
impl SimpleComponent for PlayerModel {
    type Init = ();
    type Input = PlayerMsg;
    type Output = PlayerMsg;

    view! {
        gtk::Box {
            add_css_class: "toolbar",
            set_spacing: 1,

            gtk::Button::from_icon_name("media-skip-backward") {
                connect_clicked => PlayerMsg::FirstStep
            },

            gtk::Button::from_icon_name("media-seek-backward") {
                connect_clicked => PlayerMsg::PreviousStep
            },

            if !model.is_playing {
                gtk::Button::from_icon_name("media-playback-start") {
                    connect_clicked => PlayerMsg::Play
                }
            } else {
                gtk::Button::from_icon_name("media-playback-pause") {
                    connect_clicked => PlayerMsg::Pause
                }
            },

            gtk::Button::from_icon_name("media-seek-forward") {
                connect_clicked => PlayerMsg::NextStep
            },

            gtk::Button::from_icon_name("media-skip-forward") {
                connect_clicked => PlayerMsg::LastStep
            },

            gtk::Separator {},

            gtk::Label::new(Some("t = ")) {},

            gtk::Entry {},

            gtk::Separator {},

            gtk::Separator {},

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
                    sender.input(PlayerMsg::SetUpdateInterval(s.value() as u32));
                }
            },

            gtk::Label::new(Some("Timestep interval")) {},

            #[name = "timestep_interval_selector"]
            gtk::SpinButton {
                set_digits: 0,
                set_adjustment: &gtk::Adjustment::builder()
                    .lower(1.0)
                    .upper(1000000.0)
                    .step_increment(1.0)
                    .value(1.0)
                    .build(),

                connect_value_changed[sender, timestep_interval_selector] => move |s| {
                    sender.input(PlayerMsg::SetStepSize(s.value() as usize));
                }
            },
        },
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            PlayerMsg::Play => {
                self.is_playing = true;
                _ = sender.output(PlayerMsg::Play);
            }
            PlayerMsg::Pause => {
                self.is_playing = false;
                _ = sender.output(PlayerMsg::Pause);
            }
            PlayerMsg::FirstStep => {
                _ = sender.output(PlayerMsg::FirstStep);
            }
            PlayerMsg::PreviousStep => {
                _ = sender.output(PlayerMsg::PreviousStep);
            }
            PlayerMsg::NextStep => {
                _ = sender.output(PlayerMsg::NextStep);
            }
            PlayerMsg::LastStep => {
                _ = sender.output(PlayerMsg::LastStep);
            }
            PlayerMsg::SetStepSize(step_size) => {
                self.step_size = step_size;
            }
            PlayerMsg::SetUpdateInterval(interval) => {
                self.update_interval = interval;
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = PlayerModel {
            is_playing: false,
            update_interval: 100,
            step_size: 1,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
