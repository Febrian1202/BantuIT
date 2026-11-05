use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::common::AppError;

#[async_trait]
pub trait ProfileRepo: Send + Sync {
    async fn update_password_and_revoke_token(
        &self,
        user_id: Uuid,
        current_token_hash: &str,
        new_password: String,
    ) -> Result<(), AppError>;
}

pub type DynProfileRepo = Arc<dyn ProfileRepo>;
