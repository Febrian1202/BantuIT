use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::{
    application::dtos::{DepartmentData, ProfileData, RoleData},
    domain::enums::UserStatus,
};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdatePasswordRequest {
    #[validate(length(min = 1, message = "Password saat ini wajib diisi."))]
    pub current_password: String,

    #[validate(
        length(min = 8, message = "Password baru minimal harus 8 karakter.",),
        custom(function = "validate_password_complexity"),
        must_match(
            other = "password_confirmation",
            message = "Konfirmasi password baru tidak sesuai."
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeResponse {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub status: UserStatus,
    pub must_change_password: bool,
    pub role: RoleData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<DepartmentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<ProfileData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(custom(function = "validate_full_name"))]
    pub full_name: String,

    #[validate(length(
        max = 20,
        message = "Nomor telepon tidak boleh lebih dari 20 karakter."
    ))]
    pub phone: Option<String>,
}

fn validate_full_name(name: &str) -> Result<(), ValidationError> {
    let len = name.chars().count();

    if len < 1 {
        let mut err = ValidationError::new("full_name_min");
        err.message = Some("Nama lengkap wajib diisi.".into());
        return Err(err);
    }

    if len > 255 {
        let mut err = ValidationError::new("full_name_max");
        err.message = Some("Nama lengkap tidak boleh lebih dari 255 karakter.".into());
        return Err(err);
    }

    Ok(())
}
