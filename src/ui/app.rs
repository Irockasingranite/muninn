use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct AppModel;

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::ApplicationWindow {
            set_title: Some("Muninn"),
            set_default_width: 800,
            set_default_height: 600,

            gtk::Label {
                set_label: "Hello Muninn!",
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}

