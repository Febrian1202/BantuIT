use bantuit::domain::{
    authorization::transition_matrix::TicketTransitionMatrix,
    enums::{TicketActor, TicketStatusName},
};

#[test]
fn test_transition_from_open_status() {
    // Open -> Assigned hanya Manager dan Admin
    let actors =
        TicketTransitionMatrix::allowed_actors(TicketStatusName::Open, TicketStatusName::Assigned);
    assert_eq!(actors, &[TicketActor::Manager, TicketActor::Admin]);

    // Open -> InProgress oleh AnyTechnician, Manager, Admin
    let actors_in_progress = TicketTransitionMatrix::allowed_actors(
        TicketStatusName::Open,
        TicketStatusName::InProgress,
    );
    assert_eq!(
        actors_in_progress,
        &[
            TicketActor::AnyTechnician,
            TicketActor::Manager,
            TicketActor::Admin
        ]
    );

    // Reporter/Employee TIDAK boleh mengubah Open ke InProgress
    assert!(!TicketTransitionMatrix::allows(
        TicketStatusName::Open,
        TicketStatusName::InProgress,
        &[TicketActor::Reporter]
    ));
}

#[test]
fn test_transition_from_assigned_status() {
    // Assigned -> InProgress hanya teknisi tertugaskan, Manager, Admin
    let actors = TicketTransitionMatrix::allowed_actors(
        TicketStatusName::Assigned,
        TicketStatusName::InProgress,
    );
    assert_eq!(
        actors,
        &[
            TicketActor::Technician,
            TicketActor::Manager,
            TicketActor::Admin
        ]
    );

    // AnyTechnician (bukan teknisi yang ditugaskan) DITOLAK
    assert!(!TicketTransitionMatrix::allows(
        TicketStatusName::Assigned,
        TicketStatusName::InProgress,
        &[TicketActor::AnyTechnician]
    ));

    // Teknisi yang ditugaskan DIIZINKAN
    assert!(TicketTransitionMatrix::allows(
        TicketStatusName::Assigned,
        TicketStatusName::InProgress,
        &[TicketActor::Technician, TicketActor::AnyTechnician]
    ));
}

#[test]
fn test_transition_from_in_progress_status() {
    // InProgress -> Resolved oleh Teknisi tertugaskan, Manager, Admin
    let actors = TicketTransitionMatrix::allowed_actors(
        TicketStatusName::InProgress,
        TicketStatusName::Resolved,
    );
    assert!(actors.contains(&TicketActor::Technician));
    assert!(actors.contains(&TicketActor::Manager));
    assert!(actors.contains(&TicketActor::Admin));

    // Reporter TIDAK boleh menyelesaikan (resolve) tiket
    assert!(!TicketTransitionMatrix::allows(
        TicketStatusName::InProgress,
        TicketStatusName::Resolved,
        &[TicketActor::Reporter]
    ));
}

#[test]
fn test_transition_from_resolved_status() {
    // Resolved -> Closed oleh Reporter (konfirmasi selesai), Manager, atau Admin
    let actors = TicketTransitionMatrix::allowed_actors(
        TicketStatusName::Resolved,
        TicketStatusName::Closed,
    );
    assert_eq!(
        actors,
        &[
            TicketActor::Reporter,
            TicketActor::Manager,
            TicketActor::Admin
        ]
    );

    // Resolved -> InProgress (reopen) boleh oleh Reporter, Teknisi, Manager, Admin
    let reopen_actors = TicketTransitionMatrix::allowed_actors(
        TicketStatusName::Resolved,
        TicketStatusName::InProgress,
    );
    assert!(reopen_actors.contains(&TicketActor::Reporter));
    assert!(reopen_actors.contains(&TicketActor::Technician));
}

#[test]
fn test_closed_status_is_terminal_and_disallows_all_transitions() {
    // Closed -> Any status harus menghasilkan daftar kosong
    assert_eq!(
        TicketTransitionMatrix::allowed_actors(TicketStatusName::Closed, TicketStatusName::Open),
        &[]
    );
    assert_eq!(
        TicketTransitionMatrix::allowed_actors(
            TicketStatusName::Closed,
            TicketStatusName::InProgress
        ),
        &[]
    );

    // Tidak ada aktor (termasuk Admin) yang bisa mengubah tiket berstatus CLOSED
    assert!(!TicketTransitionMatrix::allows(
        TicketStatusName::Closed,
        TicketStatusName::Open,
        &[TicketActor::Admin, TicketActor::Manager]
    ));
}

#[test]
fn test_same_status_transition_is_strictly_disallowed() {
    // Transisi ke status yang sama (misal Open -> Open) harus false
    assert!(!TicketTransitionMatrix::allows(
        TicketStatusName::Open,
        TicketStatusName::Open,
        &[TicketActor::Admin]
    ));
    assert!(!TicketTransitionMatrix::allows(
        TicketStatusName::InProgress,
        TicketStatusName::InProgress,
        &[TicketActor::Technician]
    ));
}
