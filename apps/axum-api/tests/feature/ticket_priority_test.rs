use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use bantuit::domain::enums::{RoleName, UserStatus};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use crate::common::{insert_test_user, response_json, setup_test_app};

fn random_level() -> i32 {
    let (v1, _) = Uuid::new_v4().as_u64_pair();
    ((v1 % 900_000_000) + 10_000) as i32
}

#[tokio::test]
async fn test_get_ticket_priorities_unauthenticated_returns_401() {
    let (_state, app) = setup_test_app().await;

    let request = Request::builder()
        .uri("/api/ticket-priorities")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_ticket_priorities_blocked_if_user_must_change_password() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        true, // must_change_password = true
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/ticket-priorities")
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
async fn test_get_ticket_priorities_success_for_authenticated_employee() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/ticket-priorities")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Ticket priorities retrieved.");

    let priorities = json["data"]
        .as_array()
        .expect("data harus berupa array");
    assert!(!priorities.is_empty());

    // Pastikan terurut berdasarkan sla_minutes ASC
    let mut prev_sla = -1;
    for item in priorities {
        let sla = item["sla_minutes"].as_i64().expect("sla_minutes harus angka") as i32;
        assert!(sla >= prev_sla, "Prioritas harus terurut berdasarkan sla_minutes ASC");
        prev_sla = sla;
    }
}

#[tokio::test]
async fn test_show_ticket_priority_success() {
    let (state, app) = setup_test_app().await;

    let prio_name = format!("PrioShow-{}", Uuid::new_v4().simple());
    let prio_level = random_level();
    let prio_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO ticket_priorities (name, level, sla_minutes, description)
        VALUES ($1, $2, 15, 'Sangat darurat')
        RETURNING id
        "#,
    )
    .bind(&prio_name)
    .bind(prio_level)
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
        .uri(format!("/api/ticket-priorities/{prio_id}"))
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Ticket priority retrieved.");
    assert_eq!(json["data"]["id"], prio_id);
    assert_eq!(json["data"]["name"], prio_name);
    assert_eq!(json["data"]["level"], prio_level);
    assert_eq!(json["data"]["sla_minutes"], 15);
}

#[tokio::test]
async fn test_show_ticket_priority_not_found_returns_404() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/ticket-priorities/999999")
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
async fn test_create_ticket_priority_forbidden_for_employee() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        false,
        UserStatus::Active,
    )
    .await;

    let payload = json!({
        "name": "Super Critical",
        "level": random_level(),
        "sla_minutes": 30
    });

    let request = Request::builder()
        .uri("/api/ticket-priorities")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_create_ticket_priority_success_for_admin() {
    let (state, app) = setup_test_app().await;

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let prio_name = format!("Urgent-{}", Uuid::new_v4().simple());
    let new_level = random_level();

    let payload = json!({
        "name": prio_name,
        "level": new_level,
        "sla_minutes": 45,
        "description": "Ditangani dalam 45 menit"
    });

    let request = Request::builder()
        .uri("/api/ticket-priorities")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Ticket priority created successfully.");
    assert_eq!(json["data"]["name"], prio_name);
    assert_eq!(json["data"]["level"], new_level);
    assert_eq!(json["data"]["sla_minutes"], 45);

    let created_id = json["data"]["id"].as_i64().unwrap() as i32;

    // Verifikasi row tersimpan di database
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ticket_priorities WHERE id = $1 AND deleted_at IS NULL")
        .bind(created_id)
        .fetch_one(&state.pool)
        .await
        .unwrap();
    assert_eq!(count, 1);

    // Verifikasi audit log tercatat
    let audit_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM audit_logs WHERE module = 'ticket_priority' AND module_id = $1 AND action = 'create'"
    )
    .bind(created_id.to_string())
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert_eq!(audit_count, 1);
}

#[tokio::test]
async fn test_create_ticket_priority_fails_on_duplicate_name() {
    let (state, app) = setup_test_app().await;

    let prio_name = format!("DupPrio-{}", Uuid::new_v4().simple());
    let prio_level = random_level();
    sqlx::query("INSERT INTO ticket_priorities (name, level, sla_minutes) VALUES ($1, $2, 60)")
        .bind(&prio_name)
        .bind(prio_level)
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
        "name": prio_name,
        "level": random_level(),
        "sla_minutes": 90
    });

    let request = Request::builder()
        .uri("/api/ticket-priorities")
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
        "Nama prioritas tiket sudah digunakan."
    );
}

