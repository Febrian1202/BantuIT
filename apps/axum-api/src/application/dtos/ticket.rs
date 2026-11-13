use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::domain::models::TicketCategoryWithParent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentCategoryResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketCategoryResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub parent: Option<ParentCategoryResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TicketCategoryWithParent> for TicketCategoryResponse {
    fn from(model: TicketCategoryWithParent) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            parent_id: model.parent_id,
            parent: model.parent.map(|json_parent| {
                let p = json_parent.0;
                ParentCategoryResponse {
                    id: p.id,
                    name: p.name,
                    description: p.description,
                    parent_id: p.parent_id,
                    created_at: p.created_at,
                    updated_at: p.updated_at,
                }
            }),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTicketCategoryRequest {
    #[validate(custom(function = "validate_name_category"))]
    pub name: String,

    #[validate(length(max = 500, message = "Deskripsi tidak boleh lebih dari 500 karakter."))]
    pub description: Option<String>,

    pub parent_id: Option<i32>,
}

fn validate_name_category(name: &str) -> Result<(), ValidationError> {
    let len = name.chars().count();

    if name.trim().is_empty() {
        let mut err = ValidationError::new("name");
        err.message = Some("Nama kategori wajib diisi.".into());
        return Err(err);
    }

    if len > 100 {
        let mut err = ValidationError::new("name");
        err.message = Some("Nama kategori tidak boleh lebih dari 100 karakter.".into());
        return Err(err);
    }

    Ok(())
}
