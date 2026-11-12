use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    common::AppError,
    domain::models::{EmployeeProfile, User},
};

#[async_trait]
pub trait ProfileRepo: Send + Sync {
    async fn update_password_and_revoke_token(
        &self,
        user_id: Uuid,
        current_token_hash: &str,
        new_password: String,
    ) -> Result<(), AppError>;

    async fn update_profile(
        &self,
        user_id: Uuid,
        updated_full_name: String,
        updated_phone: Option<String>,
    ) -> Result<(User, EmployeeProfile), AppError>;

    async fn get_profile_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<EmployeeProfile>, AppError>;
}

pub type DynProfileRepo = Arc<dyn ProfileRepo>;
