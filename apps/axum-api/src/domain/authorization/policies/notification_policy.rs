use crate::{
    common::AppError,
    domain::models::{Notification, User},
};

pub struct NotificationPolicy;

impl NotificationPolicy {
    pub fn view(_user: &User, _notification: &Notification) -> Result<(), AppError> {
        Err(AppError::NotFound)
    }

    pub fn view_any(_user: &User) -> Result<(), AppError> {
        Ok(())
    }

    pub fn mark_as_read(user: &User, notification: &Notification) -> Result<(), AppError> {
        if notification.user_id == user.id {
            return Ok(());
        }
        Err(AppError::Forbidden)
    }

    pub fn mark_all_as_read(_user: &User) -> Result<(), AppError> {
        Ok(())
    }
}
