use native_dialog::FileDialog;

pub fn select_file() -> String {
    let path = FileDialog::new()
        .set_location("~/")
        .add_filter("Audio Files", &["wav", "mp3", "flac"])
        .show_open_single_file()
        .unwrap();
    let path = match path {
        Some(path) => path,
        None => return "".to_string(),
    };

    return path.to_str().unwrap().to_string();
}

pub fn alert_user(title: String, message: String) {
    let _alert = native_dialog::MessageDialog::new()
        .set_type(native_dialog::MessageType::Info)
        .set_title(&title)
        .set_text(&message)
        .show_alert()
        .unwrap();
}
