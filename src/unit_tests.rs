#[cfg(test)]
mod tests {
    use crate::audio_operations;

    #[test]
    fn test_find_duration() {
        let path = "".to_string();
        let duration = audio_operations::find_duration(path);
        assert_eq!(duration.as_secs(), 0);
    }

    #[test]
    fn test_alert_user_false() {
        let title = "".to_string();
        let message = "".to_string();
        let result = crate::user_interactions::alert_user(title, message);
        assert_eq!(result, false);
    }

    #[test]
    fn test_alert_user_true() {
        let title = "Title".to_string();
        let message = "Message".to_string();
        let result = crate::user_interactions::alert_user(title, message);
        assert_eq!(result, true);
    }

}