use async_trait::async_trait;
use sqlx::{PgConnection, PgPool};

use crate::{
    common::AppError,
    domain::{
        models::{Department, department::DepartmentReferences},
        repositories::department::DepartmentRepo,
    },
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
    async fn get_departments(&self) -> Result<Vec<Department>, AppError> {
        let departments =
            sqlx::query_file_as!(Department, "./src/sql/department/get_all_department.sql")
                .fetch_all(&self.pool)
                .await?;

        Ok(departments)
    }

    async fn get_department_by_id(&self, department_id: i32) -> Result<Department, AppError> {
        let department = sqlx::query_file_as!(
            Department,
            "./src/sql/department/get_department_by_department_id.sql",
            department_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(department)
    }

    async fn is_name_exists(&self, name: &str) -> Result<bool, AppError> {
        let exists = sqlx::query_file_scalar!("./src/sql/department/is_name_exists.sql", name)
            .fetch_one(&self.pool)
            .await?;

        Ok(exists)
    }

    async fn is_name_exists_except(&self, id: i32, name: &str) -> Result<bool, AppError> {
        let exists =
            sqlx::query_file_scalar!("./src/sql/department/is_name_exists_except.sql", name, id)
                .fetch_one(&self.pool)
                .await?;

        Ok(exists)
    }

    async fn create_department(
        &self,
        conn: &mut PgConnection,
        name: String,
        desc: Option<String>,
    ) -> Result<Department, AppError> {
        let inserted = sqlx::query_file_as!(
            Department,
            "./src/sql/department/create_department.sql",
            name,
            desc
        )
        .fetch_one(conn)
        .await?;

        Ok(inserted)
    }
    async fn update_department(
        &self,
        conn: &mut PgConnection,
        id: i32,
        name: String,
        desc: Option<String>,
    ) -> Result<Department, AppError> {
        let updated = sqlx::query_file_as!(
            Department,
            "./src/sql/department/update_department.sql",
            name,
            desc,
            id
        )
        .fetch_one(conn)
        .await?;

        Ok(updated)
    }

    async fn get_reference_counts(&self, id: i32) -> Result<DepartmentReferences, AppError> {
        let refs = sqlx::query_file_as!(
            DepartmentReferences,
            "./src/sql/department/check_department_reference.sql",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(refs)
    }

    async fn soft_delete(&self, conn: &mut PgConnection, id: i32) -> Result<(), AppError> {
        sqlx::query_file!("./src/sql/department/soft_delete_by_id.sql", id)
            .execute(conn)
            .await?;

        Ok(())
    }
}
