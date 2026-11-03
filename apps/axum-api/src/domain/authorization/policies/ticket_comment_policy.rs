use chrono::{Duration, Utc};

use crate::{
    common::AppError,
    domain::models::{TicketComment, User},
};

pub struct TicketCommentPolicy;

impl TicketCommentPolicy {
    pub fn update(user: &User, comment: &TicketComment) -> Result<(), AppError> {
        if user.role_name.is_admin() {
            return Ok(());
        }

        if comment.user_id != user.id {
            return Err(AppError::NotFound);
        }

        let now = Utc::now();
        if now > comment.created_at + Duration::minutes(15) {
            return Err(AppError::Forbidden);
        }

        Ok(())
    }

    pub fn delete(user: &User, comment: &TicketComment) -> Result<(), AppError> {
        Self::update(user, comment)
    }
}
