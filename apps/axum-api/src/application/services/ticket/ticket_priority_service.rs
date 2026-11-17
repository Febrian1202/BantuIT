use serde_json::json;
use sqlx::PgPool;

use crate::{
    application::{
        dtos::{CreateTicketPriorityRequest, UpdateTicketPriorityRequest},
        services::{
            audit::{AuditEntry, AuditLogger},
            guard::ReferentialIntegrityGuard,
        },
    },
    common::AppError,
    domain::{
        enums::{AuditAction, AuditModule},
        models::{TicketPriority, User},
        repositories::ticket::DynTicketPriorityRepo,
    },
    interface::extractors::ClientInfo,
};

#[derive(Clone)]
pub struct TicketPriorityService {
    pool: PgPool,
    priority_repo: DynTicketPriorityRepo,
    audit_logger: AuditLogger,
}

impl TicketPriorityService {
    pub fn new(
        pool: PgPool,
        priority_repo: DynTicketPriorityRepo,
        audit_logger: AuditLogger,
    ) -> Self {
        Self {
            pool,
            priority_repo,
            audit_logger,
        }
    }

    pub async fn get_priorities(&self) -> Result<Vec<TicketPriority>, AppError> {
        let priorities = self.priority_repo.get_all_priorities().await?;
        Ok(priorities)
    }

    pub async fn show_priority(&self, id: i32) -> Result<TicketPriority, AppError> {
        let priority = self.priority_repo.get_priority_by_id(id).await?;
        Ok(priority)
    }

    pub async fn create_priority(
        &self,
        data: CreateTicketPriorityRequest,
        actor: User,
        client: ClientInfo,
    ) -> Result<TicketPriority, AppError> {
        if self.priority_repo.is_name_exists(&data.name).await? {
            return Err(AppError::validation_error(
                "name",
                "Nama prioritas tiket sudah digunakan.",
            ));
        }

        if self.priority_repo.is_level_exists(&data.level).await? {
            return Err(AppError::validation_error(
                "level",
                "Level prioritas sudah digunakan.",
            ));
        }

        let mut tx = self.pool.begin().await?;

        let priority = self
            .priority_repo
            .create_priority(
                &mut tx,
                data.name,
                data.level,
                data.sla_minutes,
                data.description,
            )
            .await?;

        let audit_entry = AuditEntry {
            module_id: priority.id.to_string(),
            description: Some(format!("Prioritas tiket {} dibuat.", priority.name)),
            old_data: None,
            new_data: Some(json!({
                "name": priority.name,
                "level": priority.level,
                "sla_minutes": priority.sla_minutes,
                "description": priority.description,
            })),
        };

        self.audit_logger
            .log(
                &mut tx,
                actor.id,
                AuditAction::Create,
                AuditModule::TicketPriority,
                audit_entry,
                client,
            )
            .await?;

        tx.commit().await?;
        Ok(priority)
    }

    pub async fn update_priority(
        &self,
        id: i32,
        data: UpdateTicketPriorityRequest,
        actor: User,
        client: ClientInfo,
    ) -> Result<TicketPriority, AppError> {
        let existing = self.priority_repo.get_priority_by_id(id).await?;

        if self
            .priority_repo
            .is_name_exists_except(&data.name, &id)
            .await?
        {
            return Err(AppError::validation_error(
                "name",
                "Nama prioritas tiket sudah digunakan.",
            ));
        }

        if self
            .priority_repo
            .is_level_exists_except(&data.level, &id)
            .await?
        {
            return Err(AppError::validation_error(
                "level",
                "Level prioritas sudah digunakan.",
            ));
        }

        let mut tx = self.pool.begin().await?;

        let updated_priority = self
            .priority_repo
            .update_priority(
                &mut tx,
                id,
                data.name,
                data.level,
                data.sla_minutes,
                data.description,
            )
            .await?;

        let audit_entry = AuditEntry {
            module_id: updated_priority.id.to_string(),
            description: Some(format!(
                "Prioritas tiket {} diperbarui.",
                updated_priority.name
            )),
            old_data: Some(json!({
                "name": existing.name,
                "level": existing.level,
                "sla_minutes": existing.sla_minutes,
                "description": existing.description,
            })),
            new_data: Some(json!({
                "name": updated_priority.name,
                "level": updated_priority.level,
                "sla_minutes": updated_priority.sla_minutes,
                "description": updated_priority.description,
            })),
        };

        self.audit_logger
            .log(
                &mut tx,
                actor.id,
                AuditAction::Update,
                AuditModule::TicketPriority,
                audit_entry,
                client,
            )
            .await?;

        tx.commit().await?;
        Ok(updated_priority)
    }

    pub async fn delete_priority(
        &self,
        id: i32,
        actor: User,
        client: ClientInfo,
    ) -> Result<(), AppError> {
        let priority = self.priority_repo.get_priority_by_id(id).await?;

        let refs = self.priority_repo.get_reference_counts(id).await?;

        ReferentialIntegrityGuard::assert_unreferenced(&[("Prioritas tiket", refs.ticket_count)])?;

        let mut tx = self.pool.begin().await?;

        self.priority_repo.soft_delete(&mut tx, id).await?;

        let audit_entry = AuditEntry {
            module_id: priority.id.to_string(),
            description: Some(format!("Prioritas tiket {} dihapus.", priority.name)),
            old_data: None,
            new_data: None,
        };

        self.audit_logger
            .log(
                &mut tx,
                actor.id,
                AuditAction::Delete,
                AuditModule::TicketPriority,
                audit_entry,
                client,
            )
            .await?;

        tx.commit().await?;

        Ok(())
    }
}
