use native_dialog::{FileDialog, MessageDialog, MessageType};

fn main() {
    let path = FileDialog::new()
        .set_location("~/")
        .add_filter("Audio Files", &["wav", "mp3", "flac"])
        .show_open_single_file()
        .unwrap();
    let path = match path {
        Some(path) => path,
        None => return,
    };

    let yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Success")
        .set_text(&format!("You selected: {}", path.display()))
        .show_confirm()
        .unwrap();

    if yes {
        println!("User clicked yes {}", path.display());
    } else {
        println!("User clicked no");
    }
}