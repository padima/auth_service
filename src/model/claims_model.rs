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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_creation() {
        let claims = Claims::new(Some(" user123".to_string()));
        assert_eq!(claims.sub, Some(" user123".to_string()));
        assert!(claims.iss.is_none());
        assert!(claims.aud.is_none());
        assert!(claims.exp.is_none());
        assert!(claims.iat.is_none());
    }

    #[test]
    fn test_claims_serialization_deserialized() {
        let claims = Claims {
            sub: Some("user123".to_string()),
            iss: Some("issuer".to_string()),
            aud: Some("audience".to_string()),
            exp: Some(1700000000),
            iat: Some(1600000000),
        };

        let serialized = serde_json::to_string(&claims).unwrap();
        let deserialized: Claims = serde_json::from_str(&serialized).unwrap();

        assert_eq!(claims.sub, deserialized.sub);
        assert_eq!(claims.iss, deserialized.iss);
        assert_eq!(claims.aud, deserialized.aud);
        assert_eq!(claims.exp, deserialized.exp);
        assert_eq!(claims.iat, deserialized.iat);
    }

    #[test]
    fn test_claims_clone() {
        let claims = Claims::new(Some("user123".to_string()));
        let cloned_claims = claims.clone();

        assert_eq!(claims.sub, cloned_claims.sub);
        assert_eq!(claims.iss, cloned_claims.iss);
        assert_eq!(claims.aud, cloned_claims.aud);
        assert_eq!(claims.exp, cloned_claims.exp);
        assert_eq!(claims.iat, cloned_claims.iat);
    }
}
