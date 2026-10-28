use bantuit::domain::{
    authorization::actor_resolver::resolve_ticket_actors,
    enums::{RoleName, TicketActor, TicketStatusName},
};
use uuid::Uuid;

use crate::common::{create_mock_ticket, create_mock_user};

#[test]
fn test_admin_resolves_only_to_admin() {
    let admin = create_mock_user(RoleName::Administrator);
    let ticket = create_mock_ticket(Uuid::new_v4(), TicketStatusName::Open);

    let actors = resolve_ticket_actors(&ticket, &admin);

    assert_eq!(actors, vec![TicketActor::Admin]);
}

#[test]
fn test_manager_resolves_only_to_manager() {
    let manager = create_mock_user(RoleName::Manager);
    let ticket = create_mock_ticket(Uuid::new_v4(), TicketStatusName::Open);

    let actors = resolve_ticket_actors(&ticket, &manager);

    assert_eq!(actors, vec![TicketActor::Manager]);
}

#[test]
fn test_assigned_technician_resolves_to_technician_and_any_technician() {
    let technician = create_mock_user(RoleName::Technician);
    let mut ticket = create_mock_ticket(Uuid::new_v4(), TicketStatusName::Open);
    ticket.technician_id = Some(technician.id);

    let actors = resolve_ticket_actors(&ticket, &technician);

    assert_eq!(actors.len(), 2);
    assert!(actors.contains(&TicketActor::Technician));
    assert!(actors.contains(&TicketActor::AnyTechnician));
    assert_eq!(
        actors,
        vec![TicketActor::Technician, TicketActor::AnyTechnician]
    );
}

#[test]
fn test_non_assigned_technician_resolves_only_to_any_technician() {
    let technician = create_mock_user(RoleName::Technician);
    let other_technician = create_mock_user(RoleName::Technician);

    // Kasus 1: Tiket di-assign ke teknisi lain
    let mut ticket_with_other = create_mock_ticket(Uuid::new_v4(), TicketStatusName::Open);
    ticket_with_other.technician_id = Some(other_technician.id);

    let actors_with_other = resolve_ticket_actors(&ticket_with_other, &technician);
    assert_eq!(actors_with_other, vec![TicketActor::AnyTechnician]);

    // Kasus 2: Tiket belum di-assign ke siapapun (technician_id is None)
    let ticket_unassigned = create_mock_ticket(Uuid::new_v4(), TicketStatusName::Open);
    let actors_unassigned = resolve_ticket_actors(&ticket_unassigned, &technician);
    assert_eq!(actors_unassigned, vec![TicketActor::AnyTechnician]);
}

#[test]
fn test_employee_reporter_resolves_only_to_reporter() {
    let employee = create_mock_user(RoleName::Employee);
    let ticket = create_mock_ticket(employee.id, TicketStatusName::Open);

    let actors = resolve_ticket_actors(&ticket, &employee);

    assert_eq!(actors, vec![TicketActor::Reporter]);
}

#[test]
fn test_employee_non_reporter_resolves_to_empty() {
    let employee = create_mock_user(RoleName::Employee);
    let other_employee = create_mock_user(RoleName::Employee);
    let ticket = create_mock_ticket(other_employee.id, TicketStatusName::Open);

    let actors = resolve_ticket_actors(&ticket, &employee);

    assert!(actors.is_empty());
}

#[test]
fn test_admin_reporter_does_not_receive_reporter_actor() {
    let admin = create_mock_user(RoleName::Administrator);
    let ticket = create_mock_ticket(admin.id, TicketStatusName::Open);

    let actors = resolve_ticket_actors(&ticket, &admin);

    // Admin yang bertindak sebagai pembuat tiket tidak boleh mendapatkan peran Reporter
    assert_eq!(actors, vec![TicketActor::Admin]);
    assert!(!actors.contains(&TicketActor::Reporter));
}

#[test]
fn test_manager_reporter_does_not_receive_reporter_actor() {
    let manager = create_mock_user(RoleName::Manager);
    let ticket = create_mock_ticket(manager.id, TicketStatusName::Open);

    let actors = resolve_ticket_actors(&ticket, &manager);

    // Manager yang bertindak sebagai pembuat tiket tidak boleh mendapatkan peran Reporter
    assert_eq!(actors, vec![TicketActor::Manager]);
    assert!(!actors.contains(&TicketActor::Reporter));
}

