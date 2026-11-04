use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::{
    common::AppError,
    domain::{
        enums::{RoleName, UserStatus},
        models::User,
        repositories::AuthRepo,
    },
};

pub struct AuthRepository {
    pool: PgPool,
}

impl AuthRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthRepo for AuthRepository {
    async fn find_user_by_token_hash(
        &self,
        hashed_token: &str,
        threshold: DateTime<Utc>,
    ) -> Result<User, AppError> {
        let user = sqlx::query_file_as!(
            User,
            "src/sql/auth/get_authenticated_user_by_token.sql",
            hashed_token,
            threshold,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database query user error: {}", e);
            AppError::Unauthorized
        })?
        .ok_or(AppError::Unauthorized)?;

        Ok(user)
    }

    async fn touch_token(&self, hashed_token: &str) -> Result<(), AppError> {
        let _ = sqlx::query_file!("src/sql/auth/set_last_used_token.sql", hashed_token)
            .execute(&self.pool)
            .await;

        Ok(())
    }
}
