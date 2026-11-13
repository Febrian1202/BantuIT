use serde_json::json;
use sqlx::PgPool;

use crate::{
    application::{
        dtos::{CreateTicketCategoryRequest, TicketCategoryResponse, UpdateTicketCategoryRequest},
        services::{
            audit::{AuditLogger, audit_logger::AuditEntry},
            guard::ReferentialIntegrityGuard,
        },
    },
    common::AppError,
    domain::{
        enums::{AuditAction, AuditModule},
        models::User,
        repositories::ticket::ticket_category_repo::DynTicketCategoryRepo,
    },
    interface::extractors::ClientInfo,
};

#[derive(Clone)]
pub struct TicketCategoryService {
    pool: PgPool,
    category_repo: DynTicketCategoryRepo,
    audit_logger: AuditLogger,
}

impl TicketCategoryService {
    pub fn new(
        pool: PgPool,
        category_repo: DynTicketCategoryRepo,
        audit_logger: AuditLogger,
    ) -> Self {
        Self {
            pool,
            category_repo,
            audit_logger,
        }
    }

    pub async fn get_categories(&self) -> Result<Vec<TicketCategoryResponse>, AppError> {
        let categories = self.category_repo.get_all_categories().await?;

        let response_dto = categories
            .into_iter()
            .map(TicketCategoryResponse::from)
            .collect();
        Ok(response_dto)
    }

    pub async fn show_by_id(&self, id: i32) -> Result<TicketCategoryResponse, AppError> {
        let category = self.category_repo.get_category_by_id(id).await?;
        let response_dto = TicketCategoryResponse::from(category);
        Ok(response_dto)
    }

    pub async fn create_category(
        &self,
        data: CreateTicketCategoryRequest,
        actor: User,
        client: ClientInfo,
    ) -> Result<TicketCategoryResponse, AppError> {
        if self
            .category_repo
            .is_category_name_exists(&data.name)
            .await?
        {
            return Err(AppError::validation_error(
                "name",
                "Nama kategori tiket sudah digunakan.",
            ));
        }

        if let Some(parent_id) = data.parent_id
            && !self.category_repo.is_category_id_exists(parent_id).await?
        {
            return Err(AppError::validation_error(
                "parent_id",
                "Kategori induk tidak valid.",
            ));
        }

        // Begin transaction
        let mut tx = self.pool.begin().await?;

        let created_category = self
            .category_repo
            .create_category(&mut tx, data.name, data.description, data.parent_id)
            .await?;

        self.audit_logger
            .log(
                &mut tx,
                actor.id,
                AuditAction::Create,
                AuditModule::TicketCategory,
                AuditEntry {
                    module_id: created_category.id.to_string(),
                    description: Some(format!("Kategori tiket {} dibuat.", created_category.name)),
                    old_data: None,
                    new_data: Some(json!({
                        "name": created_category.name,
                        "description": created_category.description,
                        "parent_id": created_category.parent_id
                    })),
                },
                client,
            )
            .await?;
        // Commit transaction
        tx.commit().await?;

        Ok(TicketCategoryResponse::from(created_category))
    }

    pub async fn update_category(
        &self,
        id: i32,
        data: UpdateTicketCategoryRequest,
        actor: User,
        client: ClientInfo,
    ) -> Result<TicketCategoryResponse, AppError> {
        let existing = self.category_repo.get_category_by_id(id).await?;

        if self
            .category_repo
            .is_category_name_exists_except(id, &data.name)
            .await?
        {
            return Err(AppError::validation_error(
                "name",
                "Nama kategori tiket sudah digunakan.",
            ));
        }

        if let Some(parent_id) = data.parent_id {
            if parent_id == id {
                return Err(AppError::validation_error(
                    "parent_id",
                    "Kategori tidak dapat menjadi induk bagi dirinya sendiri.",
                ));
            }

            if !self.category_repo.is_category_id_exists(parent_id).await? {
                return Err(AppError::validation_error(
                    "parent_id",
                    "Kategori induk tidak valid.",
                ));
            }
        }

        // Begin transaction
        let mut tx = self.pool.begin().await?;

        let updated_category = self
            .category_repo
            .update_category_by_id(&mut tx, id, data.name, data.description, data.parent_id)
            .await?;

        self.audit_logger
            .log(
                &mut tx,
                actor.id,
                AuditAction::Update,
                AuditModule::TicketCategory,
                AuditEntry {
                    module_id: id.to_string(),
                    description: Some(format!(
                        "Kategori tiket {} diperbarui",
                        updated_category.name
                    )),
                    old_data: Some(json!({
                        "name": existing.name,
                        "description": existing.description,
                        "parent_id": existing.parent_id,
                    })),
                    new_data: Some(json!({
                        "name": updated_category.name,
                        "description": updated_category.description,
                        "parent_id": updated_category.parent_id,
                    })),
                },
                client,
            )
            .await?;

        // Commit transaction
        tx.commit().await?;

        Ok(TicketCategoryResponse::from(updated_category))
    }

    pub async fn delete_category(
        &self,
        id: i32,
        actor: User,
        client: ClientInfo,
    ) -> Result<(), AppError> {
        let category = self.category_repo.get_category_by_id(id).await?;

        let refs = self.category_repo.get_reference_counts(id).await?;

        ReferentialIntegrityGuard::assert_unreferenced(&[
            ("Kategori tiket", refs.ticket_count),
            ("Kategori tiket (induk)", refs.child_count),
        ])?;

        let mut tx = self.pool.begin().await?;

        self.category_repo.soft_delete(&mut tx, id).await?;

        self.audit_logger
            .log(
                &mut tx,
                actor.id,
                AuditAction::Delete,
                AuditModule::TicketCategory,
                AuditEntry {
                    module_id: id.to_string(),
                    description: Some(format!("Kategori tiket {} dihapus.", category.name)),
                    old_data: None,
                    new_data: None,
                },
                client,
            )
            .await?;

        tx.commit().await?;

        Ok(())
    }
}
