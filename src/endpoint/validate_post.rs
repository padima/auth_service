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
    use std::time::{SystemTime, UNIX_EPOCH};

    fn now_unix_timestamp() -> usize {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs() as usize
    }

    #[tokio::test]
    async fn test_validate_is_valid() {
        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
        let now = now_unix_timestamp();
        let claims = Claims {
            sub: Some("user123".to_string()),
            exp: Some(now + 3600), // Token expires in 1 hour
            iat: Some(now),
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
        let now = now_unix_timestamp();
        let claims = Claims {
            sub: Some("user123".to_string()),
            exp: Some(now + 3600), // Token expires in 1 hour
            iat: Some(now),
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

    #[test]
    fn test_validate_request_clone() {
        let request = ValidateRequest {
            token: "jwt_token".to_string(),
            key: Some("my_key".to_string()),
        };

        let cloned = request.clone();
        assert_eq!(cloned.token, request.token);
        assert_eq!(cloned.key, request.key);
    }

    #[test]
    fn test_validate_request_deserialize() {
        let json = r#"{"token":"jwt_token","key":"my_key"}"#;
        let request: ValidateRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.token, "jwt_token");
        assert_eq!(request.key, Some("my_key".to_string()));
    }

    #[test]
    fn test_validate_response_clone() {
        let response = ValidateResponse {
            is_valid: true,
            claims: Some(Claims::new(Some("user123".to_string()))),
        };

        let cloned = response.clone();
        assert_eq!(cloned.is_valid, response.is_valid);
        assert_eq!(
            cloned.claims.as_ref().and_then(|c| c.sub.clone()),
            Some("user123".to_string())
        );
    }

    #[test]
    fn test_validate_response_serialize() {
        let response = ValidateResponse {
            is_valid: true,
            claims: Some(Claims::new(Some("user123".to_string()))),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let value: serde_json::Value = serde_json::from_str(&serialized).unwrap();

        assert_eq!(value["is_valid"], true);
        assert_eq!(value["claims"]["sub"], "user123");
    }
}
