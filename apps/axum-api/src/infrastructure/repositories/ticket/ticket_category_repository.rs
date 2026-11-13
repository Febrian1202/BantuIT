use async_trait::async_trait;
use sqlx::{PgConnection, PgPool, types::Json};

use crate::{
    common::AppError,
    domain::{
        models::{CategoryReferences, ParentCategory, TicketCategoryWithParent},
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

    async fn get_category_by_id(&self, id: i32) -> Result<TicketCategoryWithParent, AppError> {
        let category = sqlx::query_file_as!(
            TicketCategoryWithParent,
            "./src/sql/ticket/get_category_by_id.sql",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(category)
    }

    async fn is_category_name_exists(&self, name: &str) -> Result<bool, AppError> {
        let row = sqlx::query_file_scalar!("./src/sql/ticket/is_category_name_exist.sql", name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.is_some())
    }

    async fn is_category_id_exists(&self, id: i32) -> Result<bool, AppError> {
        let row = sqlx::query_file_scalar!("./src/sql/ticket/is_category_id_exists.sql", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.is_some())
    }

    async fn create_category(
        &self,
        conn: &mut PgConnection,
        name: String,
        desc: Option<String>,
        parent_id: Option<i32>,
    ) -> Result<TicketCategoryWithParent, AppError> {
        let category = sqlx::query_file_as!(
            TicketCategoryWithParent,
            "./src/sql/ticket/create_category.sql",
            name,
            desc,
            parent_id
        )
        .fetch_one(conn)
        .await?;

        Ok(category)
    }

    async fn is_category_name_exists_except(&self, id: i32, name: &str) -> Result<bool, AppError> {
        let exists = sqlx::query_file_scalar!(
            "./src/sql/ticket/is_category_name_exists_except.sql",
            name,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    async fn update_category_by_id(
        &self,
        conn: &mut PgConnection,
        id: i32,
        name: String,
        desc: Option<String>,
        parent_id: Option<i32>,
    ) -> Result<TicketCategoryWithParent, AppError> {
        let updated_category = sqlx::query_file_as!(
            TicketCategoryWithParent,
            "./src/sql/ticket/update_category_by_id.sql",
            name,
            desc,
            parent_id,
            id
        )
        .fetch_one(conn)
        .await?;

        Ok(updated_category)
    }

    async fn get_reference_counts(&self, id: i32) -> Result<CategoryReferences, AppError> {
        let refs = sqlx::query_file_as!(
            CategoryReferences,
            "./src/sql/ticket/check_category_references.sql",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(refs)
    }

    async fn soft_delete(&self, conn: &mut PgConnection, id: i32) -> Result<(), AppError> {
        sqlx::query_file!("./src/sql/ticket/soft_delete_by_id.sql", id)
            .execute(conn)
            .await?;

        Ok(())
    }
}
