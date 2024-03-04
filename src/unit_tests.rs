#[cfg(test)]
mod tests {
    use crate::audio_operations;

    #[test]
    fn test_find_duration() {
        let path = "".to_string();
        let duration = audio_operations::find_duration(path);
        assert_eq!(duration.as_secs(), 0);
    }

}