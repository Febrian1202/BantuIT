use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use bantuit::domain::enums::{RoleName, UserStatus};
use tower::ServiceExt;

use crate::common::{insert_test_user, response_json, setup_test_app};

#[tokio::test]
async fn test_get_me_endpoint_returns_full_profile_and_67_permissions_for_admin() {
    let (state, app) = setup_test_app().await;

    let (admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/me")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Profile retrieved successfully.");

    let data = &json["data"];
    assert_eq!(data["id"], admin.id.to_string());
    assert_eq!(data["email"], admin.email);
    assert_eq!(data["full_name"], admin.full_name);
    assert_eq!(data["must_change_password"], false);
    assert_eq!(data["role"]["name"], "administrator");

    // Profile karyawan terisi
    assert!(data["profile"]["employee_code"].is_string());
    assert_eq!(data["profile"]["position"], "Staff");

    // Paritas 1:1 terhadap Laravel (67 permissions)
    let permissions = data["permissions"]
        .as_array()
        .expect("permissions harus berupa array");
    assert_eq!(permissions.len(), 67);

    let perms_str: Vec<&str> = permissions.iter().filter_map(|p| p.as_str()).collect();
    assert!(perms_str.contains(&"ticket.create"));
    assert!(perms_str.contains(&"ticket.viewAny"));
    assert!(perms_str.contains(&"article.create"));
    assert!(perms_str.contains(&"asset.viewAny"));
    assert!(perms_str.contains(&"dashboard.admin"));
    assert!(perms_str.contains(&"user.lookup"));
}

#[tokio::test]
async fn test_get_me_unauthenticated_returns_401() {
    let (_state, app) = setup_test_app().await;

    let request = Request::builder()
        .uri("/api/me")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_me_blocked_if_user_must_change_password() {
    let (state, app) = setup_test_app().await;

    // User yang memiliki flag must_change_password: true
    let (_user, token, _) =
        insert_test_user(&state.pool, RoleName::Employee, true, UserStatus::Active).await;

    let request = Request::builder()
        .uri("/api/me")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Middleware ensure_password_checked harus memblokir akses ke /api/me dengan 403 Forbidden
    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert_eq!(json["message"], "Password change required.");
}

#[tokio::test]
async fn test_update_profile_success_and_persists_changes() {
    let (state, app) = setup_test_app().await;

    let (user, token, _) =
        insert_test_user(&state.pool, RoleName::Employee, false, UserStatus::Active).await;

    let payload = serde_json::json!({
        "full_name": "Budi Santoso Diperbarui",
        "phone": "081234567890",
    });

    let request = Request::builder()
        .uri("/api/me")
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Profile updated successfully.");

    let data = &json["data"];
    assert_eq!(data["id"], user.id.to_string());
    assert_eq!(data["full_name"], "Budi Santoso Diperbarui");
    assert_eq!(data["profile"]["phone"], "081234567890");

    // Pada update profil, permissions diabaikan (skip_serializing_if)
    assert!(data.get("permissions").is_none() || data["permissions"].is_null());

    // Verifikasi lewat GET /api/me bahwa perubahan tersimpan di database
    let get_request = Request::builder()
        .uri("/api/me")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let get_response = app.oneshot(get_request).await.unwrap();
    assert_eq!(get_response.status(), StatusCode::OK);

    let get_json = response_json(get_response).await;
    assert_eq!(get_json["data"]["full_name"], "Budi Santoso Diperbarui");
    assert_eq!(get_json["data"]["profile"]["phone"], "081234567890");
}

#[tokio::test]
async fn test_update_profile_without_phone_preserves_existing_phone() {
    let (state, app) = setup_test_app().await;

    let (_user, token, _) =
        insert_test_user(&state.pool, RoleName::Employee, false, UserStatus::Active).await;

    let payload = serde_json::json!({
        "full_name": "Nama Baru Tanpa Ganti Phone",
        "phone": null,
    });

    let request = Request::builder()
        .uri("/api/me")
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["full_name"], "Nama Baru Tanpa Ganti Phone");
    // Phone lama ("08123456789" dari seeder) tetap dipertahankan
    assert_eq!(json["data"]["profile"]["phone"], "08123456789");
}

#[tokio::test]
async fn test_update_profile_fails_on_empty_name() {
    let (state, app) = setup_test_app().await;

    let (_user, token, _) =
        insert_test_user(&state.pool, RoleName::Employee, false, UserStatus::Active).await;

    let payload = serde_json::json!({
        "full_name": "",
        "phone": "08123456789",
    });

    let request = Request::builder()
        .uri("/api/me")
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert!(json["errors"]["full_name"].is_array());
}

#[tokio::test]
async fn test_update_profile_unauthenticated_returns_401() {
    let (_state, app) = setup_test_app().await;

    let payload = serde_json::json!({
        "full_name": "Unauthorized User",
    });

    let request = Request::builder()
        .uri("/api/me")
        .method("PUT")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_password_success_and_verifies_login_with_new_password() {
    let (state, app) = setup_test_app().await;

    let (user, token, old_password) =
        insert_test_user(&state.pool, RoleName::Employee, false, UserStatus::Active).await;

    let new_password = "NewSecretPassword123!";
    let payload = serde_json::json!({
        "current_password": old_password,
        "password": new_password,
        "password_confirmation": new_password,
    });

    let update_request = Request::builder()
        .uri("/api/me/password")
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let update_response = app.clone().oneshot(update_request).await.unwrap();
    assert_eq!(update_response.status(), StatusCode::OK);

    let json = response_json(update_response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Password updated successfully.");

    // Coba login ulang dengan password baru
    let login_payload = serde_json::json!({
        "email": user.email,
        "password": new_password,
    });

    let login_request = Request::builder()
        .uri("/api/login")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(login_payload.to_string()))
        .unwrap();

    let login_response = app.oneshot(login_request).await.unwrap();
    assert_eq!(login_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_update_password_fails_on_incorrect_current_password() {
    let (state, app) = setup_test_app().await;

    let (_user, token, _) =
        insert_test_user(&state.pool, RoleName::Employee, false, UserStatus::Active).await;

    let payload = serde_json::json!({
        "current_password": "WrongCurrentPassword123!",
        "password": "NewSecretPassword123!",
        "password_confirmation": "NewSecretPassword123!",
    });

    let request = Request::builder()
        .uri("/api/me/password")
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert_eq!(
        json["errors"]["current_password"][0],
        "The current password is incorrect."
    );
}
