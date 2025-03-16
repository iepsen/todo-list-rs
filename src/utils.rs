use std::path::PathBuf;
use gtk::glib;

pub fn data_path() -> PathBuf {
    let mut path = glib::user_data_dir();
    path.push(get_app_id());
    std::fs::create_dir_all(&path).expect("Could not create directory.");
    path.push("data.json");
    path
}

pub fn get_app_id() -> &'static str {
    "com.github.iepsen.todo-list-rs"
}
