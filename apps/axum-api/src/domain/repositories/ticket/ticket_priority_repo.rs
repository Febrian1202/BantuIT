use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgConnection;

use crate::{
    common::AppError,
    domain::models::{TicketPriority, ticket_priority::PriorityReference},
};

#[async_trait]
pub trait TicketPriorityRepo: Send + Sync {
    async fn get_all_priorities(&self) -> Result<Vec<TicketPriority>, AppError>;
    async fn get_priority_by_id(&self, id: i32) -> Result<TicketPriority, AppError>;
    async fn is_name_exists(&self, name: &str) -> Result<bool, AppError>;
    async fn is_level_exists(&self, level: &i32) -> Result<bool, AppError>;
    async fn create_priority(
        &self,
        conn: &mut PgConnection,
        name: String,
        level: i32,
        sla_minutes: i32,
        description: Option<String>,
    ) -> Result<TicketPriority, AppError>;
    async fn is_name_exists_except(&self, name: &str, id: &i32) -> Result<bool, AppError>;
    async fn is_level_exists_except(&self, level: &i32, id: &i32) -> Result<bool, AppError>;
    async fn update_priority(
        &self,
        conn: &mut PgConnection,
        id: i32,
        name: String,
        level: i32,
        sla_minutes: i32,
        description: Option<String>,
    ) -> Result<TicketPriority, AppError>;
    async fn get_reference_counts(&self, id: i32) -> Result<PriorityReference, AppError>;
    async fn soft_delete(&self, conn: &mut PgConnection, id: i32) -> Result<(), AppError>;
}

pub type DynTicketPriorityRepo = Arc<dyn TicketPriorityRepo>;
