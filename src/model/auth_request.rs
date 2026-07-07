use serde::{ Serialize, Deserialize };

use crate::model::Claims;

/// Request structure for authentication.
#[derive(Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    pub user_id: String,
    pub claims: Option<Claims>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_request_creation() {
        let auth_request = AuthRequest {
            user_id: "user123".to_string(),
            claims: None,
        };
        assert_eq!(auth_request.user_id, "user123");
    }
}
