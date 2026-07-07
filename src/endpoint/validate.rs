use axum::{ extract::{ Json, State }, http::{ StatusCode } };
use jsonwebtoken::{ Algorithm, Validation, decode };
use crate::model::{ ValidateRequest, ValidateResponse, AppState, Claims };

pub async fn validate(
    State(state): State<AppState>,
    Json(body): Json<ValidateRequest>
) -> Result<Json<ValidateResponse>, StatusCode> {
    let key = jsonwebtoken::DecodingKey::from_secret(state.key.as_ref());

    match decode::<Claims>(&body.token, &key, &Validation::new(Algorithm::HS256)) {
        Ok(data) => Ok(Json(ValidateResponse { is_valid: true, claims: Some(data.claims) })),
        Err(_) => Ok(Json(ValidateResponse { is_valid: false, claims: Some(Claims::clear()) })),
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::EncodingKey;
    use super::*;

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
        let app_state = AppState { key: "my_secret_key".to_string() };
        let key = EncodingKey::from_secret(app_state.key.as_ref());
        let token = jsonwebtoken::encode(&header, &claims, &key).expect("Failed to encode token");

        println!("Generated token: {}", token);

        let request = ValidateRequest { token };
        let result = validate(State(app_state), Json(request)).await;

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
        let app_state = AppState { key: "my_secret_key".to_string() };
        let key = EncodingKey::from_secret(app_state.key.as_ref());
        let token = jsonwebtoken::encode(&header, &claims, &key).expect("Failed to encode token");
        let request = ValidateRequest { token };
        let app_state = AppState { key: "my_secret_key1".to_string() };
        let result = validate(State(app_state), Json(request)).await;

        println!("Validation result: {:?}", result.as_ref().unwrap().is_valid);
        println!("Claims: {:?}", result.as_ref().unwrap().claims);

        assert!(!result.as_ref().unwrap().is_valid);
    }
}
