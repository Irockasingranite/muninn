use relm4::RelmApp;

mod app;
mod header;
mod player;

pub fn run_app() {
    let app = RelmApp::new("muninn.muninn");
    app.run::<app::AppModel>(());
}
