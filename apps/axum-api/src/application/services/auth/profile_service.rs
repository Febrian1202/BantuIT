use std::collections::HashMap;

use crate::{
    application::dtos::UpdatePasswordRequest,
    common::{AppError, Crypto},
    domain::repositories::DynProfileRepo,
    interface::extractors::auth_user::AuthUser,
};

#[derive(Clone)]
pub struct ProfileService {
    pub profile_repo: DynProfileRepo,
}

impl ProfileService {
    pub fn new(profile_repo: DynProfileRepo) -> Self {
        Self { profile_repo }
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
}
