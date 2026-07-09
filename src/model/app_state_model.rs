/// Application state shared across handlers.
#[derive(Clone)]
pub struct AppState {
    pub key: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let key = "my_secret_key".to_string();
        let _ = AppState { key };
        assert!(true);
    }
    #[test]
    fn test_app_state_clone() {
        let key = "my_secret_key".to_string();
        let app_state = AppState { key };
        let cloned_app_state = app_state.clone();

        assert_eq!(app_state.key, cloned_app_state.key);
    }
}
