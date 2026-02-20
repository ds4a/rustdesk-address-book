use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};

use crate::auth::jwt::{validate_token, Claims};
use crate::error::ApiError;
use crate::state::AppState;

/// Extractor that validates the Bearer token and provides user claims.
pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| ApiError::Unauthorized("Missing authorization header".to_string()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| ApiError::Unauthorized("Invalid authorization format".to_string()))?;

        let claims = validate_token(token, &state.config.jwt_secret)
            .map_err(|_| ApiError::Unauthorized("Invalid or expired token".to_string()))?;

        Ok(AuthUser(claims))
    }
}
