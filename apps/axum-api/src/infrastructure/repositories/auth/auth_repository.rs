use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::AppError,
    domain::{
        enums::{RoleName, UserStatus},
        models::User,
        repositories::auth::AuthRepo,
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
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_file_as!(User, "./src/sql/auth/find_user_by_email.sql", email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    async fn clean_personal_access_token(&self, hashed_token: String) -> Result<(), AppError> {
        sqlx::query_file!(
            "./src/sql/token/clean_personal_access_token.sql",
            hashed_token
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn create_token_and_update_last_login(
        &self,
        user_id: Uuid,
        hashed_token: &str,
    ) -> Result<(), AppError> {
        let _ = sqlx::query_file!(
            "./src/sql/auth/create_token_and_update_last_login.sql",
            user_id,
            hashed_token
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_user_by_token_hash(
        &self,
        hashed_token: &str,
        threshold: DateTime<Utc>,
    ) -> Result<User, AppError> {
        let user = sqlx::query_file_as!(
            User,
            "./src/sql/auth/get_authenticated_user_by_token.sql",
            hashed_token,
            threshold,
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::Unauthorized)?;

        Ok(user)
    }

    async fn touch_token(&self, hashed_token: &str) -> Result<(), AppError> {
        sqlx::query_file!("./src/sql/auth/set_last_used_token.sql", hashed_token)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn set_last_login_at(&self, user_id: Uuid, time: DateTime<Utc>) -> Result<(), AppError> {
        sqlx::query_file!("./src/sql/auth/set_last_login_at.sql", time, user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
