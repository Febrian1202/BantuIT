use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use bantuit::domain::enums::{RoleName, UserStatus};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use crate::common::{insert_test_user, response_json, setup_test_app};

fn unique_name(prefix: &str) -> String {
    format!("{prefix}-{}", Uuid::new_v4().simple())
}

fn unique_desc(prefix: &str) -> String {
    let s = format!("{prefix}-{}", Uuid::new_v4().simple());
    if s.len() > 50 { s[..50].to_string() } else { s }
}

#[tokio::test]
async fn test_get_departments_unauthenticated_returns_401() {
    let (_state, app) = setup_test_app().await;

    let request = Request::builder()
        .uri("/api/departments")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_departments_blocked_if_user_must_change_password() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        true, // must_change_password = true
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/departments")
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
async fn test_get_departments_success_for_authenticated_employee() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/departments")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Departments retrieved.");

    let departments = json["data"]
        .as_array()
        .expect("data harus berupa array");
    assert!(!departments.is_empty());

    // Pastikan terurut berdasarkan name ASC
    let mut prev_name = String::new();
    for item in departments {
        let name = item["name"].as_str().expect("name harus string");
        assert!(
            name >= prev_name.as_str(),
            "Departemen harus terurut berdasarkan name ASC (prev='{prev_name}', cur='{name}')"
        );
        prev_name = name.to_string();
    }
}

#[tokio::test]
async fn test_show_department_success() {
    let (state, app) = setup_test_app().await;

    let dept_name = unique_name("DeptShow");
    let dept_desc = unique_desc("DescShow");
    let dept_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO departments (name, description, created_at, updated_at)
        VALUES ($1, $2, NOW(), NOW())
        RETURNING id
        "#,
    )
    .bind(&dept_name)
    .bind(&dept_desc)
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
        .uri(format!("/api/departments/{dept_id}"))
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Department retrieved.");
    assert_eq!(json["data"]["id"], dept_id);
    assert_eq!(json["data"]["name"], dept_name);
    assert_eq!(json["data"]["description"], dept_desc);
}

#[tokio::test]
async fn test_show_department_not_found_returns_404() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/departments/999999")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_show_department_soft_deleted_returns_404() {
    let (state, app) = setup_test_app().await;

    let dept_name = unique_name("DeptDel404");
    let dept_desc = unique_desc("DescDel404");
    let dept_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO departments (name, description, created_at, updated_at, deleted_at)
        VALUES ($1, $2, NOW(), NOW(), NOW())
        RETURNING id
        "#,
    )
    .bind(&dept_name)
    .bind(&dept_desc)
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
        .uri(format!("/api/departments/{dept_id}"))
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_create_department_forbidden_for_employee() {
    let (state, app) = setup_test_app().await;

    let (_employee, token, _) = insert_test_user(
        &state.pool,
        RoleName::Employee,
        false,
        UserStatus::Active,
    )
    .await;

    let payload = json!({
        "name": "Unauthorized Department",
        "description": "Tidak boleh dibuat oleh karyawan"
    });

    let request = Request::builder()
        .uri("/api/departments")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_create_department_success_for_admin() {
    let (state, app) = setup_test_app().await;

    let (admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let dept_name = unique_name("DeptCreate");
    let dept_desc = unique_desc("DescCreate");
    let payload = json!({
        "name": dept_name,
        "description": dept_desc
    });

    let request = Request::builder()
        .uri("/api/departments")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Department created successfully.");
    assert_eq!(json["data"]["name"], dept_name);
    assert_eq!(json["data"]["description"], dept_desc);

    let dept_id = json["data"]["id"].as_i64().unwrap() as i32;

    // Verifikasi tersimpan di database
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM departments WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(dept_id)
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert!(exists);

    // Verifikasi audit_logs tercatat
    let audit_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM audit_logs
        WHERE module = 'department'
          AND action = 'create'
          AND module_id = $1
          AND user_id = $2
        "#,
    )
    .bind(dept_id.to_string())
    .bind(admin.id)
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert!(audit_count >= 1);
}

#[tokio::test]
async fn test_create_department_fails_on_duplicate_name() {
    let (state, app) = setup_test_app().await;

    let dept_name = unique_name("DeptDup");
    let dept_desc = unique_desc("DescDup");
    sqlx::query(
        "INSERT INTO departments (name, description, created_at, updated_at) VALUES ($1, $2, NOW(), NOW())",
    )
    .bind(&dept_name)
    .bind(&dept_desc)
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
        "name": dept_name,
        "description": unique_desc("DescNewDup")
    });

    let request = Request::builder()
        .uri("/api/departments")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert!(
        json["errors"]["name"]
            .as_array()
            .unwrap()
            .iter()
            .any(|e| e.as_str().unwrap().contains("sudah digunakan"))
    );
}

