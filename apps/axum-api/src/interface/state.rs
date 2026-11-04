use std::{sync::Arc, time::Instant};

use sqlx::PgPool;

use crate::{
    application::services::{
        auth::auth_service::AuthService, ticket::ticket_service::TicketService,
    },
    infrastructure::{
        AppConfig,
        repositories::{AuthRepository, TicketRepository},
    },
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: PgPool,
    pub started_at: Instant,
    pub ticket_service: TicketService,
    pub auth_service: AuthService,
}

impl AppState {
    pub fn new(config: AppConfig, pool: PgPool, started_at: Instant) -> Self {
        // Setup TicketRepo dan TicketService
        let pg_ticket_repo = TicketRepository::new(pool.clone());
        let ticket_repo = Arc::new(pg_ticket_repo);
        let ticket_service = TicketService::new(ticket_repo);

        // Setup AuthRepo dan AuthService
        let pg_auth_repo = AuthRepository::new(pool.clone());
        let auth_repo = Arc::new(pg_auth_repo);
        let auth_service = AuthService::new(auth_repo);

        Self {
            config,
            pool,
            started_at,
            ticket_service,
            auth_service,
        }
    }
}
