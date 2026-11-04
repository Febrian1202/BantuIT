use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};

use crate::{
    common::AppError,
    domain::{models::User, repositories::DynAuthRepo},
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
}