#[tokio::test]
async fn test_update_department_forbidden_for_employee() {
    let (state, app) = setup_test_app().await;

    let dept_name = unique_name("DeptEmpUp");
    let dept_id: i32 = sqlx::query_scalar(
        "INSERT INTO departments (name, created_at, updated_at) VALUES ($1, NOW(), NOW()) RETURNING id",
    )
    .bind(&dept_name)
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

    let payload = json!({
        "name": "New Name",
        "description": "Update attempt"
    });

    let request = Request::builder()
        .uri(format!("/api/departments/{dept_id}"))
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_update_department_not_found_returns_404() {
    let (state, app) = setup_test_app().await;

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let payload = json!({
        "name": unique_name("DeptNotFound"),
        "description": unique_desc("DescNotFound")
    });

    let request = Request::builder()
        .uri("/api/departments/999999")
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_department_success_for_admin() {
    let (state, app) = setup_test_app().await;

    let old_name = unique_name("DeptOld");
    let old_desc = unique_desc("DescOld");
    let dept_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO departments (name, description, created_at, updated_at)
        VALUES ($1, $2, NOW(), NOW())
        RETURNING id
        "#,
    )
    .bind(&old_name)
    .bind(&old_desc)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let (admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let new_name = unique_name("DeptNew");
    let new_desc = unique_desc("DescNew");
    let payload = json!({
        "name": new_name,
        "description": new_desc
    });

    let request = Request::builder()
        .uri(format!("/api/departments/{dept_id}"))
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Department updated successfully.");
    assert_eq!(json["data"]["id"], dept_id);
    assert_eq!(json["data"]["name"], new_name);
    assert_eq!(json["data"]["description"], new_desc);

    // Verifikasi database
    let db_name: String = sqlx::query_scalar("SELECT name FROM departments WHERE id = $1")
        .bind(dept_id)
        .fetch_one(&state.pool)
        .await
        .unwrap();
    assert_eq!(db_name, new_name);

    // Verifikasi audit_logs tercatat dengan action update
    let audit_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM audit_logs
        WHERE module = 'department'
          AND action = 'update'
          AND module_id = $1
          AND user_id = $2
        "#,
    )
    .bind(dept_id.to_string())
    .bind(admin.id)
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert!(audit_count >= 1);
}

#[tokio::test]
async fn test_update_department_allows_same_name_for_same_id() {
    let (state, app) = setup_test_app().await;

    let dept_name = unique_name("DeptSame");
    let old_desc = unique_desc("DescSameOld");
    let dept_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO departments (name, description, created_at, updated_at)
        VALUES ($1, $2, NOW(), NOW())
        RETURNING id
        "#,
    )
    .bind(&dept_name)
    .bind(&old_desc)
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

    let new_desc = unique_desc("DescSameNew");
    let payload = json!({
        "name": dept_name, // Menggunakan nama yang sama
        "description": new_desc
    });

    let request = Request::builder()
        .uri(format!("/api/departments/{dept_id}"))
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["description"], new_desc);
}

#[tokio::test]
async fn test_update_department_fails_on_duplicate_name_with_other_department() {
    let (state, app) = setup_test_app().await;

    let name_a = unique_name("DeptA");
    let name_b = unique_name("DeptB");
    let desc_a = unique_desc("DescA");
    let desc_b = unique_desc("DescB");

    sqlx::query("INSERT INTO departments (name, description, created_at, updated_at) VALUES ($1, $2, NOW(), NOW())")
        .bind(&name_a)
        .bind(&desc_a)
        .execute(&state.pool)
        .await
        .unwrap();

    let id_b: i32 = sqlx::query_scalar(
        "INSERT INTO departments (name, description, created_at, updated_at) VALUES ($1, $2, NOW(), NOW()) RETURNING id",
    )
    .bind(&name_b)
    .bind(&desc_b)
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

    // Coba update Dept B menggunakan nama Dept A
    let payload = json!({
        "name": name_a,
        "description": unique_desc("DescConflict")
    });

    let request = Request::builder()
        .uri(format!("/api/departments/{id_b}"))
        .method("PUT")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert!(
        json["errors"]["name"]
            .as_array()
            .unwrap()
            .iter()
            .any(|e| e.as_str().unwrap().contains("sudah digunakan"))
    );
}

