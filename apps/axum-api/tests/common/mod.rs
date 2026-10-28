use bantuit::domain::{
    enums::{RoleName, TicketStatusName, UserStatus},
    models::{Ticket, User},
};
use chrono::Utc;
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
