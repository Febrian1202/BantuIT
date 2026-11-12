use chrono::Utc;
use serde_json::Value;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    common::AppError,
    domain::{
        enums::{AuditAction, AuditModule},
        models::AuditLog,
        repositories::audit::DynAuditLogRepo,
    },
};

#[derive(Clone)]
pub struct AuditLogger {
    pub audit_repo: DynAuditLogRepo,
}

impl AuditLogger {
    const REDACTED_KEYS: &[&str] = &["password", "remember_token", "token", "secret", "api_token"];

    pub fn new(audit_repo: DynAuditLogRepo) -> Self {
        Self { audit_repo }
    }

    pub async fn log(
        &self,
        conn: &mut PgConnection,
        user_id: Uuid,
        action: AuditAction,
        module: AuditModule,
        module_id: Uuid,
        description: Option<String>,
        old_data: Option<Value>,
        new_data: Option<Value>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<AuditLog, AppError> {
        let audit = AuditLog {
            id: Uuid::now_v7(),
            user_id,
            action,
            module,
            module_id,
            description,
            old_data: old_data.map(Self::redact),
            new_data: new_data.map(Self::redact),
            ip_address,
            user_agent,
            created_at: Utc::now(),
        };

        self.audit_repo.create(conn, &audit).await
    }

    fn redact(mut value: Value) -> Value {
        if let Value::Object(ref mut map) = value {
            for key in Self::REDACTED_KEYS {
                map.remove(*key);
            }
        }
        value
    }
}
