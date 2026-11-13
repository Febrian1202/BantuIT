use bantuit::{application::services::guard::ReferentialIntegrityGuard, common::AppError};

#[test]
fn test_assert_unreferenced_success_when_all_zero() {
    let refs = [("Kategori tiket", 0), ("Kategori tiket (induk)", 0)];
    let result = ReferentialIntegrityGuard::assert_unreferenced(&refs);
    assert!(result.is_ok());
}

#[test]
fn test_assert_unreferenced_conflict_when_ticket_referenced() {
    let refs = [("Kategori tiket", 3), ("Kategori tiket (induk)", 0)];
    let result = ReferentialIntegrityGuard::assert_unreferenced(&refs);
    match result {
        Err(AppError::StateConflict(msg)) => {
            assert_eq!(
                msg,
                "Kategori tiket masih digunakan oleh 3 data terkait dan tidak dapat dihapus."
            );
        }
        _ => panic!("Expected StateConflict error, got {:?}", result),
    }
}

#[test]
fn test_assert_unreferenced_conflict_when_child_referenced() {
    let refs = [("Kategori tiket", 0), ("Kategori tiket (induk)", 1)];
    let result = ReferentialIntegrityGuard::assert_unreferenced(&refs);
    match result {
        Err(AppError::StateConflict(msg)) => {
            assert_eq!(
                msg,
                "Kategori tiket (induk) masih digunakan oleh 1 data terkait dan tidak dapat dihapus."
            );
        }
        _ => panic!("Expected StateConflict error, got {:?}", result),
    }
}
