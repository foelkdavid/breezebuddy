use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application};

mod ui;
mod style;

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id("BreezeBuddy").build();

    // Connect to signals
    app.connect_startup(|_| style::load_css());
    app.connect_activate(|app| ui::build_ui(app));

    // Run the application
    app.run()
}
