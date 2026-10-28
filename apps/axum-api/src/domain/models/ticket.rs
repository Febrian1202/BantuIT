use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ticket {
    pub id: Uuid,
    pub ticket_number: String,
    pub title: String,
    pub description: String,
    pub category_id: i32,
    pub priority_id: i32,
    pub status_id: i32,
    pub reporter_id: Uuid,
    pub technician_id: Option<Uuid>,
    pub department_id: Option<i32>,
    pub asset_id: Option<Uuid>,
    pub sla_duration_minutes: i32,
    pub sla_deadline: Option<DateTime<Utc>>,
    pub sla_breached: bool,
    pub sla_breached_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
