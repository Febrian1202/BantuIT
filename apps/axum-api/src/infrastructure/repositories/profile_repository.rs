use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{common::AppError, domain::repositories::ProfileRepo};

pub struct ProfileRepository {
    pool: PgPool,
}

impl ProfileRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProfileRepo for ProfileRepository {
    async fn update_password_and_revoke_token(
        &self,
        user_id: Uuid,
        current_token_hash: &str,
        new_password: String,
    ) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query_file!(
            "./src/sql/profile/update_user_password.sql",
            new_password,
            user_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Gagal update_user_password: {}", e);
            AppError::Database(e)
        })?;

        sqlx::query_file!(
            "./src/sql/profile/revoke_token_other_user_id.sql",
            user_id,
            current_token_hash
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Gagal revoke_token_other_user_id: {}", e);
            AppError::Database(e)
        })?;

        tx.commit().await?;

        Ok(())
    }
}
