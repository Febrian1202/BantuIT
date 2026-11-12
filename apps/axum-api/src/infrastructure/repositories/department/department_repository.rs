use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    common::AppError,
    domain::{models::Department, repositories::department::DepartmentRepo},
};

pub struct DepartmentRepository {
    pool: PgPool,
}

impl DepartmentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DepartmentRepo for DepartmentRepository {
    async fn get_department_by_department_id(
        &self,
        department_id: i32,
    ) -> Result<Option<Department>, AppError> {
        let department = sqlx::query_file_as!(
            Department,
            "./src/sql/department/get_department_by_department_id.sql",
            department_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(department)
    }
}
