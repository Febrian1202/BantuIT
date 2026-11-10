use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use bantuit::domain::enums::{RoleName, UserStatus};
use tower::ServiceExt;

use crate::common::{insert_test_user, response_json, setup_test_app};

#[tokio::test]
async fn test_login_successful_with_valid_credentials() {
    let (state, app) = setup_test_app().await;

    let (_user, _token, password) =
        insert_test_user(&state.pool, RoleName::Employee, false, UserStatus::Active).await;

    let payload = serde_json::json!({
        "email": _user.email,
        "password": password,
    });

    let request = Request::builder()
        .uri("/api/login")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Login successful.");
    assert!(json["data"]["token"].is_string());
    assert_eq!(json["data"]["user"]["email"], _user.email);
    assert_eq!(json["data"]["user"]["role"]["name"], "employee");
}

#[tokio::test]
async fn test_login_fails_with_invalid_credentials() {
    let (_state, app) = setup_test_app().await;

    let payload = serde_json::json!({
        "email": "nonexistent@jarvisops.test",
        "password": "WrongPassword123!",
    });

    let request = Request::builder()
        .uri("/api/login")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert_eq!(
        json["errors"]["email"][0],
        "These credentials do not match our records."
    );
}

#[tokio::test]
async fn test_login_fails_with_inactive_account() {
    let (state, app) = setup_test_app().await;

    let (user, _token, password) =
        insert_test_user(&state.pool, RoleName::Employee, false, UserStatus::Inactive).await;

    let payload = serde_json::json!({
        "email": user.email,
        "password": password,
    });

    let request = Request::builder()
        .uri("/api/login")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert_eq!(json["errors"]["email"][0], "This account is inactive.");
}

#[tokio::test]
async fn test_logout_revokes_token_successfully() {
    let (state, app) = setup_test_app().await;

    let (_user, plain_token, _) =
        insert_test_user(&state.pool, RoleName::Employee, false, UserStatus::Active).await;

    // 1. Logout dengan Bearer token yang valid
    let logout_request = Request::builder()
        .uri("/api/logout")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {plain_token}"))
        .body(Body::empty())
        .unwrap();

    let logout_response = app.clone().oneshot(logout_request).await.unwrap();
    assert_eq!(logout_response.status(), StatusCode::OK);

    let json = response_json(logout_response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Logout successful.");

    // 2. Request /api/me berikutnya dengan token yang sudah di-revoke harus 401 Unauthorized
    let me_request = Request::builder()
        .uri("/api/me")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {plain_token}"))
        .body(Body::empty())
        .unwrap();

    let me_response = app.oneshot(me_request).await.unwrap();
    assert_eq!(me_response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_logout_unauthenticated_returns_401() {
    let (_state, app) = setup_test_app().await;

    let request = Request::builder()
        .uri("/api/logout")
        .method("POST")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
