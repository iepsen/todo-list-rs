mod window;
mod task_object;
mod task_row;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

const APP_ID: &str = "com.github.iepsen.todo-list-rs";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("templates.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
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