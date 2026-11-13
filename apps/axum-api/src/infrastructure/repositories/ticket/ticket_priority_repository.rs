use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::repositories::ticket::TicketPriorityRepo;

pub struct TicketPriorityRepository {
    pub pool: PgPool,
}

impl TicketPriorityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TicketPriorityRepo for TicketPriorityRepository {}
