use serde_json::json;
use sqlx::PgPool;

use crate::{
    application::{
        dtos::{CreateTicketCategoryRequest, TicketCategoryResponse},
        services::audit::{AuditLogger, audit_logger::AuditEntry},
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

        if let Some(parent_id) = data.parent_id {
            if !self.category_repo.is_category_id_exists(parent_id).await? {
                return Err(AppError::validation_error(
                    "parent_id",
                    "Kategoori induk tidak valid.",
                ));
            }
        }

        // Begin transaction
        let mut tx = self.pool.begin().await?;

        let created_category = self
            .category_repo
            .create_category(&mut *tx, data.name, data.description, data.parent_id)
            .await?;

        self.audit_logger
            .log(
                &mut *tx,
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
        tx.commit().await?;

        Ok(TicketCategoryResponse::from(created_category))
    }

    pub async fn update_category(&self) -> Result<TicketCategoryResponse, AppError> {
        todo!()
    }

    pub async fn delete_category(&self) -> Result<(), AppError> {
        todo!()
    }
}
