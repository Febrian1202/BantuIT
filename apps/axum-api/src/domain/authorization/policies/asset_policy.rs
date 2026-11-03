use crate::{
    common::AppError,
    domain::{
        enums::RoleName,
        models::{Asset, User},
    },
};

pub struct AssetPolicy;

impl AssetPolicy {
    pub fn view_assignable(_user: &User) -> Result<(), AppError> {
        Ok(())
    }

    pub fn view_own(_user: &User) -> Result<(), AppError> {
        Ok(())
    }

    pub fn view(user: &User, _asset: &Asset) -> Result<(), AppError> {
        if user.role_name.is_admin()
            || user
                .role_name
                .has_role(&[RoleName::Manager, RoleName::Technician])
        {
            return Ok(());
        }
        Err(AppError::Forbidden)
    }

    pub fn view_any(user: &User) -> Result<(), AppError> {
        if user.role_name.is_admin()
            || user
                .role_name
                .has_role(&[RoleName::Manager, RoleName::Technician])
        {
            return Ok(());
        }

        Err(AppError::Forbidden)
    }

    pub fn create(user: &User) -> Result<(), AppError> {
        if user.role_name.is_admin()
            || user
                .role_name
                .has_role(&[RoleName::Manager, RoleName::Technician])
        {
            return Ok(());
        }

        Err(AppError::Forbidden)
    }

    pub fn update(user: &User) -> Result<(), AppError> {
        if user.role_name.is_admin()
            || user
                .role_name
                .has_role(&[RoleName::Manager, RoleName::Technician])
        {
            return Ok(());
        }

        Err(AppError::Forbidden)
    }

    pub fn delete(user: &User) -> Result<(), AppError> {
        if user.role_name.is_admin() || user.role_name.is_manager() {
            return Ok(());
        }

        Err(AppError::Forbidden)
    }

    pub fn assign(user: &User) -> Result<(), AppError> {
        if user.role_name.is_admin()
            || user
                .role_name
                .has_role(&[RoleName::Manager, RoleName::Technician])
        {
            return Ok(());
        }

        Err(AppError::Forbidden)
    }

    pub fn release(user: &User) -> Result<(), AppError> {
        if user.role_name.is_admin()
            || user
                .role_name
                .has_role(&[RoleName::Manager, RoleName::Technician])
        {
            return Ok(());
        }
        Err(AppError::Forbidden)
    }

    pub fn view_history(user: &User) -> Result<(), AppError> {
        if user.role_name.is_admin()
            || user
                .role_name
                .has_role(&[RoleName::Manager, RoleName::Technician])
        {
            return Ok(());
        }
        Err(AppError::Forbidden)
    }
}
