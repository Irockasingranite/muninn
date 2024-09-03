use relm4::RelmApp;

mod app;

pub fn run_app() {
    let app = RelmApp::new("muninn");
    app.run::<app::AppModel>(());
}
