use axum::{ extract::{ Json, State }, http::{ StatusCode } };
use jsonwebtoken::EncodingKey;
use crate::model::{ AppState, AuthRequest, AuthResponse, Claims };
use crate::constants;

pub async fn generate_post(
    State(state): State<AppState>,
    Json(body): Json<AuthRequest>
) -> Result<Json<AuthResponse>, StatusCode> {
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);

    let claims = Claims {
        sub: body.claims
            .as_ref()
            .and_then(|c| c.sub.clone())
            .or(Some(body.user_id)),
        exp: Some((chrono::Utc::now().timestamp() as usize) + constants::TOKEN_EXPIRATION), // Token expires in 1 hour
        iat: Some(chrono::Utc::now().timestamp() as usize),
        iss: body.claims.as_ref().and_then(|c| c.iss.clone()),
        aud: body.claims.as_ref().and_then(|c| c.aud.clone()),
    };

    let key = EncodingKey::from_secret(state.key.as_ref());

    let token = jsonwebtoken
        ::encode(&header, &claims, &key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse { token }))
}
