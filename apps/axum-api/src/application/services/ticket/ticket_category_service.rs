use crate::{
    application::dtos::TicketCategoryResponse, common::AppError,
    domain::repositories::ticket::ticket_category_repo::DynTicketCategoryRepo,
};

#[derive(Clone)]
pub struct TicketCategoryService {
    category_repo: DynTicketCategoryRepo,
}

impl TicketCategoryService {
    pub fn new(category_repo: DynTicketCategoryRepo) -> Self {
        Self { category_repo }
    }

    pub async fn index(&self) -> Result<Vec<TicketCategoryResponse>, AppError> {
        let categories = self.category_repo.get_all_categories().await?;

        let response_dtos = categories
            .into_iter()
            .map(TicketCategoryResponse::from)
            .collect();
        Ok(response_dtos)
    }
}
