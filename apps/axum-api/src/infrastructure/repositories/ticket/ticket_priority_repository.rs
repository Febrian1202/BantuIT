use async_trait::async_trait;
use sqlx::{PgConnection, PgPool};

use crate::{
    common::AppError,
    domain::{
        models::{TicketPriority, ticket_priority::PriorityReference},
        repositories::ticket::TicketPriorityRepo,
    },
};

pub struct TicketPriorityRepository {
    pub pool: PgPool,
}

impl TicketPriorityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TicketPriorityRepo for TicketPriorityRepository {
    async fn get_all_priorities(&self) -> Result<Vec<TicketPriority>, AppError> {
        let priorities =
            sqlx::query_file_as!(TicketPriority, "./src/sql/ticket/get_all_priority.sql")
                .fetch_all(&self.pool)
                .await?;

        Ok(priorities)
    }

    async fn get_priority_by_id(&self, id: i32) -> Result<TicketPriority, AppError> {
        let priority = sqlx::query_file_as!(
            TicketPriority,
            "./src/sql/ticket/get_priority_by_id.sql",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(priority)
    }

    async fn is_name_exists(&self, name: &str) -> Result<bool, AppError> {
        let row = sqlx::query_file_scalar!("./src/sql/ticket/is_priority_name_exists.sql", name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.is_some())
    }

    async fn is_level_exists(&self, level: &i32) -> Result<bool, AppError> {
        let row = sqlx::query_file_scalar!("./src/sql/ticket/is_priority_level_exists.sql", level)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.is_some())
    }

    async fn create_priority(
        &self,
        conn: &mut PgConnection,
        name: String,
        level: i32,
        sla_minutes: i32,
        description: Option<String>,
    ) -> Result<TicketPriority, AppError> {
        let inserted = sqlx::query_file_as!(
            TicketPriority,
            "./src/sql/ticket/create_priority.sql",
            name,
            level,
            sla_minutes,
            description
        )
        .fetch_one(conn)
        .await?;

        Ok(inserted)
    }

    async fn is_name_exists_except(&self, name: &str, id: &i32) -> Result<bool, AppError> {
        let exists = sqlx::query_file_scalar!(
            "./src/sql/ticket/is_priority_name_exists_except.sql",
            name,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    async fn is_level_exists_except(&self, level: &i32, id: &i32) -> Result<bool, AppError> {
        let exists = sqlx::query_file_scalar!(
            "./src/sql/ticket/is_priority_level_exists_except.sql",
            level,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    async fn update_priority(
        &self,
        conn: &mut PgConnection,
        id: i32,
        name: String,
        level: i32,
        sla_minutes: i32,
        description: Option<String>,
    ) -> Result<TicketPriority, AppError> {
        let updated = sqlx::query_file_as!(
            TicketPriority,
            "./src/sql/ticket/update_priority_by_id.sql",
            name,
            level,
            sla_minutes,
            description,
            id
        )
        .fetch_one(conn)
        .await?;

        Ok(updated)
    }

    async fn get_reference_counts(&self, id: i32) -> Result<PriorityReference, AppError> {
        let refs = sqlx::query_file_as!(
            PriorityReference,
            "./src/sql/ticket/check_priority_references.sql",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(refs)
    }

    async fn soft_delete(&self, conn: &mut PgConnection, id: i32) -> Result<(), AppError> {
        sqlx::query_file!("./src/sql/ticket/soft_delete_priority_by_id.sql", id)
            .execute(conn)
            .await?;

        Ok(())
    }
}
