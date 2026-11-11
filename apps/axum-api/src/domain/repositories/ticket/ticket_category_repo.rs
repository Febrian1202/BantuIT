use std::sync::Arc;

use async_trait::async_trait;

use crate::{common::AppError, domain::models::TicketCategoryWithParent};

#[async_trait]
pub trait TicketCategoryRepo: Send + Sync {
    async fn get_all_categories(&self) -> Result<Vec<TicketCategoryWithParent>, AppError>;
}

pub type DynTicketCategoryRepo = Arc<dyn TicketCategoryRepo>;
