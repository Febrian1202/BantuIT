use std::{sync::Arc, time::Instant};

use sqlx::PgPool;

use crate::{
    application::services::ticket::ticket_service::TicketService,
    infrastructure::{AppConfig, repositories::TicketRepository},
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: PgPool,
    pub started_at: Instant,
    pub ticket_service: TicketService,
}

impl AppState {
    pub fn new(config: AppConfig, pool: PgPool, started_at: Instant) -> Self {
        // Setup TicketRepo dan TicketService
        let pg_ticket_repo = TicketRepository::new(pool.clone());
        let ticket_repo = Arc::new(pg_ticket_repo);
        let ticket_service = TicketService::new(ticket_repo);
        Self {
            config,
            pool,
            started_at,
            ticket_service,
        }
    }
}
