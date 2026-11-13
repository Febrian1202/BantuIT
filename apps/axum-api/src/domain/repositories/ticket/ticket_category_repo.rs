use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgConnection;

use crate::{common::AppError, domain::models::TicketCategoryWithParent};

#[async_trait]
pub trait TicketCategoryRepo: Send + Sync {
    async fn get_all_categories(&self) -> Result<Vec<TicketCategoryWithParent>, AppError>;
    async fn get_category_by_id(&self, id: i32) -> Result<TicketCategoryWithParent, AppError>;
    async fn is_category_name_exists(&self, name: &str) -> Result<bool, AppError>;
    async fn is_category_id_exists(&self, id: i32) -> Result<bool, AppError>;
    async fn create_category(
        &self,
        conn: &mut PgConnection,
        name: String,
        desc: Option<String>,
        parent_id: Option<i32>,
    ) -> Result<TicketCategoryWithParent, AppError>;
}

pub type DynTicketCategoryRepo = Arc<dyn TicketCategoryRepo>;
