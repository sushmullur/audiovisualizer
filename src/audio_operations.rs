use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

pub fn find_duration(path: String) -> std::time::Duration {
    mp3_duration::from_path(path).unwrap()
}

pub fn play_audio(path: String, duration: std::time::Duration) {
    if duration.as_secs() == 0 {
        return;
    }
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(path.clone()).unwrap());
    let source = Decoder::new(file).unwrap();
    
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(duration);
}