use crate::{
    common::AppError,
    domain::{
        authorization::policies::ticket_policy::TicketPolicy,
        models::{Ticket, TicketAttachment, User},
    },
};

pub struct TicketAttachmentPolicy;

impl TicketAttachmentPolicy {
    pub fn view(
        user: &User,
        _ticket_attachment: &TicketAttachment,
        ticket: &Ticket,
    ) -> Result<(), AppError> {
        TicketPolicy::view(user, ticket)
    }

    pub fn download(
        user: &User,
        ticket_attachment: &TicketAttachment,
        ticket: &Ticket,
    ) -> Result<(), AppError> {
        Self::view(user, ticket_attachment, ticket)
    }

    pub fn create(_user: &User, _ticket_attachment: &TicketAttachment) -> Result<(), AppError> {
        Ok(())
    }

    pub fn delete(user: &User, ticket_attachment: &TicketAttachment) -> Result<(), AppError> {
        if user.role_name.is_admin() || user.role_name.is_manager() {
            return Ok(());
        }
        if ticket_attachment.uploaded_by == user.id {
            return Ok(());
        }
        Err(AppError::Forbidden)
    }
}
