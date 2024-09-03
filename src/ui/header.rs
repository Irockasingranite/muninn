use std::path::PathBuf;

use gtk::prelude::*;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};
use relm4_components::open_dialog::{
    OpenDialogMsg, OpenDialogMulti, OpenDialogResponse, OpenDialogSettings,
};

pub struct HeaderModel {
    open_files_dialog: Controller<OpenDialogMulti>,
}

#[derive(Debug)]
pub enum HeaderInput {
    ShowOpenFilesDialog,
    OpenFiles(Vec<PathBuf>),
    Ignore,
}

#[derive(Debug)]
pub enum HeaderOutput {
    OpenFiles(Vec<PathBuf>),
}

#[relm4::component(pub)]
impl SimpleComponent for HeaderModel {
    type Init = ();
    type Input = HeaderInput;
    type Output = HeaderOutput;

    view! {
        gtk::HeaderBar {

            pack_start = &gtk::Button::with_label("Open Files") {
                connect_clicked => HeaderInput::ShowOpenFilesDialog,
            },
        },
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let open_files_dialog = OpenDialogMulti::builder()
            .transient_for_native(&root)
            .launch(OpenDialogSettings {
                accept_label: String::from("Open Files"),
                ..Default::default()
            })
            .forward(sender.input_sender(), |msg| match msg {
                OpenDialogResponse::Accept(paths) => HeaderInput::OpenFiles(paths),
                _ => HeaderInput::Ignore,
            });

        let model = HeaderModel { open_files_dialog };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            HeaderInput::ShowOpenFilesDialog => {
                self.open_files_dialog
                    .sender()
                    .send(OpenDialogMsg::Open)
                    .unwrap();
            }
            HeaderInput::OpenFiles(paths) => {
                sender.output(HeaderOutput::OpenFiles(paths)).unwrap();
            }
            HeaderInput::Ignore => (),
        }
    }
}
