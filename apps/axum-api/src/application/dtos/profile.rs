use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdatePasswordRequest {
    #[validate(length(min = 1, message = "Password saat ini wajib diisi."))]
    pub current_password: String,

    #[validate(
        length(min = 8, message = "Password baru minimal harus 8 karakter.",),
        custom(function = "validate_password_complexity"),
        must_match(
            other = "password_confirmation",
            message = "Konfirmasi password baru tidak sesuai"
        )
    )]
    pub password: String,

    pub password_confirmation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePasswordResponse;

fn validate_password_complexity(password: &str) -> Result<(), ValidationError> {
    let has_letters = password.chars().any(|c| c.is_alphabetic());
    let has_numbers = password.chars().any(|c| c.is_ascii_digit());

    if !has_letters {
        let mut error = ValidationError::new("password_letters");
        error.message = Some(Cow::Borrowed("Password baru harus mengandung huruf."));
        return Err(error);
    }

    if !has_numbers {
        let mut error = ValidationError::new("password_numbers");
        error.message = Some(Cow::Borrowed("Password baru harus mengandung angka."));
        return Err(error);
    }

    Ok(())
}
