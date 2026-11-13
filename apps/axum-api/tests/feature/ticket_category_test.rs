use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use bantuit::domain::enums::{RoleName, UserStatus};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

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
    assert_eq!(json["message"], "Password change required.");
}

#[tokio::test]
async fn test_get_ticket_categories_success_for_authenticated_employee() {
    let (state, app) = setup_test_app().await;

    let parent_name = format!("Hardware-{}", Uuid::new_v4().simple());
    let child_name = format!("Laptop-{}", Uuid::new_v4().simple());

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

    let child_cat = categories
        .iter()
        .find(|c| c["name"] == child_name)
        .expect("child category harus ditemukan");

    assert_eq!(child_cat["parent_id"], parent_id);
    assert_eq!(child_cat["parent"]["id"], parent_id);
    assert_eq!(child_cat["parent"]["name"], parent_name);
}

#[tokio::test]
async fn test_show_ticket_category_success() {
    let (state, app) = setup_test_app().await;

    let cat_name = format!("Networking-{}", Uuid::new_v4().simple());
    let cat_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO ticket_categories (name, description)
        VALUES ($1, $2)
        RETURNING id
        "#,
    )
    .bind(&cat_name)
    .bind("Jaringan & internet")
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
        .uri(format!("/api/ticket-categories/{cat_id}"))
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Ticket category retrieved.");
    assert_eq!(json["data"]["id"], cat_id);
    assert_eq!(json["data"]["name"], cat_name);
}

#[tokio::test]
async fn test_show_ticket_category_not_found_returns_404() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/ticket-categories/999999")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert_eq!(json["message"], "Resource not found.");
}

#[tokio::test]
async fn test_create_ticket_category_forbidden_for_employee() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        false,
        UserStatus::Active,
    )
    .await;

    let payload = json!({
        "name": format!("Cat-{}", Uuid::new_v4().simple()),
        "description": "Deskripsi"
    });

    let request = Request::builder()
        .uri("/api/ticket-categories")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_create_ticket_category_success_for_admin() {
    let (state, app) = setup_test_app().await;

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let cat_name = format!("Server-{}", Uuid::new_v4().simple());
    let payload = json!({
        "name": cat_name,
        "description": "Infrastruktur server & cloud"
    });

    let request = Request::builder()
        .uri("/api/ticket-categories")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Ticket category created successfully.");
    assert_eq!(json["data"]["name"], cat_name);

    let created_id = json["data"]["id"].as_i64().unwrap() as i32;

    // Verifikasi row tersimpan di DB
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ticket_categories WHERE id = $1 AND deleted_at IS NULL")
        .bind(created_id)
        .fetch_one(&state.pool)
        .await
        .unwrap();
    assert_eq!(count, 1);

    // Verifikasi audit log tercatat
    let audit_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM audit_logs WHERE module = 'ticket_category' AND module_id = $1 AND action = 'create'"
    )
    .bind(created_id.to_string())
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert_eq!(audit_count, 1);
}

#[tokio::test]
async fn test_create_ticket_category_with_parent_success() {
    let (state, app) = setup_test_app().await;

    let parent_name = format!("Hardware-{}", Uuid::new_v4().simple());
    let parent_id: i32 = sqlx::query_scalar(
        "INSERT INTO ticket_categories (name, description) VALUES ($1, $2) RETURNING id",
    )
    .bind(&parent_name)
    .bind("Perangkat keras")
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let child_name = format!("Printer-{}", Uuid::new_v4().simple());
    let payload = json!({
        "name": child_name,
        "description": "Mesin cetak kantor",
        "parent_id": parent_id
    });

    let request = Request::builder()
        .uri("/api/ticket-categories")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let json = response_json(response).await;
    assert_eq!(json["data"]["parent_id"], parent_id);
    assert_eq!(json["data"]["parent"]["id"], parent_id);
    assert_eq!(json["data"]["parent"]["name"], parent_name);
}

