use std::path::PathBuf;

use gtk::prelude::*;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

use super::header::{HeaderModel, HeaderOutput};

pub struct AppModel {
    header: Controller<HeaderModel>,
}

#[derive(Debug)]
pub enum AppMsg {
    OpenFiles(Vec<PathBuf>),
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Muninn"),
            set_default_width: 800,
            set_default_height: 600,

            set_titlebar: Some(model.header.widget()),

            gtk::Label {
                set_label: "Hello Muninn!",
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header = HeaderModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| match msg {
                HeaderOutput::OpenFiles(paths) => AppMsg::OpenFiles(paths),
            });

        let model = AppModel { header };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::OpenFiles(paths) => println!("Opening {:?}", paths),
        }
    }
}
