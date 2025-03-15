mod custom_button;

use gtk::{glib, Application, ApplicationWindow};
use gtk::prelude::*;
use crate::custom_button::CustomButton;

const APP_ID: &str = "com.github.iepsen.todo-list-rs";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button
    let button = CustomButton::new();
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("To-do List")
        .child(&button)
        .width_request(200)
        .height_request(140)
        .build();

    // Present window
    window.present();
}