#[tokio::test]
async fn test_create_ticket_category_fails_on_duplicate_name() {
    let (state, app) = setup_test_app().await;

    let cat_name = format!("DuplicateName-{}", Uuid::new_v4().simple());
    sqlx::query("INSERT INTO ticket_categories (name) VALUES ($1)")
        .bind(&cat_name)
        .execute(&state.pool)
        .await
        .unwrap();

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let payload = json!({
        "name": cat_name,
        "description": "Duplikat nama"
    });

    let request = Request::builder()
        .uri("/api/ticket-categories")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert_eq!(
        json["errors"]["name"][0],
        "Nama kategori tiket sudah digunakan."
    );
}

#[tokio::test]
async fn test_create_ticket_category_fails_on_invalid_parent() {
    let (state, app) = setup_test_app().await;

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let payload = json!({
        "name": format!("ValidName-{}", Uuid::new_v4().simple()),
        "parent_id": 999999
    });

    let request = Request::builder()
        .uri("/api/ticket-categories")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let json = response_json(response).await;
    assert_eq!(
        json["errors"]["parent_id"][0],
        "Kategori induk tidak valid."
    );
}

#[tokio::test]
async fn test_update_ticket_category_forbidden_for_employee() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        false,
        UserStatus::Active,
    )
    .await;

    let payload = json!({
        "name": "Baru"
    });

    let request = Request::builder()
        .uri("/api/ticket-categories/1")
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_update_ticket_category_success_for_admin() {
    let (state, app) = setup_test_app().await;

    let original_name = format!("BeforeUpdate-{}", Uuid::new_v4().simple());
    let cat_id: i32 = sqlx::query_scalar(
        "INSERT INTO ticket_categories (name, description) VALUES ($1, $2) RETURNING id",
    )
    .bind(&original_name)
    .bind("Deskripsi lama")
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let updated_name = format!("AfterUpdate-{}", Uuid::new_v4().simple());
    let payload = json!({
        "name": updated_name,
        "description": "Deskripsi baru terupdate",
        "parent_id": null
    });

    let request = Request::builder()
        .uri(format!("/api/ticket-categories/{cat_id}"))
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Ticket category updated successfully.");
    assert_eq!(json["data"]["name"], updated_name);
    assert_eq!(json["data"]["description"], "Deskripsi baru terupdate");

    // Pastikan audit log update tercatat
    let audit_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM audit_logs WHERE module = 'ticket_category' AND module_id = $1 AND action = 'update'"
    )
    .bind(cat_id.to_string())
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert_eq!(audit_count, 1);
}

