use serde::Serialize;

/// Response structure for authentication.
#[derive(Clone, Serialize)]
pub struct AuthResponse {
    pub token: String,
}
