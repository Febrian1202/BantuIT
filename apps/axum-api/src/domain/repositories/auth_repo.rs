use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;

use crate::{common::AppError, domain::models::User};

#[async_trait]
pub trait AuthRepo: Send + Sync {
    async fn find_user_by_token_hash(
        &self,
        hashed_token: &str,
        threshold: DateTime<Utc>,
    ) -> Result<User, AppError>;

    async fn touch_token(&self, hashed_token: &str) -> Result<(), AppError>;
}

pub type DynAuthRepo = Arc<dyn AuthRepo>;
