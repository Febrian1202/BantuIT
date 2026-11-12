use std::sync::Arc;

use async_trait::async_trait;

use crate::{common::AppError, domain::models::Department};

#[async_trait]
pub trait DepartmentRepo: Send + Sync {
    async fn get_department_by_department_id(
        &self,
        department_id: i32,
    ) -> Result<Option<Department>, AppError>;
}

pub type DynDepartmentRepo = Arc<dyn DepartmentRepo>;
