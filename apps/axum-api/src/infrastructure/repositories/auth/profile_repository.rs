use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::AppError,
    domain::{
        enums::{RoleName, UserStatus},
        models::{EmployeeProfile, User},
        repositories::auth::ProfileRepo,
    },
};

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
        .await?;

        sqlx::query_file!(
            "./src/sql/profile/revoke_token_other_user_id.sql",
            user_id,
            current_token_hash
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn update_profile(
        &self,
        user_id: Uuid,
        updated_full_name: String,
        updated_phone: Option<String>,
    ) -> Result<(User, EmployeeProfile), AppError> {
        let mut tx = self.pool.begin().await?;

        let updated_user = sqlx::query_file_as!(
            User,
            "./src/sql/profile/update_user_full_name.sql",
            updated_full_name,
            user_id
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(AppError::NotFound)?;

        let updated_profile = if let Some(phone) = updated_phone {
            sqlx::query_file_as!(
                EmployeeProfile,
                "./src/sql/profile/update_phone_number.sql",
                phone,
                user_id
            )
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(AppError::NotFound)?
        } else {
            sqlx::query_file_as!(
                EmployeeProfile,
                "./src/sql/profile//get_profile_by_user_id.sql",
                user_id
            )
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(AppError::NotFound)?
        };

        tx.commit().await?;

        Ok((updated_user, updated_profile))
    }

    async fn get_profile_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<EmployeeProfile>, AppError> {
        let profile = sqlx::query_file_as!(
            EmployeeProfile,
            "./src/sql/profile/get_profile_by_user_id.sql",
            user_id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(profile)
    }
}
