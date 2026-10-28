use axum::{extract::FromRequestParts, http::request::Parts};
use chrono::Utc;
use sha2::{Digest, Sha256};

use crate::{
    common::AppError,
    domain::{
        enums::{RoleName, UserStatus},
        models::User,
    },
    interface::AppState,
};

pub struct AuthUser(pub User);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
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

        // Hash token dengan SHA-256
        let mut hasher = Sha256::new();
        hasher.update(plain_token.as_bytes());
        let hashed_token = hex::encode(hasher.finalize());

        // Batas waktu kedaluwarsa (12 jam yang lalu)
        let expired_threshold = Utc::now() - chrono::Duration::hours(12);

        // Query sekaligus (1 Round trip)
        let auth_user = sqlx::query_file_as!(
            User,
            "src/sql/auth/get_authenticated_user_by_token.sql",
            hashed_token,
            expired_threshold
        )
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database query user error: {}", e);
            AppError::Unauthorized
        })?
        .ok_or(AppError::Unauthorized)?;

        // Update last_used_at secara asinkron tanpa memblokir response
        let pool = state.pool.clone();
        tokio::spawn(async move {
            let _ = sqlx::query_file!("src/sql/auth/set_last_used_token.sql", hashed_token)
                .execute(&pool)
                .await;
        });

        Ok(AuthUser(auth_user))
    }
}
