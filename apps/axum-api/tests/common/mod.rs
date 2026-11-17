use std::{sync::LazyLock, time::Instant};

use axum::{Router, response::Response};
use bantuit::{
    common::Crypto,
    domain::{
        enums::{RoleName, TicketStatusName, UserStatus},
        models::{Ticket, User},
    },
    infrastructure::{AppConfig, database::DatabaseConfig},
    interface::{AppState, router::create_router},
};
use chrono::Utc;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::sync::OnceCell;
use uuid::Uuid;

/// Helper untuk membuat mock user
pub fn create_mock_user(role_name: RoleName) -> User {
    let role_id = match role_name {
        RoleName::Administrator => 1,
        RoleName::Manager => 2,
        RoleName::Technician => 3,
        RoleName::Employee => 4,
    };

    User {
        id: Uuid::new_v4(),
        role_id,
        department_id: None,
        email: format!("user-{}@example.com", Uuid::new_v4()),
        password: "hashed_password".to_string(),
        full_name: "Test User".to_string(),
        status: UserStatus::Active,
        must_change_password: false,
        last_login_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
        role_name,
    }
}

/// Helper untuk membuat Ticket tiruan dengan reporter_id dan status tertentu
pub fn create_mock_ticket(reporter_id: Uuid, status: TicketStatusName) -> Ticket {
    Ticket {
        id: Uuid::new_v4(),
        ticket_number: "TIC-20261028-0001".to_string(),
        title: "Test Ticket Actor Resolution".to_string(),
        description: "Testing ticket actor resolver".to_string(),
        category_id: 1,
        priority_id: 1,
        status_id: status.id(),
        reporter_id,
        technician_id: None,
        department_id: None,
        asset_id: None,
        sla_duration_minutes: 240,
        sla_deadline: None,
        sla_breached: false,
        sla_breached_at: None,
        resolved_at: None,
        closed_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    }
}

/// Inisialisasi AppState dan Router untuk feature/integration test
pub async fn setup_test_app() -> (AppState, Router) {
    dotenvy::dotenv().ok();
    dotenvy::from_filename("apps/axum-api/.env").ok();
    dotenvy::from_filename(".env").ok();

    let mut config = AppConfig::from_env().unwrap_or_else(|_| {
        let db_cfg = DatabaseConfig {
            host: std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: std::env::var("DB_PORT").map_or(5432, |p| p.parse().unwrap_or(5432)),
            user: std::env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()),
            password: std::env::var("DB_PASSWORD").unwrap_or_else(|_| "root".to_string()),
            name: std::env::var("DB_NAME").unwrap_or_else(|_| "bantuit".to_string()),
            db_max_connections: 20,
            db_min_connections: 1,
        };
        AppConfig {
            host: "0.0.0.0".to_string(),
            port: 9000,
            env: "testing".to_string(),
            db: db_cfg,
        }
    });

    if let Ok(user) = std::env::var("DB_USER") {
        config.db.user = user;
    }
    if let Ok(pass) = std::env::var("DB_PASSWORD") {
        config.db.password = pass;
    }

    let conn_str = std::env::var("DATABASE_URL").unwrap_or_else(|_| config.db.connection_string());

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&conn_str)
        .await
        .expect("Koneksi PostgreSQL untuk testing gagal. Pastikan PostgreSQL sedang berjalan.");

    // Reset database hanya sekali di awal eksekusi test suite
    DB_RESET
        .get_or_init(|| async {
            reset_test_database(&pool).await;
        })
        .await;

    let state = AppState::new(config, pool, Instant::now());
    let router = create_router(state.clone());
    (state, router)
}

static DEFAULT_HASHED_PASSWORD: LazyLock<String> =
    LazyLock::new(|| Crypto::hash_password("Password123!").expect("Hash password gagal"));

