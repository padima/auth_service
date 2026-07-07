use crate::model::Claims;

pub struct ValidateResponse {
    pub is_valid: bool,
    pub claims: Option<Claims>,
}
