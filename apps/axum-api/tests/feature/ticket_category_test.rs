use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use bantuit::domain::enums::{RoleName, UserStatus};
use tower::ServiceExt;

use crate::common::{insert_test_user, response_json, setup_test_app};

#[tokio::test]
async fn test_get_ticket_categories_unauthenticated_returns_401() {
    let (_state, app) = setup_test_app().await;

    let request = Request::builder()
        .uri("/api/ticket-categories")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_ticket_categories_blocked_if_user_must_change_password() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        true, // must_change_password = true
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/ticket-categories")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert_eq!(
        json["message"],
        "Password change required."
    );
}

#[tokio::test]
async fn test_get_ticket_categories_success_for_authenticated_employee() {
    let (state, app) = setup_test_app().await;

    // Masukkan kategori parent & child untuk verifikasi relasi dan sorting
    let parent_name = format!("Hardware-{}", uuid::Uuid::new_v4().simple());
    let child_name = format!("Laptop-{}", uuid::Uuid::new_v4().simple());

    let parent_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO ticket_categories (name, description)
        VALUES ($1, $2)
        RETURNING id
        "#,
    )
    .bind(&parent_name)
    .bind("Perangkat keras")
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let _child_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO ticket_categories (name, description, parent_id)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
    )
    .bind(&child_name)
    .bind("Komputer jinjing")
    .bind(parent_id)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/ticket-categories")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Ticket categories retrieved.");

    let categories = json["data"]
        .as_array()
        .expect("data harus berupa array");
    assert!(!categories.is_empty());

    // Pastikan child kategori memuat relasi parent dengan benar
    let child_cat = categories
        .iter()
        .find(|c| c["name"] == child_name)
        .expect("child category harus ditemukan");

    assert_eq!(child_cat["parent_id"], parent_id);
    assert_eq!(child_cat["parent"]["id"], parent_id);
    assert_eq!(child_cat["parent"]["name"], parent_name);
}
