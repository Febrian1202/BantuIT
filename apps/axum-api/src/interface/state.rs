use std::{sync::Arc, time::Instant};

use sqlx::PgPool;

use crate::{
    application::services::{
        auth::{ProfileService, auth_service::AuthService},
        ticket::{TicketCategoryService, ticket_service::TicketService},
    },
    infrastructure::{
        AppConfig,
        repositories::{
            auth::{AuthRepository, ProfileRepository},
            ticket::{TicketCategoryRepository, TicketRepository},
        },
    },
    interface::middleware::rate_limiter::RateLimiterRegistry,
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: PgPool,
    pub started_at: Instant,
    pub ticket_service: TicketService,
    pub auth_service: AuthService,
    pub profile_service: ProfileService,
    pub ticket_category_service: TicketCategoryService,
    pub rate_limit: RateLimiterRegistry,
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
        let auth_service = AuthService::new(auth_repo.clone());

        // Setup ProfileRepo dan ProfileService
        let pg_profile_repo = ProfileRepository::new(pool.clone());
        let profile_repo = Arc::new(pg_profile_repo);
        let profile_service = ProfileService::new(profile_repo, auth_repo.clone());

        // Setup TicketCategoryRepo dan TicketCategoryService
        let pg_category_repo = TicketCategoryRepository::new(pool.clone());
        let category_repo = Arc::new(pg_category_repo);
        let ticket_category_service = TicketCategoryService::new(category_repo);

        // Setup rate-limit
        let rate_limit = RateLimiterRegistry::new(config.env.clone());

        Self {
            config,
            pool,
            started_at,
            ticket_service,
            auth_service,
            profile_service,
            rate_limit,
            ticket_category_service,
        }
    }
}
