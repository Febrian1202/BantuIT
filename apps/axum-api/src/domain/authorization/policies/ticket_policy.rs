use crate::{
    common::AppError,
    domain::{
        enums::{RoleName, TicketStatusName},
        models::{Ticket, User},
    },
};

pub struct TicketPolicy;

impl TicketPolicy {
    pub fn view_any(_user: &User) -> Result<(), AppError> {
        Ok(())
    }

    pub fn view(user: &User, ticket: &Ticket) -> Result<(), AppError> {
        if user.role_name.is_admin()
            || user
                .role_name
                .has_role(&[RoleName::Manager, RoleName::Technician])
        {
            return Ok(());
        }

        if ticket.reporter_id == user.id {
            return Ok(());
        }

        Err(AppError::NotFound)
    }

    pub fn create(_user: &User) -> Result<(), AppError> {
        Ok(())
    }

    pub fn update(user: &User, ticket: &Ticket) -> Result<(), AppError> {
        let status = TicketStatusName::from_id(ticket.status_id);
        if status?.is_final() {
            return Err(AppError::Forbidden);
        }

        if user.role_name.is_admin()
            || user
                .role_name
                .has_role(&[RoleName::Manager, RoleName::Technician])
        {
            return Ok(());
        }

        if ticket.reporter_id == user.id {
            return Ok(());
        }

        Err(AppError::NotFound)
    }

    pub fn delete(user: &User, _ticket: &Ticket) -> Result<(), AppError> {
        if user.role_name.is_admin() {
            return Ok(());
        }

        Err(AppError::Forbidden)
    }

    pub fn assign(user: &User, _ticket: &Ticket) -> Result<(), AppError> {
        if user.role_name.is_admin() || user.role_name.is_manager() {
            return Ok(());
        }

        Err(AppError::Forbidden)
    }

    pub fn unassign(user: &User, _ticket: &Ticket) -> Result<(), AppError> {
        if user.role_name.is_admin() || user.role_name.is_manager() {
            return Ok(());
        }

        Err(AppError::Forbidden)
    }

    pub fn change_status(user: &User, ticket: &Ticket) -> Result<(), AppError> {
        if user.role_name.is_admin() || user.role_name.is_manager() {
            return Ok(());
        }

        if user.role_name.is_technician() {
            if ticket.technician_id == Some(user.id) {
                return Ok(());
            }

            if ticket.status_id == 1 && ticket.technician_id.is_none() {
                return Ok(());
            }

            return Err(AppError::Forbidden);
        }

        if ticket.reporter_id == user.id {
            return Ok(());
        }

        Err(AppError::NotFound)
    }

    pub fn self_assign(user: &User, ticket: &Ticket) -> Result<(), AppError> {
        if user.role_name.is_technician() && ticket.status_id == 1 && ticket.technician_id.is_none()
        {
            return Ok(());
        }

        Err(AppError::Forbidden)
    }

    pub fn change_priority(user: &User, ticket: &Ticket) -> Result<(), AppError> {
        if user.role_name.is_admin() || user.role_name.is_manager() {
            return Ok(());
        }

        if user.role_name.is_technician() && ticket.technician_id == Some(user.id) {
            return Ok(());
        }

        Err(AppError::Forbidden)
    }

    pub fn comment(user: &User, ticket: &Ticket) -> Result<(), AppError> {
        let status = TicketStatusName::try_from(ticket.status_id);
        if status?.is_final() {
            return Err(AppError::Forbidden);
        }

        if user.role_name.is_admin() || user.role_name.is_manager() {
            return Ok(());
        }

        if user.role_name.is_technician() && ticket.technician_id == Some(user.id) {
            return Ok(());
        }

        if ticket.reporter_id == user.id {
            return Ok(());
        }

        Err(AppError::NotFound)
    }

    pub fn view_history(user: &User, ticket: &Ticket) -> Result<(), AppError> {
        Self::view(user, ticket)
    }

    pub fn attach(user: &User, ticket: &Ticket) -> Result<(), AppError> {
        Self::is_participant(user, ticket)
    }

    pub fn is_participant(user: &User, ticket: &Ticket) -> Result<(), AppError> {
        if user.role_name.is_admin() || user.role_name.is_manager() {
            return Ok(());
        }

        if user.role_name.is_technician() && ticket.technician_id == Some(user.id) {
            return Ok(());
        }

        if ticket.reporter_id == user.id {
            return Ok(());
        }

        Err(AppError::NotFound)
    }
}