#[tokio::test]
async fn test_update_ticket_category_allows_same_name_for_same_id() {
    let (state, app) = setup_test_app().await;

    let name = format!("SameName-{}", Uuid::new_v4().simple());
    let cat_id: i32 = sqlx::query_scalar(
        "INSERT INTO ticket_categories (name, description) VALUES ($1, $2) RETURNING id",
    )
    .bind(&name)
    .bind("Deskripsi A")
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let payload = json!({
        "name": name, // Nama tetap sama
        "description": "Deskripsi B terupdate"
    });

    let request = Request::builder()
        .uri(format!("/api/ticket-categories/{cat_id}"))
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_update_ticket_category_fails_on_duplicate_name_with_other_category() {
    let (state, app) = setup_test_app().await;

    let name1 = format!("CatOne-{}", Uuid::new_v4().simple());
    let name2 = format!("CatTwo-{}", Uuid::new_v4().simple());

    sqlx::query("INSERT INTO ticket_categories (name) VALUES ($1)")
        .bind(&name1)
        .execute(&state.pool)
        .await
        .unwrap();

    let cat_id2: i32 = sqlx::query_scalar("INSERT INTO ticket_categories (name) VALUES ($1) RETURNING id")
        .bind(&name2)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    // Coba ganti nama CatTwo menjadi CatOne
    let payload = json!({
        "name": name1
    });

    let request = Request::builder()
        .uri(format!("/api/ticket-categories/{cat_id2}"))
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let json = response_json(response).await;
    assert_eq!(
        json["errors"]["name"][0],
        "Nama kategori tiket sudah digunakan."
    );
}

#[tokio::test]
async fn test_update_ticket_category_fails_on_self_parenting() {
    let (state, app) = setup_test_app().await;

    let name = format!("SelfParent-{}", Uuid::new_v4().simple());
    let cat_id: i32 = sqlx::query_scalar("INSERT INTO ticket_categories (name) VALUES ($1) RETURNING id")
        .bind(&name)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let payload = json!({
        "name": name,
        "parent_id": cat_id // Parent sama dengan dirinya sendiri
    });

    let request = Request::builder()
        .uri(format!("/api/ticket-categories/{cat_id}"))
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let json = response_json(response).await;
    assert_eq!(
        json["errors"]["parent_id"][0],
        "Kategori tidak dapat menjadi induk bagi dirinya sendiri."
    );
}

#[tokio::test]
async fn test_delete_ticket_category_forbidden_for_employee() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/ticket-categories/1")
        .method("DELETE")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_ticket_category_not_found_returns_404() {
    let (state, app) = setup_test_app().await;

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/ticket-categories/999999")
        .method("DELETE")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_ticket_category_conflict_when_referenced_by_child() {
    let (state, app) = setup_test_app().await;

    let parent_name = format!("ParentCat-{}", Uuid::new_v4().simple());
    let parent_id: i32 = sqlx::query_scalar(
        "INSERT INTO ticket_categories (name) VALUES ($1) RETURNING id",
    )
    .bind(&parent_name)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let child_name = format!("ChildCat-{}", Uuid::new_v4().simple());
    sqlx::query("INSERT INTO ticket_categories (name, parent_id) VALUES ($1, $2)")
        .bind(&child_name)
        .bind(parent_id)
        .execute(&state.pool)
        .await
        .unwrap();

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri(format!("/api/ticket-categories/{parent_id}"))
        .method("DELETE")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);

    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert!(
        json["message"]
            .as_str()
            .unwrap()
            .contains("Kategori tiket (induk) masih digunakan")
    );
}

#[tokio::test]
async fn test_delete_ticket_category_conflict_when_referenced_by_ticket() {
    let (state, app) = setup_test_app().await;

    let cat_name = format!("TicketParent-{}", Uuid::new_v4().simple());
    let cat_id: i32 = sqlx::query_scalar(
        "INSERT INTO ticket_categories (name) VALUES ($1) RETURNING id",
    )
    .bind(&cat_name)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let (reporter, _, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        false,
        UserStatus::Active,
    )
    .await;

    // Buat tiket yang merujuk category ini
    let ticket_id = Uuid::now_v7();
    sqlx::query(
        r#"
        INSERT INTO tickets (
            id, ticket_number, title, description, category_id, priority_id,
            status_id, reporter_id, sla_duration_minutes
        )
        VALUES ($1, $2, $3, $4, $5, 1, 1, $6, 120)
        "#,
    )
    .bind(ticket_id)
    .bind(format!("TIC-{}", Uuid::new_v4().simple()))
    .bind("Tiket Uji Guard")
    .bind("Deskripsi tiket")
    .bind(cat_id)
    .bind(reporter.id)
    .execute(&state.pool)
    .await
    .unwrap();

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri(format!("/api/ticket-categories/{cat_id}"))
        .method("DELETE")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);

    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert!(
        json["message"]
            .as_str()
            .unwrap()
            .contains("Kategori tiket masih digunakan")
    );
}

#[tokio::test]
async fn test_delete_ticket_category_success_for_admin() {
    let (state, app) = setup_test_app().await;

    let cat_name = format!("ToDelete-{}", Uuid::new_v4().simple());
    let cat_id: i32 = sqlx::query_scalar(
        "INSERT INTO ticket_categories (name) VALUES ($1) RETURNING id",
    )
    .bind(&cat_name)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri(format!("/api/ticket-categories/{cat_id}"))
        .method("DELETE")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Ticket category deleted successfully.");

    // Pastikan di DB sudah di-soft-delete (deleted_at IS NOT NULL)
    let is_deleted: bool = sqlx::query_scalar(
        "SELECT deleted_at IS NOT NULL FROM ticket_categories WHERE id = $1",
    )
    .bind(cat_id)
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert!(is_deleted);

    // Pastikan audit log delete tercatat
    let audit_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM audit_logs WHERE module = 'ticket_category' AND module_id = $1 AND action = 'delete'",
    )
    .bind(cat_id.to_string())
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert_eq!(audit_count, 1);
}
