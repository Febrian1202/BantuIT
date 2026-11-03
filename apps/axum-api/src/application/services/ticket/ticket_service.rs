use uuid::Uuid;

use crate::{
    common::AppError,
    domain::{models::Ticket, repositories::DynTicketRepo},
};

#[derive(Clone)]
pub struct TicketService {
    repo: DynTicketRepo,
}

impl TicketService {
    pub fn new(repo: DynTicketRepo) -> Self {
        Self { repo }
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Ticket, AppError> {
        self.repo.find_by_id(id).await?.ok_or(AppError::NotFound)
    }
}
