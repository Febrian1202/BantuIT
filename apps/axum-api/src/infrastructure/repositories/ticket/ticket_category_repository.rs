use async_trait::async_trait;
use sqlx::{PgPool, types::Json};

use crate::{
    common::AppError,
    domain::{
        models::{ParentCategory, TicketCategoryWithParent},
        repositories::ticket::TicketCategoryRepo,
    },
};

pub struct TicketCategoryRepository {
    pool: PgPool,
}

impl TicketCategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl TicketCategoryRepo for TicketCategoryRepository {
    async fn get_all_categories(&self) -> Result<Vec<TicketCategoryWithParent>, AppError> {
        let categories = sqlx::query_file_as!(
            TicketCategoryWithParent,
            "./src/sql/ticket/get_all_category.sql"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }
}
