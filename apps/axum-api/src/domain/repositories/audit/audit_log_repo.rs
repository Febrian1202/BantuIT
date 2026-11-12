use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgConnection;

use crate::{common::AppError, domain::models::AuditLog};

#[async_trait]
pub trait AuditLogRepo: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, log: &AuditLog) -> Result<AuditLog, AppError>;
}

pub type DynAuditLogRepo = Arc<dyn AuditLogRepo>;
