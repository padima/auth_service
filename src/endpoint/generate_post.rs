use crate::constants;
use crate::model::{AppState, Claims};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use jsonwebtoken::EncodingKey;
use serde::{Deserialize, Serialize};

pub async fn generate_post(
    State(state): State<AppState>,
    Json(body): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, StatusCode> {
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
    let now = chrono::Utc::now().timestamp() as usize;
    let mut claims = body
        .claims
        .clone()
        .unwrap_or_else(|| Claims::new(Some(body.user_id.clone())));

    if claims.sub.is_none() {
        claims.sub = Some(body.user_id);
    }

    if claims.iat.is_none() {
        claims.iat = Some(now);
    }

    if claims.exp.is_none() {
        claims.exp = Some(now + constants::TOKEN_EXPIRATION);
    }

    let key = EncodingKey::from_secret(body.key.as_ref().unwrap_or_else(|| &state.key).as_ref());

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
}