#[tokio::test]
async fn test_create_ticket_priority_fails_on_duplicate_level() {
    let (state, app) = setup_test_app().await;

    let existing_level = random_level();
    sqlx::query("INSERT INTO ticket_priorities (name, level, sla_minutes) VALUES ($1, $2, 60)")
        .bind(format!("ExistingLevel-{}", Uuid::new_v4().simple()))
        .bind(existing_level)
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
        "name": format!("NewPrio-{}", Uuid::new_v4().simple()),
        "level": existing_level,
        "sla_minutes": 100
    });

    let request = Request::builder()
        .uri("/api/ticket-priorities")
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
        json["errors"]["level"][0],
        "Level prioritas sudah digunakan."
    );
}

#[tokio::test]
async fn test_update_ticket_priority_forbidden_for_employee() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        false,
        UserStatus::Active,
    )
    .await;

    let payload = json!({
        "name": "Baru",
        "level": random_level(),
        "sla_minutes": 60
    });

    let request = Request::builder()
        .uri("/api/ticket-priorities/1")
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_update_ticket_priority_not_found_returns_404() {
    let (state, app) = setup_test_app().await;

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let payload = json!({
        "name": "NonExistent",
        "level": random_level(),
        "sla_minutes": 60
    });

    let request = Request::builder()
        .uri("/api/ticket-priorities/999999")
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_ticket_priority_success_for_admin() {
    let (state, app) = setup_test_app().await;

    let original_name = format!("OrigPrio-{}", Uuid::new_v4().simple());
    let original_level = random_level();
    let prio_id: i32 = sqlx::query_scalar(
        "INSERT INTO ticket_priorities (name, level, sla_minutes, description) VALUES ($1, $2, 60, 'Lama') RETURNING id",
    )
    .bind(&original_name)
    .bind(original_level)
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

    let updated_name = format!("UpdPrio-{}", Uuid::new_v4().simple());
    let updated_level = random_level();
    let payload = json!({
        "name": updated_name,
        "level": updated_level,
        "sla_minutes": 35,
        "description": "Baru diupdate"
    });

    let request = Request::builder()
        .uri(format!("/api/ticket-priorities/{prio_id}"))
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Ticket priority updated successfully.");
    assert_eq!(json["data"]["name"], updated_name);
    assert_eq!(json["data"]["level"], updated_level);
    assert_eq!(json["data"]["sla_minutes"], 35);
    assert_eq!(json["data"]["description"], "Baru diupdate");

    // Pastikan audit log update tercatat
    let audit_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM audit_logs WHERE module = 'ticket_priority' AND module_id = $1 AND action = 'update'"
    )
    .bind(prio_id.to_string())
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert_eq!(audit_count, 1);
}

#[tokio::test]
async fn test_update_ticket_priority_allows_same_name_and_level_for_same_id() {
    let (state, app) = setup_test_app().await;

    let name = format!("KeepPrio-{}", Uuid::new_v4().simple());
    let level = random_level();
    let prio_id: i32 = sqlx::query_scalar(
        "INSERT INTO ticket_priorities (name, level, sla_minutes, description) VALUES ($1, $2, 60, 'Desc A') RETURNING id",
    )
    .bind(&name)
    .bind(level)
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
        "name": name,   // Nama tetap sama
        "level": level, // Level tetap sama
        "sla_minutes": 45,
        "description": "Desc B"
    });

    let request = Request::builder()
        .uri(format!("/api/ticket-priorities/{prio_id}"))
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_update_ticket_priority_fails_on_duplicate_name_with_other_priority() {
    let (state, app) = setup_test_app().await;

    let name1 = format!("PrioOne-{}", Uuid::new_v4().simple());
    let name2 = format!("PrioTwo-{}", Uuid::new_v4().simple());
    let level1 = random_level();
    let level2 = random_level();

    sqlx::query("INSERT INTO ticket_priorities (name, level, sla_minutes) VALUES ($1, $2, 60)")
        .bind(&name1)
        .bind(level1)
        .execute(&state.pool)
        .await
        .unwrap();

    let prio_id2: i32 = sqlx::query_scalar("INSERT INTO ticket_priorities (name, level, sla_minutes) VALUES ($1, $2, 60) RETURNING id")
        .bind(&name2)
        .bind(level2)
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

    // Coba update name2 menjadi name1
    let payload = json!({
        "name": name1,
        "level": level2,
        "sla_minutes": 60
    });

    let request = Request::builder()
        .uri(format!("/api/ticket-priorities/{prio_id2}"))
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
        "Nama prioritas tiket sudah digunakan."
    );
}

