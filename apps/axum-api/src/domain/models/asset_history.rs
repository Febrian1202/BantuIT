use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::AssetHistoryAction;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AssetHistory {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub action: AssetHistoryAction,
    pub description: Option<String>,
    pub action_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
