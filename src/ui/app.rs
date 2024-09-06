use std::path::PathBuf;

use crate::data::Data;

use gtk::prelude::*;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

use super::header::{HeaderModel, HeaderOutput};
use super::player::PlayerModel;

pub struct AppModel {
    data: Data,
    header: Controller<HeaderModel>,
    player: Controller<PlayerModel>,
}

#[derive(Debug)]
pub enum AppMsg {
    OpenFiles(Vec<PathBuf>),
    Ignore,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = Data;
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Muninn"),
            set_default_width: 800,
            set_default_height: 600,

            set_titlebar: Some(model.header.widget()),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                model.player.widget(),

                gtk::Label {
                    set_label: "Hello Muninn!",
                },

                gtk::Label {
                    set_label: &format!("{} timeslices loaded", model.data.times().len()),
                },

                if model.player.model().is_playing {
                    gtk::Label {
                        set_label: "Playing!",
                    }
                } else {
                    gtk::Label {}
                },
            },
        }
    }

    fn init(
        data: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header = HeaderModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| match msg {
                HeaderOutput::OpenFiles(paths) => AppMsg::OpenFiles(paths),
            });

        let player = PlayerModel::builder()
            .launch(())
            .forward(sender.input_sender(), |_| AppMsg::Ignore);

        let model = AppModel {
            data,
            header,
            player,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::OpenFiles(paths) => println!("Opening {:?}", paths),
            AppMsg::Ignore => (),
        }
    }
}
