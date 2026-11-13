use bantuit::application::dtos::{
    CreateTicketCategoryRequest, LoginRequest, UpdatePasswordRequest, UpdateProfileRequest,
    UpdateTicketCategoryRequest,
};
use validator::Validate;

#[test]
fn test_login_request_validation_success() {
    let req = LoginRequest {
        email: "admin@jarvisops.test".to_string(),
        password: "SecretPassword123".to_string(),
    };
    assert!(req.validate().is_ok());
}

#[test]
fn test_login_request_validation_fails_on_empty_fields() {
    let empty_email = LoginRequest {
        email: "".to_string(),
        password: "SecretPassword123".to_string(),
    };
    assert!(empty_email.validate().is_err());

    let empty_pass = LoginRequest {
        email: "admin@jarvisops.test".to_string(),
        password: "".to_string(),
    };
    assert!(empty_pass.validate().is_err());
}

#[test]
fn test_login_request_validation_fails_on_invalid_email_format() {
    let invalid_email = LoginRequest {
        email: "not-a-valid-email".to_string(),
        password: "SecretPassword123".to_string(),
    };
    let result = invalid_email.validate();
    assert!(result.is_err());
}

#[test]
fn test_update_password_request_validation_success() {
    let req = UpdatePasswordRequest {
        current_password: "OldPassword123".to_string(),
        password: "NewPassword123".to_string(),
        password_confirmation: "NewPassword123".to_string(),
    };
    assert!(req.validate().is_ok());
}

#[test]
fn test_update_password_request_fails_on_short_password() {
    let short_req = UpdatePasswordRequest {
        current_password: "OldPassword123".to_string(),
        password: "Pass1".to_string(), // < 8 karakter
        password_confirmation: "Pass1".to_string(),
    };
    assert!(short_req.validate().is_err());
}

#[test]
fn test_update_password_request_fails_on_missing_letters_or_numbers() {
    // Hanya angka (tidak ada huruf)
    let no_letters = UpdatePasswordRequest {
        current_password: "OldPassword123".to_string(),
        password: "1234567890".to_string(),
        password_confirmation: "1234567890".to_string(),
    };
    assert!(no_letters.validate().is_err());

    // Hanya huruf (tidak ada angka)
    let no_numbers = UpdatePasswordRequest {
        current_password: "OldPassword123".to_string(),
        password: "PasswordRahasia".to_string(),
        password_confirmation: "PasswordRahasia".to_string(),
    };
    assert!(no_numbers.validate().is_err());
}

#[test]
fn test_update_password_request_fails_on_confirmation_mismatch() {
    let mismatch_req = UpdatePasswordRequest {
        current_password: "OldPassword123".to_string(),
        password: "NewPassword123".to_string(),
        password_confirmation: "DifferentPassword123".to_string(),
    };
    assert!(mismatch_req.validate().is_err());
}

#[test]
fn test_update_profile_request_validation_success() {
    let req = UpdateProfileRequest {
        full_name: "Budi Santoso".to_string(),
        phone: Some("08123456789".to_string()),
    };
    assert!(req.validate().is_ok());

    let req_no_phone = UpdateProfileRequest {
        full_name: "Budi Santoso".to_string(),
        phone: None,
    };
    assert!(req_no_phone.validate().is_ok());
}

#[test]
fn test_update_profile_request_fails_on_empty_or_excessive_name() {
    let empty_name = UpdateProfileRequest {
        full_name: "".to_string(),
        phone: Some("08123456789".to_string()),
    };
    assert!(empty_name.validate().is_err());

    let long_name = UpdateProfileRequest {
        full_name: "A".repeat(256),
        phone: Some("08123456789".to_string()),
    };
    assert!(long_name.validate().is_err());
}

#[test]
fn test_update_profile_request_fails_on_excessive_phone_length() {
    let long_phone = UpdateProfileRequest {
        full_name: "Budi Santoso".to_string(),
        phone: Some("0812345678901234567890".to_string()), // > 20 karakter
    };
    assert!(long_phone.validate().is_err());
}

#[test]
fn test_create_ticket_category_request_validation_success() {
    let req = CreateTicketCategoryRequest {
        name: "Hardware".to_string(),
        description: Some("Kategori hardware".to_string()),
        parent_id: None,
    };
    assert!(req.validate().is_ok());

    let req_with_parent = CreateTicketCategoryRequest {
        name: "Laptop".to_string(),
        description: None,
        parent_id: Some(1),
    };
    assert!(req_with_parent.validate().is_ok());
}

#[test]
fn test_create_ticket_category_request_fails_on_empty_name() {
    let req = CreateTicketCategoryRequest {
        name: "".to_string(),
        description: None,
        parent_id: None,
    };
    let result = req.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.field_errors().contains_key("name"));
}

#[test]
fn test_create_ticket_category_request_fails_on_name_exceeding_100_chars() {
    let req = CreateTicketCategoryRequest {
        name: "A".repeat(101),
        description: None,
        parent_id: None,
    };
    let result = req.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.field_errors().contains_key("name"));
}

#[test]
fn test_create_ticket_category_request_fails_on_description_exceeding_500_chars() {
    let req = CreateTicketCategoryRequest {
        name: "Hardware".to_string(),
        description: Some("D".repeat(501)),
        parent_id: None,
    };
    let result = req.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.field_errors().contains_key("description"));
}

#[test]
fn test_update_ticket_category_request_validation_success() {
    let req = UpdateTicketCategoryRequest {
        name: "Software & OS".to_string(),
        description: Some("Operating systems and software licenses".to_string()),
        parent_id: Some(2),
    };
    assert!(req.validate().is_ok());
}

#[test]
fn test_update_ticket_category_request_fails_on_empty_or_excessive_fields() {
    let empty_name = UpdateTicketCategoryRequest {
        name: "".to_string(),
        description: None,
        parent_id: None,
    };
    assert!(empty_name.validate().is_err());

    let long_desc = UpdateTicketCategoryRequest {
        name: "Valid Name".to_string(),
        description: Some("X".repeat(501)),
        parent_id: None,
    };
    assert!(long_desc.validate().is_err());
}
