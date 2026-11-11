use std::sync::Arc;

use uuid::Uuid;

use crate::{common::AppError, domain::models::Ticket};
use async_trait::async_trait;

#[async_trait]
pub trait TicketRepo: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Ticket, AppError>;
}

pub type DynTicketRepo = Arc<dyn TicketRepo>;
