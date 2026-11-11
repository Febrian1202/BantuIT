use std::collections::HashMap;

use crate::{
    application::dtos::{
        DepartmentData, MeResponse, ProfileData, RoleData, UpdatePasswordRequest,
        UpdateProfileRequest,
    },
    common::{AppError, Crypto},
    domain::{
        authorization::AbilityMatrix,
        models::User,
        repositories::auth::{DynAuthRepo, DynProfileRepo},
    },
    interface::extractors::auth_user::AuthUser,
};

#[derive(Clone)]
pub struct ProfileService {
    pub profile_repo: DynProfileRepo,
    pub auth_repo: DynAuthRepo,
}

impl ProfileService {
    pub fn new(profile_repo: DynProfileRepo, auth_repo: DynAuthRepo) -> Self {
        Self {
            profile_repo,
            auth_repo,
        }
    }

    pub async fn update_password(
        &self,
        AuthUser(user, token_hash): AuthUser,
        data: UpdatePasswordRequest,
    ) -> Result<(), AppError> {
        if !Crypto::verify_password(&data.current_password, &user.password) {
            let mut errors = HashMap::new();
            errors.insert(
                "current_password".to_string(),
                vec!["The current password is incorrect.".to_string()],
            );
            return Err(AppError::ValidationError(errors));
        }

        let hashed_password = Crypto::hash_password(&data.password)?;

        self.profile_repo
            .update_password_and_revoke_token(user.id, &token_hash, hashed_password)
            .await?;

        Ok(())
    }

    pub async fn show(&self, user: User) -> Result<MeResponse, AppError> {
        let role = RoleData {
            id: user.role_id,
            name: user.role_name,
        };

        let department = match user.department_id {
            Some(id) => self
                .auth_repo
                .get_department_by_department_id(id)
                .await?
                .map(|d| DepartmentData {
                    id: d.id,
                    name: d.name,
                }),
            None => None,
        };

        let profile = self
            .auth_repo
            .get_profile_by_user_id(user.id)
            .await?
            .map(|d| ProfileData {
                employee_code: d.employee_code,
                position: d.position,
                phone: d.phone,
            });

        let permissions = AbilityMatrix::permission_for(user.role_name)
            .into_iter()
            .map(String::from)
            .collect();

        let response = MeResponse {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            status: user.status,
            must_change_password: user.must_change_password,
            role,
            department,
            profile,
            permissions: Some(permissions),
        };

        Ok(response)
    }

    pub async fn update(
        &self,
        user: User,
        data: UpdateProfileRequest,
    ) -> Result<MeResponse, AppError> {
        let (updated_user, updated_profile) = self
            .profile_repo
            .update_profile(user.id, data.full_name, data.phone)
            .await?;

        let department = match updated_user.department_id {
            Some(id) => self
                .auth_repo
                .get_department_by_department_id(id)
                .await?
                .map(|d| DepartmentData {
                    id: d.id,
                    name: d.name,
                }),
            None => None,
        };

        let role = RoleData {
            id: updated_user.role_id,
            name: updated_user.role_name,
        };

        let profile = ProfileData {
            employee_code: updated_profile.employee_code,
            position: updated_profile.position,
            phone: updated_profile.phone,
        };

        let response = MeResponse {
            id: updated_user.id,
            email: updated_user.email,
            full_name: updated_user.full_name,
            must_change_password: updated_user.must_change_password,
            status: updated_user.status,
            department,
            role,
            profile: Some(profile),
            permissions: None,
        };

        Ok(response)
    }
}
