use gtk::prelude::*;
use relm4::{gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller, SimpleComponent};

use super::plot_settings::PlotSettingsModel;

pub struct PlotViewModel {
    settings: Controller<PlotSettingsModel>,
}

#[derive(Debug)]
pub enum PlotViewMsg {
    Ignore,
}

#[relm4::component(pub)]
impl SimpleComponent for PlotViewModel {
    type Init = ();
    type Input = PlotViewMsg;
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            model.settings.widget(),

            gtk::Label::new(Some("Plot here")),

            gtk::Label {
                #[watch]
                set_text: &format!("x-range: {:.3} - {:.3}", model.settings.model().x_range_min, model.settings.model().x_range_max),
            },

            gtk::Label {
                #[watch]
                set_text: &format!("y-range: {:.3} - {:.3}", model.settings.model().y_range_min, model.settings.model().y_range_max),
            }
        },
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = PlotSettingsModel::builder()
            .launch(())
            .forward(sender.input_sender(), |_| PlotViewMsg::Ignore);

        let model = Self { settings };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        _msg: Self::Input,
        _sender: ComponentSender<Self>,
    ) -> Self::Output {
        ()
    }
}
