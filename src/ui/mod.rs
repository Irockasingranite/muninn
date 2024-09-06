use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use relm4::{gtk, RelmApp};

use crate::data::Data;

mod app;
mod header;
mod player;

pub fn run_app() {
    let mut flags = ApplicationFlags::empty();
    flags.insert(ApplicationFlags::HANDLES_OPEN);
    flags.insert(ApplicationFlags::NON_UNIQUE);

    let app = gtk::Application::builder()
        .application_id("muninn.muninn")
        .flags(flags)
        .build();

    // Dummy handler, files are actually handled below and data passed into the
    // AppModel
    app.connect_open(|app, _files, _hint| {
        app.activate();
    });

    let filenames = std::env::args().collect();
    let data = Data::from_files(filenames).unwrap_or(Data::new());

    let app = RelmApp::from_app(app);
    app.run::<app::AppModel>(data);
}
