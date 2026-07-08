use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Option<String>, // ID користувача
    pub iss: Option<String>, // Optional. Issuer
    pub aud: Option<String>, // Optional. Audience
    pub exp: Option<usize>,  // час закінчення дії токена (Unix timestamp)
    pub iat: Option<usize>,  // час створення
}

impl Claims {
    pub fn new(sub: Option<String>) -> Self {
        Self {
            sub,
            iss: None,
            aud: None,
            exp: None,
            iat: None,
        }
    }
}
