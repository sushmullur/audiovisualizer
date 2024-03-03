mod audio_operations;
mod user_interactions;

fn main() {

    let path = user_interactions::select_file();

    if path == "".to_string() {
        user_interactions::alert_user("Error".to_string(), "No file selected".to_string());
        return;
    }
    
    user_interactions::alert_user("File Loaded".to_string(), format!("You selected: {}", path));

    let duration = audio_operations::find_duration(path.clone());

    audio_operations::play_audio(path, duration);
}