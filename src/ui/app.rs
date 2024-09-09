use crate::data::Data;

use gtk::prelude::*;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

use super::header::{HeaderModel, HeaderOutput};
use super::player::{PlayerModel, PlayerMsg};

pub struct AppModel {
    header: Controller<HeaderModel>,
    player: Controller<PlayerModel>,
}

#[derive(Debug)]
pub enum AppMsg {
    OpenFiles(Vec<String>),
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
            set_default_width: 1200,
            set_default_height: 800,

            set_titlebar: Some(model.header.widget()),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                model.player.widget(),
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
            .launch(data)
            .forward(sender.input_sender(), |_| AppMsg::Ignore);

        let model = AppModel { header, player };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::OpenFiles(paths) => {
                if let Some(data) = Data::from_files(paths) {
                    _ = self.player.sender().send(PlayerMsg::NewData(data));
                }
            }
            AppMsg::Ignore => (),
        }
    }
}
