use async_trait::async_trait;
use sqlx::{PgConnection, PgPool};

use crate::{
    common::AppError,
    domain::{
        enums::{AuditAction, AuditModule},
        models::AuditLog,
        repositories::audit::AuditLogRepo,
    },
};

pub struct AuditLogRepository {
    pub pool: PgPool,
}

impl AuditLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditLogRepo for AuditLogRepository {
    async fn create(&self, conn: &mut PgConnection, log: &AuditLog) -> Result<AuditLog, AppError> {
        let action_str: &'static str = log.action.into();
        let module_str: &'static str = log.module.into();

        let inserted = sqlx::query_file_as!(
            AuditLog,
            "./src/sql/audit/create_audit_log.sql",
            log.id,
            log.user_id,
            action_str,
            module_str,
            log.module_id,
            log.description,
            log.old_data,
            log.new_data,
            log.ip_address,
            log.user_agent,
            log.created_at
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(inserted)
    }
}
