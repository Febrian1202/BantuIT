use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    common::AppError,
    domain::{models::Ticket, repositories::TicketRepo},
};

#[derive(Clone)]
pub struct TicketRepository {
    pool: PgPool,
}

impl TicketRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TicketRepo for TicketRepository {
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Ticket>, AppError> {
        let ticket = sqlx::query_file_as!(Ticket, "src/sql/ticket/get_ticket_by_id.sql", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Database error pada saat find_by_id tiket: {:?}", e);
                AppError::Database(e)
            })?;

        Ok(ticket)
    }
}
