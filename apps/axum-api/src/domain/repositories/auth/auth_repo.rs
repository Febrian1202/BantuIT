use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

use crate::{common::AppError, domain::models::User};

#[async_trait]
pub trait AuthRepo: Send + Sync {
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError>;

    async fn clean_personal_access_token(&self, hashed_token: String) -> Result<(), AppError>;

    async fn create_token_and_update_last_login(
        &self,
        user_id: Uuid,
        hashed_token: &str,
    ) -> Result<(), AppError>;

    async fn find_user_by_token_hash(
        &self,
        hashed_token: &str,
        threshold: DateTime<Utc>,
    ) -> Result<User, AppError>;

    async fn touch_token(&self, hashed_token: &str) -> Result<(), AppError>;

    async fn set_last_login_at(&self, user_id: Uuid, time: DateTime<Utc>) -> Result<(), AppError>;
}

pub type DynAuthRepo = Arc<dyn AuthRepo>;
