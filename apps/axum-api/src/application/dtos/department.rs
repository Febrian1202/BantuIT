use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::domain::models::Department;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Department> for DepartmentResponse {
    fn from(value: Department) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateDepartmentRequest {
    #[validate(custom(function = "validate_name_department"))]
    pub name: String,
    #[validate(length(
        max = 500,
        message = "Deskripsi department tidak boleh lebih dari 500 karakter."
    ))]
    pub description: Option<String>,
}

fn validate_name_department(name: &str) -> Result<(), ValidationError> {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        let mut err = ValidationError::new("name");
        err.message = Some("Nama department wajib diisi.".into());
        return Err(err);
    }

    if trimmed.chars().count() > 100 {
        let mut err = ValidationError::new("name");
        err.message = Some("Nama department tidak boleh lebih dari 100 karakter.".into());
        return Err(err);
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateDepartmentRequest {
    #[validate(custom(function = "validate_name_department"))]
    pub name: String,
    #[validate(length(
        max = 500,
        message = "Deskripsi department tidak boleh lebih dari 500 karakter."
    ))]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDepartmentResponse;