#[test]
fn test_technician_reporter_does_not_receive_reporter_actor() {
    let technician = create_mock_user(RoleName::Technician);

    // Kasus 1: Teknisi pembuat tiket sekaligus pemegang tiket
    let mut assigned_ticket = create_mock_ticket(technician.id, TicketStatusName::Open);
    assigned_ticket.technician_id = Some(technician.id);

    let assigned_actors = resolve_ticket_actors(&assigned_ticket, &technician);
    assert_eq!(
        assigned_actors,
        vec![TicketActor::Technician, TicketActor::AnyTechnician]
    );
    assert!(!assigned_actors.contains(&TicketActor::Reporter));

    // Kasus 2: Teknisi pembuat tiket tetapi bukan pemegang tiket
    let mut unassigned_ticket = create_mock_ticket(technician.id, TicketStatusName::Open);
    unassigned_ticket.technician_id = Some(Uuid::new_v4());

    let unassigned_actors = resolve_ticket_actors(&unassigned_ticket, &technician);
    assert_eq!(unassigned_actors, vec![TicketActor::AnyTechnician]);
    assert!(!unassigned_actors.contains(&TicketActor::Reporter));
}

/// Data-driven matrix test menyelaraskan kombinasi status dan role setup
/// mirip dengan dataset `availableActionMatrix` pada `TicketActionResolverTest.php`.
#[test]
fn test_actor_resolution_matrix_across_all_statuses() {
    let statuses = [
        TicketStatusName::Open,
        TicketStatusName::Assigned,
        TicketStatusName::InProgress,
        TicketStatusName::Resolved,
        TicketStatusName::Closed,
    ];

    for status in statuses {
        // 1. Role: Admin
        {
            let admin = create_mock_user(RoleName::Administrator);
            let ticket = create_mock_ticket(Uuid::new_v4(), status);
            let actors = resolve_ticket_actors(&ticket, &admin);
            assert_eq!(
                actors,
                vec![TicketActor::Admin],
                "Gagal pada status {:?} untuk Admin",
                status
            );
        }

        // 2. Role: Manager
        {
            let manager = create_mock_user(RoleName::Manager);
            let ticket = create_mock_ticket(Uuid::new_v4(), status);
            let actors = resolve_ticket_actors(&ticket, &manager);
            assert_eq!(
                actors,
                vec![TicketActor::Manager],
                "Gagal pada status {:?} untuk Manager",
                status
            );
        }

        // 3. Role: Technician (as_technician: true / pemegang)
        {
            let tech = create_mock_user(RoleName::Technician);
            let mut ticket = create_mock_ticket(Uuid::new_v4(), status);
            ticket.technician_id = Some(tech.id);
            let actors = resolve_ticket_actors(&ticket, &tech);
            assert_eq!(
                actors,
                vec![TicketActor::Technician, TicketActor::AnyTechnician],
                "Gagal pada status {:?} untuk Technician pemegang",
                status
            );
        }

        // 4. Role: Technician (as_technician: false / bukan pemegang)
        {
            let tech = create_mock_user(RoleName::Technician);
            let mut ticket = create_mock_ticket(Uuid::new_v4(), status);
            ticket.technician_id = Some(Uuid::new_v4());
            let actors = resolve_ticket_actors(&ticket, &tech);
            assert_eq!(
                actors,
                vec![TicketActor::AnyTechnician],
                "Gagal pada status {:?} untuk Technician bukan pemegang",
                status
            );
        }

        // 5. Role: Employee (as_reporter: true)
        {
            let employee = create_mock_user(RoleName::Employee);
            let ticket = create_mock_ticket(employee.id, status);
            let actors = resolve_ticket_actors(&ticket, &employee);
            assert_eq!(
                actors,
                vec![TicketActor::Reporter],
                "Gagal pada status {:?} untuk Employee reporter",
                status
            );
        }

        // 6. Role: Employee (as_reporter: false)
        {
            let employee = create_mock_user(RoleName::Employee);
            let ticket = create_mock_ticket(Uuid::new_v4(), status);
            let actors = resolve_ticket_actors(&ticket, &employee);
            assert!(
                actors.is_empty(),
                "Gagal pada status {:?} untuk Employee bukan reporter (harus kosong)",
                status
            );
        }
    }
}
