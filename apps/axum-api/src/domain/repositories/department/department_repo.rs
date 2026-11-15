use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgConnection;

use crate::{
    common::AppError,
    domain::models::{Department, department::DepartmentReferences},
};

#[async_trait]
pub trait DepartmentRepo: Send + Sync {
    async fn get_departments(&self) -> Result<Vec<Department>, AppError>;
    async fn get_department_by_id(&self, department_id: i32) -> Result<Department, AppError>;
    async fn is_name_exists(&self, name: &str) -> Result<bool, AppError>;
    async fn is_name_exists_except(&self, id: i32, name: &str) -> Result<bool, AppError>;
    async fn create_department(
        &self,
        conn: &mut PgConnection,
        name: String,
        desc: Option<String>,
    ) -> Result<Department, AppError>;
    async fn update_department(
        &self,
        conn: &mut PgConnection,
        id: i32,
        name: String,
        desc: Option<String>,
    ) -> Result<Department, AppError>;
    async fn get_reference_counts(&self, id: i32) -> Result<DepartmentReferences, AppError>;
    async fn soft_delete(&self, conn: &mut PgConnection, id: i32) -> Result<(), AppError>;
}

pub type DynDepartmentRepo = Arc<dyn DepartmentRepo>;
