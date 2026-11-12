use crate::{
    application::dtos::TicketCategoryResponse,
    common::AppError,
    domain::repositories::{
        audit::DynAuditLogRepo, ticket::ticket_category_repo::DynTicketCategoryRepo,
    },
};

#[derive(Clone)]
pub struct TicketCategoryService {
    category_repo: DynTicketCategoryRepo,
    audit_repo: DynAuditLogRepo,
}

impl TicketCategoryService {
    pub fn new(category_repo: DynTicketCategoryRepo, audit_repo: DynAuditLogRepo) -> Self {
        Self {
            category_repo,
            audit_repo,
        }
    }

    pub async fn get_categories(&self) -> Result<Vec<TicketCategoryResponse>, AppError> {
        let categories = self.category_repo.get_all_categories().await?;

        let response_dtos = categories
            .into_iter()
            .map(TicketCategoryResponse::from)
            .collect();
        Ok(response_dtos)
    }

    pub async fn show_by_id(&self) -> Result<_, AppError> {
        todo!()
    }

    pub async fn create_category(&self) -> Result<_, AppError> {
        todo!()
    }

    pub async fn update_category(&self) -> Result<_, AppError> {
        todo!()
    }

    pub async fn delete_category(&self) -> Result<_, AppError> {
        todo!()
    }
}