#[tokio::test]
async fn test_update_ticket_priority_fails_on_duplicate_level_with_other_priority() {
    let (state, app) = setup_test_app().await;

    let name1 = format!("LevelOne-{}", Uuid::new_v4().simple());
    let name2 = format!("LevelTwo-{}", Uuid::new_v4().simple());
    let level1 = random_level();
    let level2 = random_level();

    sqlx::query("INSERT INTO ticket_priorities (name, level, sla_minutes) VALUES ($1, $2, 60)")
        .bind(&name1)
        .bind(level1)
        .execute(&state.pool)
        .await
        .unwrap();

    let prio_id2: i32 = sqlx::query_scalar("INSERT INTO ticket_priorities (name, level, sla_minutes) VALUES ($1, $2, 60) RETURNING id")
        .bind(&name2)
        .bind(level2)
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

    // Coba update level prio2 menjadi level1
    let payload = json!({
        "name": name2,
        "level": level1,
        "sla_minutes": 60
    });

    let request = Request::builder()
        .uri(format!("/api/ticket-priorities/{prio_id2}"))
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let json = response_json(response).await;
    assert_eq!(
        json["errors"]["level"][0],
        "Level prioritas sudah digunakan."
    );
}

#[tokio::test]
async fn test_delete_ticket_priority_forbidden_for_employee() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/ticket-priorities/1")
        .method("DELETE")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_ticket_priority_not_found_returns_404() {
    let (state, app) = setup_test_app().await;

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/ticket-priorities/999999")
        .method("DELETE")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_ticket_priority_conflict_when_referenced_by_ticket() {
    let (state, app) = setup_test_app().await;

    let prio_name = format!("PrioRef-{}", Uuid::new_v4().simple());
    let prio_level = random_level();
    let prio_id: i32 = sqlx::query_scalar(
        "INSERT INTO ticket_priorities (name, level, sla_minutes) VALUES ($1, $2, 60) RETURNING id",
    )
    .bind(&prio_name)
    .bind(prio_level)
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

    // Buat tiket yang merujuk prioritas ini
    let ticket_id = Uuid::now_v7();
    sqlx::query(
        r#"
        INSERT INTO tickets (
            id, ticket_number, title, description, category_id, priority_id,
            status_id, reporter_id, sla_duration_minutes
        )
        VALUES ($1, $2, $3, $4, 1, $5, 1, $6, 60)
        "#,
    )
    .bind(ticket_id)
    .bind(format!("TIC-{}", Uuid::new_v4().simple()))
    .bind("Tiket Uji Guard Priority")
    .bind("Deskripsi tiket")
    .bind(prio_id)
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
        .uri(format!("/api/ticket-priorities/{prio_id}"))
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
            .contains("Prioritas tiket masih digunakan")
    );
}

#[tokio::test]
async fn test_delete_ticket_priority_success_for_admin() {
    let (state, app) = setup_test_app().await;

    let prio_name = format!("PrioDel-{}", Uuid::new_v4().simple());
    let prio_level = random_level();
    let prio_id: i32 = sqlx::query_scalar(
        "INSERT INTO ticket_priorities (name, level, sla_minutes) VALUES ($1, $2, 60) RETURNING id",
    )
    .bind(&prio_name)
    .bind(prio_level)
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
        .uri(format!("/api/ticket-priorities/{prio_id}"))
        .method("DELETE")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Ticket priority deleted successfully.");

    // Pastikan di database sudah di-soft-delete (deleted_at IS NOT NULL)
    let is_deleted: bool = sqlx::query_scalar(
        "SELECT deleted_at IS NOT NULL FROM ticket_priorities WHERE id = $1",
    )
    .bind(prio_id)
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert!(is_deleted);

    // Pastikan audit log delete tercatat
    let audit_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM audit_logs WHERE module = 'ticket_priority' AND module_id = $1 AND action = 'delete'",
    )
    .bind(prio_id.to_string())
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert_eq!(audit_count, 1);

    // Pastikan show berikutnya mengembalikan 404
    let show_req = Request::builder()
        .uri(format!("/api/ticket-priorities/{prio_id}"))
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let show_res = app.oneshot(show_req).await.unwrap();
    assert_eq!(show_res.status(), StatusCode::NOT_FOUND);
}
