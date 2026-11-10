use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::enums::{RoleName, UserStatus};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(
        length(min = 1, message = "Email wajib diisi."),
        email(message = "Format email tidak valid.")
    )]
    pub email: String,

    #[validate(length(min = 1, message = "Password wajib diisi."))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub status: UserStatus,
    pub must_change_password: bool,
    pub role: RoleData,
    pub department: Option<DepartmentData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleData {
    pub id: i32,
    pub name: RoleName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentData {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileData {
    pub employee_code: String,
    pub position: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutResponse;
