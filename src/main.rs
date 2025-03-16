mod window;
mod task_object;
mod task_row;
mod utils;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;
use crate::utils::get_app_id;

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("templates.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(get_app_id()).build();

    // Connect to "activate" signal of `app`
    app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);

    // Set keyboard accelerator to trigger "win.close".
    app.set_accels_for_action("win.close", &["<Ctrl>W", "<Meta>Q"]);

    // Run the application
    app.run()
}

// ANCHOR: build_ui
fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a", "<Meta>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o", "<Meta>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d", "<Meta>d"]);
}
