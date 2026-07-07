use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Option<String>, // ID користувача
    pub iss: Option<String>, // Optional. Issuer
    pub aud: Option<String>, // Optional. Audience
    pub exp: Option<usize>, // час закінчення дії токена (Unix timestamp)
    pub iat: Option<usize>, // час створення
}

impl Claims {
    pub fn clear() -> Self {
        Self { sub: None, iss: None, aud: None, exp: None, iat: None }
    }
}
