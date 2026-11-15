use serde_json::json;
use sqlx::PgPool;

use crate::{
    application::{
        dtos::{CreateDepartmentRequest, DepartmentResponse, UpdateDepartmentRequest},
        services::{
            audit::{AuditEntry, AuditLogger},
            guard::ReferentialIntegrityGuard,
        },
    },
    common::AppError,
    domain::{
        enums::{AuditAction, AuditModule},
        models::User,
        repositories::department::DynDepartmentRepo,
    },
    interface::extractors::ClientInfo,
};

#[derive(Clone)]
pub struct DepartmentService {
    pool: PgPool,
    department_repo: DynDepartmentRepo,
    audit_logger: AuditLogger,
}

impl DepartmentService {
    pub fn new(
        pool: PgPool,
        department_repo: DynDepartmentRepo,
        audit_logger: AuditLogger,
    ) -> Self {
        Self {
            pool,
            department_repo,
            audit_logger,
        }
    }

    pub async fn get_departments(&self) -> Result<Vec<DepartmentResponse>, AppError> {
        let departments = self.department_repo.get_departments().await?;
        let response_dtos = departments
            .into_iter()
            .map(DepartmentResponse::from)
            .collect();
        Ok(response_dtos)
    }

    pub async fn show_department(&self, id: i32) -> Result<DepartmentResponse, AppError> {
        let department = self.department_repo.get_department_by_id(id).await?;
        Ok(DepartmentResponse::from(department))
    }

    pub async fn create_department(
        &self,
        data: CreateDepartmentRequest,
        actor: User,
        client: ClientInfo,
    ) -> Result<DepartmentResponse, AppError> {
        if self.department_repo.is_name_exists(&data.name).await? {
            return Err(AppError::validation_error(
                "name",
                "Nama departemen sudah digunakan.",
            ));
        }

        let mut tx = self.pool.begin().await?;

        let department = self
            .department_repo
            .create_department(&mut tx, data.name, data.description)
            .await?;

        let audit_entry = AuditEntry {
            module_id: department.id.to_string(),
            description: Some(format!("Department {} dibuat.", department.name)),
            old_data: None,
            new_data: Some(json!({
                "name": department.name,
                "description": department.description,
            })),
        };

        self.audit_logger
            .log(
                &mut tx,
                actor.id,
                AuditAction::Create,
                AuditModule::Department,
                audit_entry,
                client,
            )
            .await?;

        tx.commit().await?;

        Ok(DepartmentResponse::from(department))
    }

    pub async fn update_department(
        &self,
        id: i32,
        data: UpdateDepartmentRequest,
        actor: User,
        client: ClientInfo,
    ) -> Result<DepartmentResponse, AppError> {
        let exist = self.department_repo.get_department_by_id(id).await?;

        if self
            .department_repo
            .is_name_exists_except(id, &data.name)
            .await?
        {
            return Err(AppError::validation_error(
                "name",
                "Nama departemen sudah digunakan.",
            ));
        }

        let mut tx = self.pool.begin().await?;

        let updated_department = self
            .department_repo
            .update_department(&mut tx, id, data.name, data.description)
            .await?;

        let audit_entry = AuditEntry {
            module_id: id.to_string(),
            description: Some(format!(
                "Department {} diperbarui.",
                updated_department.name
            )),
            old_data: Some(json!({
                "name": exist.name,
                "description": exist.description
            })),
            new_data: Some(json!({
                "name": updated_department.name,
                "description": updated_department.description
            })),
        };

        self.audit_logger
            .log(
                &mut tx,
                actor.id,
                AuditAction::Update,
                AuditModule::Department,
                audit_entry,
                client,
            )
            .await?;

        tx.commit().await?;
        Ok(DepartmentResponse::from(updated_department))
    }

    pub async fn delete_department(
        &self,
        id: i32,
        actor: User,
        client: ClientInfo,
    ) -> Result<(), AppError> {
        let department = self.department_repo.get_department_by_id(id).await?;

        let refs = self.department_repo.get_reference_counts(id).await?;

        ReferentialIntegrityGuard::assert_unreferenced(&[
            ("Departemen", refs.user_count),
            ("Departemen", refs.ticket_count),
        ])?;

        let mut tx = self.pool.begin().await?;

        self.department_repo.soft_delete(&mut tx, id).await?;

        let entry = AuditEntry {
            module_id: department.id.to_string(),
            description: Some(format!("Department {} dihapus.", department.name)),
            new_data: None,
            old_data: None,
        };

        self.audit_logger
            .log(
                &mut tx,
                actor.id,
                AuditAction::Delete,
                AuditModule::Department,
                entry,
                client,
            )
            .await?;

        tx.commit().await?;

        Ok(())
    }
}
