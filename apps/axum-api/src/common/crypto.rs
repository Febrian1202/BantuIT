use anyhow::anyhow;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand::Rng;
use sha2::{Digest, Sha256};

use crate::common::AppError;

pub struct Crypto;

impl Crypto {
    pub fn hash_password(plain_password: &str) -> Result<String, AppError> {
        let argon2 = Argon2::default();

        argon2
            .hash_password(plain_password.as_bytes())
            .map(|hash| hash.to_string())
            .map_err(|e| {
                tracing::error!("Gagal membuat hash password: {:?}", e);
                AppError::Internal(anyhow!("Password hashing error: {}", e))
            })
    }

    pub fn verify_password(plain_password: &str, hash_password: &str) -> bool {
        let argon2 = Argon2::default();

        match argon2.verify_password(plain_password.as_bytes(), hash_password) {
            Ok(()) => true,
            Err(e) => {
                tracing::debug!("Verifikasi password gagal: {:?}", e);
                false
            }
        }
    }

    pub fn generate_token() -> (String, String) {
        let mut bytes = [0u8; 40];
        rand::rng().fill_bytes(&mut bytes);
        let plain_token = hex::encode(bytes);

        let hashed_token = Self::hash_token(&plain_token);

        (plain_token, hashed_token)
    }

    pub fn hash_token(plain_token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(plain_token.as_bytes());
        hex::encode(hasher.finalize())
    }
}
