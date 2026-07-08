use crate::model::{AppState, Claims};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use jsonwebtoken::{Algorithm, Validation, decode};
use serde::{Deserialize, Serialize};

pub async fn validate_post(
    State(state): State<AppState>,
    Json(body): Json<ValidateRequest>,
) -> Result<Json<ValidateResponse>, StatusCode> {
    let key = jsonwebtoken::DecodingKey::from_secret(if let Some(ref key) = body.key {
        key.as_ref()
    } else {
        state.key.as_ref()
    });

    match decode::<Claims>(&body.token, &key, &Validation::new(Algorithm::HS256)) {
        Ok(data) => Ok(Json(ValidateResponse {
            is_valid: true,
            claims: Some(data.claims),
        })),
        Err(_) => Ok(Json(ValidateResponse {
            is_valid: false,
            claims: None,
        })),
    }
}

#[derive(Clone, Deserialize)]
pub struct ValidateRequest {
    pub token: String,
    pub key: Option<String>, // Optional key for validating the token
}

#[derive(Clone, Serialize)]
pub struct ValidateResponse {
    pub is_valid: bool,
    pub claims: Option<Claims>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::EncodingKey;

    #[tokio::test]
    async fn test_validate_is_valid() {
        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
        let claims = Claims {
            sub: Some("user123".to_string()),
            exp: Some((chrono::Utc::now().timestamp() as usize) + 3600), // Token expires in 1 hour
            iat: Some(chrono::Utc::now().timestamp() as usize),
            iss: None,
            aud: None,
        };
        let app_state = AppState {
            key: "my_secret_key".to_string(),
        };
        let key = EncodingKey::from_secret(app_state.key.as_ref());
        let token = jsonwebtoken::encode(&header, &claims, &key).expect("Failed to encode token");

        println!("Generated token: {}", token);

        let request = ValidateRequest { token, key: None };
        let result = validate_post(State(app_state), Json(request)).await;

        println!("Validation result: {:?}", result.as_ref().unwrap().is_valid);
        println!("Claims: {:?}", result.as_ref().unwrap().claims);

        assert!(result.as_ref().unwrap().is_valid);
    }

    #[tokio::test]
    async fn test_validate_is_not_valid() {
        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
        let claims = Claims {
            sub: Some("user123".to_string()),
            exp: Some((chrono::Utc::now().timestamp() as usize) + 3600), // Token expires in 1 hour
            iat: Some(chrono::Utc::now().timestamp() as usize),
            iss: None,
            aud: None,
        };

        let app_state = AppState {
            key: "my_secret_key".to_string(),
        };

        let key = EncodingKey::from_secret(app_state.key.as_ref());
        let token = jsonwebtoken::encode(&header, &claims, &key).expect("Failed to encode token");

        let request = ValidateRequest {
            token,
            key: Some("wrong_key".to_string()),
        };

        let result = validate_post(State(app_state), Json(request)).await;

        println!("Validation result: {:?}", result.as_ref().unwrap().is_valid);
        println!("Claims: {:?}", result.as_ref().unwrap().claims);

        assert!(!result.as_ref().unwrap().is_valid);
    }
}
