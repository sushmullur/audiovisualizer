use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
extern crate mp3_duration;

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
    let path_clone = path.clone();

    let _yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Success")
        .set_text(&format!("You selected: {}", path.display()))
        .show_confirm()
        .unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();

    let duration = mp3_duration::from_path(path_clone).unwrap();
    println!("Duration: {:?}", duration);
    

    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(9));


}