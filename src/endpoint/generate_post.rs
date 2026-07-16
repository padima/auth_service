use crate::constants;
use crate::model::{AppState, Claims};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use jsonwebtoken::EncodingKey;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn generate_post(
    State(state): State<AppState>,
    Json(body): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, StatusCode> {
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .as_secs() as usize;
    let mut claims = body
        .claims
        .clone()
        .unwrap_or(Claims::new(Some(body.user_id.clone())));

    if claims.sub.is_none() {
        claims.sub = Some(body.user_id);
    }

    if claims.iat.is_none() {
        claims.iat = Some(now);
    }

    if claims.exp.is_none() {
        claims.exp = Some(now + constants::TOKEN_EXPIRATION);
    }

    let key = EncodingKey::from_secret(body.key.as_ref().unwrap_or(&state.key).as_ref());

    let token = jsonwebtoken::encode(&header, &claims, &key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(GenerateResponse { token }))
}

#[derive(Clone, Deserialize)]
pub struct GenerateRequest {
    pub user_id: String,
    pub claims: Option<Claims>,
    pub key: Option<String>, // Optional key for signing the token
}

#[derive(Clone, Serialize)]
pub struct GenerateResponse {
    pub token: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_request_creation() {
        let generate_request = GenerateRequest {
            user_id: "user123".to_string(),
            claims: None,
            key: None,
        };
        assert_eq!(generate_request.user_id, "user123");
    }

    #[test]
    fn test_generate_request_clone() {
        let request = GenerateRequest {
            user_id: "user123".to_string(),
            claims: Some(Claims::new(Some("user123".to_string()))),
            key: Some("secret".to_string()),
        };

        let cloned = request.clone();
        assert_eq!(cloned.user_id, request.user_id);
        assert_eq!(cloned.key, request.key);
        assert_eq!(
            cloned.claims.as_ref().and_then(|c| c.sub.clone()),
            Some("user123".to_string())
        );
    }

    #[test]
    fn test_generate_request_deserialize() {
        let json = r#"{"user_id":"user123","claims":null,"key":"my_key"}"#;
        let request: GenerateRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.user_id, "user123");
        assert!(request.claims.is_none());
        assert_eq!(request.key, Some("my_key".to_string()));
    }

    #[test]
    fn test_generate_response_clone() {
        let response = GenerateResponse {
            token: "jwt_token".to_string(),
        };

        let cloned = response.clone();
        assert_eq!(cloned.token, response.token);
    }

    #[test]
    fn test_generate_response_serialize() {
        let response = GenerateResponse {
            token: "jwt_token".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        assert_eq!(serialized, "{\"token\":\"jwt_token\"}");
    }
}
