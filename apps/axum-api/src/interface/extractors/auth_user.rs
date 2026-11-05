use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{common::AppError, domain::models::User, interface::AppState};

#[derive(Clone)]
pub struct AuthUser(pub User, pub String);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        if let Some(auth) = parts.extensions.get::<AuthUser>() {
            return Ok(auth.clone());
        }
        // Ekstrak header Authorization
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        // Validasi Prefix bearer
        let plain_token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;

        let (user, token_hash) = state.auth_service.verify_token(plain_token).await?;

        Ok(AuthUser(user, token_hash))
    }
}
