use relm4::RelmApp;

mod app;
mod header;

pub fn run_app() {
    let app = RelmApp::new("muninn.muninn");
    app.run::<app::AppModel>(());
}
