use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TicketAttachment {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub uploaded_by: Uuid,
    pub original_filename: String,
    pub stored_filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub storage_path: String,
    pub created_at: DateTime<Utc>,
}
