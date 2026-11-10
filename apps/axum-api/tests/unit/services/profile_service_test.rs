use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use bantuit::{
    application::{dtos::UpdatePasswordRequest, services::auth::ProfileService},
    common::{AppError, Crypto},
    domain::{
        enums::{RoleName, UserStatus},
        models::{Department, EmployeeProfile, User},
        repositories::{AuthRepo, ProfileRepo},
    },
    interface::extractors::auth_user::AuthUser,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::common::create_mock_user;

struct MockProfileRepo {
    pub updated_password: Mutex<Option<String>>,
}

#[async_trait]
impl ProfileRepo for MockProfileRepo {
    async fn update_password_and_revoke_token(
        &self,
        _user_id: Uuid,
        _current_token_hash: &str,
        new_password: String,
    ) -> Result<(), AppError> {
        let mut lock = self.updated_password.lock().unwrap();
        *lock = Some(new_password);
        Ok(())
    }
}

struct MockAuthRepo {
    pub department: Option<Department>,
    pub profile: Option<EmployeeProfile>,
}

#[async_trait]
impl AuthRepo for MockAuthRepo {
    async fn find_user_by_email(&self, _email: &str) -> Result<Option<User>, AppError> {
        Ok(None)
    }

    async fn clean_personal_access_token(&self, _hashed_token: String) -> Result<(), AppError> {
        Ok(())
    }

    async fn create_token_and_update_last_login(
        &self,
        _user_id: Uuid,
        _hashed_token: &str,
    ) -> Result<(), AppError> {
        Ok(())
    }

    async fn find_user_by_token_hash(
        &self,
        _hashed_token: &str,
        _threshold: DateTime<Utc>,
    ) -> Result<User, AppError> {
        Err(AppError::Unauthorized)
    }

    async fn touch_token(&self, _hashed_token: &str) -> Result<(), AppError> {
        Ok(())
    }

    async fn get_department_by_department_id(
        &self,
        _department_id: i32,
    ) -> Result<Option<Department>, AppError> {
        Ok(self.department.clone())
    }

    async fn get_profile_by_user_id(
        &self,
        _user_id: Uuid,
    ) -> Result<Option<EmployeeProfile>, AppError> {
        Ok(self.profile.clone())
    }

    async fn set_last_login_at(
        &self,
        _user_id: Uuid,
        _time: DateTime<Utc>,
    ) -> Result<(), AppError> {
        Ok(())
    }
}

#[tokio::test]
async fn test_profile_service_show_with_full_relations() {
    let mut user = create_mock_user(RoleName::Administrator);
    user.department_id = Some(1);

    let department = Department {
        id: 1,
        name: "Teknologi Informasi".to_string(),
        description: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    };

    let profile = EmployeeProfile {
        id: Uuid::new_v4(),
        user_id: user.id,
        employee_code: "EMP-0001".to_string(),
        phone: Some("08123456789".to_string()),
        position: Some("Lead Architect".to_string()),
        hire_date: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    };

    let auth_repo = Arc::new(MockAuthRepo {
        department: Some(department),
        profile: Some(profile),
    });
    let profile_repo = Arc::new(MockProfileRepo {
        updated_password: Mutex::new(None),
    });

    let service = ProfileService::new(profile_repo, auth_repo);
    let me = service
        .show(user.clone())
        .await
        .expect("show profil harus berhasil");

    assert_eq!(me.id, user.id);
    assert_eq!(me.email, user.email);
    assert_eq!(me.full_name, user.full_name);
    assert_eq!(me.status, UserStatus::Active);
    assert_eq!(me.permissions.as_ref().unwrap().len(), 67);

    let dept = me.department.expect("Department harus ada");
    assert_eq!(dept.id, 1);
    assert_eq!(dept.name, "Teknologi Informasi");

    let prof = me.profile.expect("Profile karyawan harus ada");
    assert_eq!(prof.employee_code, "EMP-0001");
    assert_eq!(prof.position.as_deref(), Some("Lead Architect"));
    assert_eq!(prof.phone.as_deref(), Some("08123456789"));
}

#[tokio::test]
async fn test_profile_service_show_with_empty_relations() {
    let mut user = create_mock_user(RoleName::Employee);
    user.department_id = None;

    let auth_repo = Arc::new(MockAuthRepo {
        department: None,
        profile: None,
    });
    let profile_repo = Arc::new(MockProfileRepo {
        updated_password: Mutex::new(None),
    });

    let service = ProfileService::new(profile_repo, auth_repo);
    let me = service
        .show(user.clone())
        .await
        .expect("show profil harus berhasil");

    assert!(me.department.is_none());
    assert!(me.profile.is_none());
    assert!(me.permissions.contains(&"ticket.create".to_string()));
}

#[tokio::test]
async fn test_profile_service_update_password_fails_on_incorrect_current_password() {
    let mut user = create_mock_user(RoleName::Employee);
    let raw_current = "PasswordLama123";
    user.password = Crypto::hash_password(raw_current).unwrap();

    let auth_repo = Arc::new(MockAuthRepo {
        department: None,
        profile: None,
    });
    let profile_repo = Arc::new(MockProfileRepo {
        updated_password: Mutex::new(None),
    });

    let service = ProfileService::new(profile_repo, auth_repo);

    let request = UpdatePasswordRequest {
        current_password: "PasswordSalahTotal".to_string(),
        password: "PasswordBaru123".to_string(),
        password_confirmation: "PasswordBaru123".to_string(),
    };

    let result = service
        .update_password(AuthUser(user, "token_hash".to_string()), request)
        .await;

    assert!(matches!(result, Err(AppError::ValidationError(_))));
}

#[tokio::test]
async fn test_profile_service_update_password_success() {
    let mut user = create_mock_user(RoleName::Employee);
    let raw_current = "PasswordLama123";
    user.password = Crypto::hash_password(raw_current).unwrap();

    let auth_repo = Arc::new(MockAuthRepo {
        department: None,
        profile: None,
    });
    let profile_repo = Arc::new(MockProfileRepo {
        updated_password: Mutex::new(None),
    });

    let service = ProfileService::new(profile_repo.clone(), auth_repo);

    let request = UpdatePasswordRequest {
        current_password: raw_current.to_string(),
        password: "PasswordBaru123".to_string(),
        password_confirmation: "PasswordBaru123".to_string(),
    };

    let result = service
        .update_password(AuthUser(user, "token_hash".to_string()), request)
        .await;

    assert!(result.is_ok());

    let updated = profile_repo.updated_password.lock().unwrap();
    assert!(updated.is_some());
    assert!(Crypto::verify_password(
        "PasswordBaru123",
        updated.as_ref().unwrap()
    ));
}