#[tokio::test]
async fn test_delete_department_forbidden_for_employee() {
    let (state, app) = setup_test_app().await;

    let dept_name = unique_name("DeptDelEmp");
    let dept_id: i32 = sqlx::query_scalar(
        "INSERT INTO departments (name, created_at, updated_at) VALUES ($1, NOW(), NOW()) RETURNING id",
    )
    .bind(&dept_name)
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
        .uri(format!("/api/departments/{dept_id}"))
        .method("DELETE")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_department_not_found_returns_404() {
    let (state, app) = setup_test_app().await;

    let (_admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri("/api/departments/999999")
        .method("DELETE")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_department_conflict_when_referenced_by_user() {
    let (state, app) = setup_test_app().await;

    let dept_name = unique_name("DeptRefUser");
    let dept_id: i32 = sqlx::query_scalar(
        "INSERT INTO departments (name, created_at, updated_at) VALUES ($1, NOW(), NOW()) RETURNING id",
    )
    .bind(&dept_name)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    // Buat user yang merujuk pada department ini
    let email = format!("user-dept-ref-{}@test.com", Uuid::new_v4().simple());
    sqlx::query(
        r#"
        INSERT INTO users (id, role_id, department_id, email, password, full_name, status)
        VALUES ($1, 4, $2, $3, 'dummy-hashed', 'User Departemen Uji', 'active')
        "#,
    )
    .bind(Uuid::now_v7())
    .bind(dept_id)
    .bind(email)
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
        .uri(format!("/api/departments/{dept_id}"))
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
            .contains("Departemen masih digunakan")
    );
}

#[tokio::test]
async fn test_delete_department_conflict_when_referenced_by_ticket() {
    let (state, app) = setup_test_app().await;

    let dept_name = unique_name("DeptRefTicket");
    let dept_id: i32 = sqlx::query_scalar(
        "INSERT INTO departments (name, created_at, updated_at) VALUES ($1, NOW(), NOW()) RETURNING id",
    )
    .bind(&dept_name)
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

    // Buat tiket yang merujuk pada department ini
    let ticket_id = Uuid::now_v7();
    sqlx::query(
        r#"
        INSERT INTO tickets (
            id, ticket_number, title, description, category_id, priority_id,
            status_id, reporter_id, department_id, sla_duration_minutes
        )
        VALUES ($1, $2, 'Tiket Departemen', 'Deskripsi tiket', 1, 1, 1, $3, $4, 60)
        "#,
    )
    .bind(ticket_id)
    .bind(format!("TIC-DEPT-{}", Uuid::new_v4().simple()))
    .bind(reporter.id)
    .bind(dept_id)
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
        .uri(format!("/api/departments/{dept_id}"))
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
            .contains("Departemen masih digunakan")
    );
}

#[tokio::test]
async fn test_delete_department_success_for_admin() {
    let (state, app) = setup_test_app().await;

    let dept_name = unique_name("DeptDelSuccess");
    let dept_id: i32 = sqlx::query_scalar(
        "INSERT INTO departments (name, created_at, updated_at) VALUES ($1, NOW(), NOW()) RETURNING id",
    )
    .bind(&dept_name)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let (admin, token, _) = insert_test_user(
        &state.pool,
        RoleName::Administrator,
        false,
        UserStatus::Active,
    )
    .await;

    let request = Request::builder()
        .uri(format!("/api/departments/{dept_id}"))
        .method("DELETE")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;
    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Department deleted successfully.");

    // Verifikasi soft delete di DB
    let deleted_at: Option<chrono::DateTime<chrono::Utc>> =
        sqlx::query_scalar("SELECT deleted_at FROM departments WHERE id = $1")
            .bind(dept_id)
            .fetch_one(&state.pool)
            .await
            .unwrap();
    assert!(deleted_at.is_some(), "deleted_at harus terisi");

    // Verifikasi audit_logs tercatat dengan action delete
    let audit_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM audit_logs
        WHERE module = 'department'
          AND action = 'delete'
          AND module_id = $1
          AND user_id = $2
        "#,
    )
    .bind(dept_id.to_string())
    .bind(admin.id)
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert!(audit_count >= 1);

    // Verifikasi GET selanjutnya menghasilkan 404 Not Found
    let get_request = Request::builder()
        .uri(format!("/api/departments/{dept_id}"))
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let get_response = app.oneshot(get_request).await.unwrap();
    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}
