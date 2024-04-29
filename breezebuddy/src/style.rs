use gtk::{gdk::Display, CssProvider};
use gtk4 as gtk;

/// loads the sass file and adds it to the provider
/// then adds the provider to the default screen
pub fn load_sass() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("sass/style.sass"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
