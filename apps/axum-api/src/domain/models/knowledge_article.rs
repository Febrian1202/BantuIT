use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::ArticleStatus;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct KnowledgeArticle {
    pub id: Uuid,
    pub category_id: i32,
    pub author_id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: ArticleStatus,
    pub view_count: i32,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
