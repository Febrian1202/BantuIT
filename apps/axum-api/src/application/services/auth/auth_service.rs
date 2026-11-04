use std::collections::HashMap;

use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};

use crate::{
    application::dtos::{DepartmentData, LoginRequest, LoginResponse, RoleData, UserData},
    common::{AppError, Crypto},
    domain::{enums::UserStatus, models::User, repositories::DynAuthRepo},
};

#[derive(Clone)]
pub struct AuthService {
    auth_repo: DynAuthRepo,
}

impl AuthService {
    /// Constructor untuk AuthService
    pub fn new(auth_repo: DynAuthRepo) -> Self {
        Self { auth_repo }
    }

    pub async fn login(&self, data: LoginRequest) -> Result<LoginResponse, AppError> {
        let user = self
            .auth_repo
            .find_user_by_email(&data.email)
            .await?
            .ok_or_else(|| {
                Self::validation_error("email", "These credentials do not match our records.")
            })?;

        let password_valid = Crypto::verify_password(&data.password, &user.password);
        if !password_valid {
            return Err(Self::validation_error(
                "email",
                "These credentials do not match our records.",
            ));
        }

        if user.status != UserStatus::Active {
            return Err(Self::validation_error("email", "This account is inactive."));
        }

        let (plain_token, hashed_token) = Crypto::generate_token();

        self.auth_repo
            .create_token_and_update_last_login(user.id, &hashed_token)
            .await?;

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

        let role = RoleData {
            id: user.role_id,
            name: user.role_name,
        };

        let user_data = UserData {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            status: user.status,
            must_change_password: user.must_change_password,
            role,
            department,
        };

        let response = LoginResponse {
            token: plain_token,
            user: user_data,
        };

        Ok(response)
    }

    pub async fn verify_token(&self, plain_token: &str) -> Result<User, AppError> {
        let mut hasher = Sha256::new();
        hasher.update(plain_token.as_bytes());
        let hashed_token = hex::encode(hasher.finalize());

        let threshold = Utc::now() - Duration::hours(12);

        let user = self
            .auth_repo
            .find_user_by_token_hash(&hashed_token, threshold)
            .await?;

        let repo = self.auth_repo.clone();
        tokio::spawn(async move {
            if let Err(e) = repo.touch_token(&hashed_token).await {
                tracing::warn!("Gagal memperbarui last_used_at: {:?}", e);
            };
        });

        Ok(user)
    }

    fn validation_error(field: &str, message: &str) -> AppError {
        let mut errors = HashMap::new();
        errors.insert(field.to_string(), vec![message.to_string()]);
        AppError::ValidationError(errors)
    }
}