/// Helper untuk menyisipkan test user ke dalam database
pub async fn insert_test_user(
    pool: &PgPool,
    role_name: RoleName,
    must_change_password: bool,
    status: UserStatus,
) -> (User, String, String) {
    let role_id = match role_name {
        RoleName::Administrator => 1,
        RoleName::Manager => 2,
        RoleName::Technician => 3,
        RoleName::Employee => 4,
    };
    let user_id = Uuid::now_v7();
    let email = format!("test-{}@jarvisops.test", Uuid::new_v4());
    let raw_password = "Password123!".to_string();
    let hashed_password = &*DEFAULT_HASHED_PASSWORD;
    let full_name = format!("Test User {}", <&'static str>::from(role_name));
    let status_str: &'static str = status.into();

    sqlx::query(
        r#"
        INSERT INTO users (id, role_id, email, password, full_name, status, must_change_password)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(user_id)
    .bind(role_id)
    .bind(&email)
    .bind(hashed_password)
    .bind(&full_name)
    .bind(status_str)
    .bind(must_change_password)
    .execute(pool)
    .await
    .expect("Gagal menyisipkan user test");

    let profile_code = format!("EMP-{}", Uuid::new_v4().simple());
    sqlx::query(
        r#"
        INSERT INTO employee_profiles (user_id, employee_code, position, phone)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(user_id)
    .bind(&profile_code)
    .bind("Staff")
    .bind("08123456789")
    .execute(pool)
    .await
    .expect("Gagal menyisipkan employee profile");

    let (plain_token, token_hash) = Crypto::generate_token();

    sqlx::query(
        r#"
        INSERT INTO personal_access_tokens (tokenable_id, name, token, abilities, created_at, updated_at)
        VALUES ($1, 'auth_token', $2, '["*"]', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        "#,
    )
    .bind(user_id)
    .bind(&token_hash)
    .execute(pool)
    .await
    .expect("Gagal menyisipkan token test");

    let user = User {
        id: user_id,
        role_id,
        department_id: None,
        email,
        password: hashed_password.clone(),
        full_name,
        status,
        must_change_password,
        last_login_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
        role_name,
    };

    (user, plain_token, raw_password)
}

/// Helper untuk membaca response body sebagai JSON serde_json::Value
pub async fn response_json(response: Response) -> serde_json::Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Gagal membaca body response");
    serde_json::from_slice(&bytes).expect("Body response bukan format JSON valid")
}

static DB_RESET: OnceCell<()> = OnceCell::const_new();

/// Membersihkan seluruh data test dummy dan mengembalikan database ke kondisi master seeder yang bersih
pub async fn reset_test_database(pool: &PgPool) {
    let reset_query = r#"
        DELETE FROM ticket_attachments;
        DELETE FROM ticket_comments;
        DELETE FROM ticket_histories;
        DELETE FROM tickets;
        DELETE FROM asset_assignments;
        DELETE FROM asset_histories;
        DELETE FROM assets;
        DELETE FROM notifications;
        DELETE FROM knowledge_articles;
        DELETE FROM audit_logs;
        DELETE FROM personal_access_tokens;
        DELETE FROM employee_profiles;
        DELETE FROM users;

        DELETE FROM ticket_categories WHERE id > 5;
        UPDATE ticket_categories SET deleted_at = NULL WHERE id <= 5;
        INSERT INTO ticket_categories (id, name, description) VALUES
            (1, 'Hardware', 'Perangkat keras dan komputer'),
            (2, 'Software', 'Sistem operasi dan aplikasi'),
            (3, 'Network', 'Konektivitas jaringan dan internet'),
            (4, 'Account', 'Identitas akun dan akses'),
            (5, 'Other', 'Kategori lainnya')
        ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, description = EXCLUDED.description, deleted_at = NULL;
        SELECT setval('ticket_categories_id_seq', (SELECT COALESCE(MAX(id), 1) FROM ticket_categories));

        DELETE FROM departments WHERE id > 5;
        UPDATE departments SET deleted_at = NULL WHERE id <= 5;
        SELECT setval('departments_id_seq', (SELECT COALESCE(MAX(id), 1) FROM departments));

        DELETE FROM ticket_priorities WHERE id > 4;
        UPDATE ticket_priorities SET deleted_at = NULL WHERE id <= 4;
        SELECT setval('ticket_priorities_id_seq', (SELECT COALESCE(MAX(id), 1) FROM ticket_priorities));

        DELETE FROM knowledge_categories WHERE id > 5;
        UPDATE knowledge_categories SET deleted_at = NULL WHERE id <= 5;
        SELECT setval('knowledge_categories_id_seq', (SELECT COALESCE(MAX(id), 1) FROM knowledge_categories));
    "#;

    sqlx::raw_sql(reset_query)
        .execute(pool)
        .await
        .expect("Gagal mereset database pengujian ke kondisi bersih");
}
