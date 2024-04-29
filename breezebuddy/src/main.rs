use gtk::prelude::*;
use gtk::{glib, Application};
use gtk4 as gtk;

mod actions;
mod style;
mod ui;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.foelk.BreezeBuddy")
        .build();

    // Connect to signals
    app.connect_startup(|_| style::load_sass());
    app.connect_activate(|app| ui::build_ui(app));

    app.run()
}
