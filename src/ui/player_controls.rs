use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct PlayerControlsModel {
    pub is_playing: bool,
}

#[derive(Debug)]
pub enum PlayerControlsMsg {
    Play,
    Pause,
    FirstStep,
    StepBackward,
    StepForward,
    LastStep,
}

#[relm4::component(pub)]
impl SimpleComponent for PlayerControlsModel {
    type Init = ();
    type Input = PlayerControlsMsg;
    type Output = PlayerControlsMsg;

    view! {
        gtk::Box {
            add_css_class: "toolbar",
            set_spacing: 1,

            gtk::Button::from_icon_name("media-skip-backward") {
                connect_clicked => PlayerControlsMsg::FirstStep
            },

            gtk::Button::from_icon_name("media-seek-backward") {
                connect_clicked => PlayerControlsMsg::StepBackward
            },

            if !model.is_playing {
                gtk::Button::from_icon_name("media-playback-start") {
                    connect_clicked => PlayerControlsMsg::Play
                }
            } else {
                gtk::Button::from_icon_name("media-playback-pause") {
                    connect_clicked => PlayerControlsMsg::Pause
                }
            },

            gtk::Button::from_icon_name("media-seek-forward") {
                connect_clicked => PlayerControlsMsg::StepForward
            },

            gtk::Button::from_icon_name("media-skip-forward") {
                connect_clicked => PlayerControlsMsg::LastStep
            },
        },
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            PlayerControlsMsg::Play => {
                self.is_playing = true;
                _ = sender.output(PlayerControlsMsg::Play);
            }
            PlayerControlsMsg::Pause => {
                self.is_playing = false;
                _ = sender.output(PlayerControlsMsg::Pause);
            }
            PlayerControlsMsg::FirstStep => {
                _ = sender.output(PlayerControlsMsg::FirstStep);
            }
            PlayerControlsMsg::StepBackward => {
                _ = sender.output(PlayerControlsMsg::StepBackward);
            }
            PlayerControlsMsg::StepForward => {
                _ = sender.output(PlayerControlsMsg::StepForward);
            }
            PlayerControlsMsg::LastStep => {
                _ = sender.output(PlayerControlsMsg::LastStep);
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = PlayerControlsModel { is_playing: false };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
