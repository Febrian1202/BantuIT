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
    interface::extractors::ClientInfo,
};

#[derive(Debug, Default)]
pub struct AuditEntry {
    pub module_id: String,
    pub description: Option<String>,
    pub old_data: Option<Value>,
    pub new_data: Option<Value>,
}

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
        entry: AuditEntry,
        client: ClientInfo,
    ) -> Result<AuditLog, AppError> {
        let audit = AuditLog {
            id: Uuid::now_v7(),
            user_id,
            action,
            module,
            module_id: entry.module_id,
            description: entry.description,
            old_data: entry.old_data.map(Self::redact),
            new_data: entry.new_data.map(Self::redact),
            ip_address: client.ip_address,
            user_agent: client.user_agent,
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
