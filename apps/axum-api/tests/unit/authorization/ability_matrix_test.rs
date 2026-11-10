use bantuit::domain::{
    authorization::AbilityMatrix,
    enums::{Ability, RoleName},
};

#[test]
fn test_administrator_has_exactly_67_permissions() {
    let permissions = AbilityMatrix::permission_for(RoleName::Administrator);

    // Paritas 1:1 terhadap Laravel ProfileTest (total 67 abilities)
    assert_eq!(permissions.len(), 67);

    // Memastikan gate abilities dan policy abilities utama tersedia
    assert!(permissions.contains(&"ticket.create"));
    assert!(permissions.contains(&"ticket.viewAny"));
    assert!(permissions.contains(&"ticket.delete"));
    assert!(permissions.contains(&"article.create"));
    assert!(permissions.contains(&"article.publish"));
    assert!(permissions.contains(&"asset.viewAny"));
    assert!(permissions.contains(&"asset.delete"));
    assert!(permissions.contains(&"dashboard.admin"));
    assert!(permissions.contains(&"user.lookup"));
    assert!(permissions.contains(&"user.viewAny"));
    assert!(permissions.contains(&"audit-log.viewAny"));
}

#[test]
fn test_manager_permissions_matrix() {
    let permissions = AbilityMatrix::permission_for(RoleName::Manager);

    assert!(permissions.contains(&"dashboard.manager"));
    assert!(permissions.contains(&"ticket.viewAny"));
    assert!(permissions.contains(&"ticket.assign"));
    assert!(permissions.contains(&"ticket.unassign"));
    assert!(permissions.contains(&"asset.viewAny"));
    assert!(permissions.contains(&"asset.delete"));
    assert!(permissions.contains(&"article.create"));
    assert!(permissions.contains(&"audit-log.viewAny"));

    // Manager TIDAK boleh memiliki selfAssign (hanya technician & admin) atau kelola user
    assert!(!permissions.contains(&"ticket.selfAssign"));
    assert!(!permissions.contains(&"user.viewAny"));
    assert!(!permissions.contains(&"dashboard.admin"));
}

#[test]
fn test_technician_permissions_matrix() {
    let permissions = AbilityMatrix::permission_for(RoleName::Technician);

    assert!(permissions.contains(&"dashboard.technician"));
    assert!(permissions.contains(&"ticket.viewAny"));
    assert!(permissions.contains(&"ticket.selfAssign"));
    assert!(permissions.contains(&"ticket.changeStatus"));
    assert!(permissions.contains(&"asset.viewAny"));
    assert!(permissions.contains(&"article.create"));

    // Teknisi TIDAK boleh menghapus aset atau menugaskan tiket ke orang lain
    assert!(!permissions.contains(&"asset.delete"));
    assert!(!permissions.contains(&"ticket.assign"));
    assert!(!permissions.contains(&"ticket.unassign"));
    assert!(!permissions.contains(&"dashboard.manager"));
}

#[test]
fn test_employee_permissions_matrix() {
    let permissions = AbilityMatrix::permission_for(RoleName::Employee);

    assert!(permissions.contains(&"dashboard.employee"));
    assert!(permissions.contains(&"ticket.create"));
    assert!(permissions.contains(&"ticket.viewAny"));
    assert!(permissions.contains(&"ticket.changeStatus"));
    assert!(permissions.contains(&"asset.viewOwn"));
    assert!(permissions.contains(&"asset.viewAssignable"));
    assert!(permissions.contains(&"article.viewAny"));

    // Employee TIDAK boleh melihat aset milik user lain atau membuat artikel KB
    assert!(!permissions.contains(&"asset.viewAny"));
    assert!(!permissions.contains(&"article.create"));
    assert!(!permissions.contains(&"user.viewAny"));
    assert!(!permissions.contains(&"dashboard.technician"));
}

#[test]
fn test_allows_role_evaluates_gate_rbac_strictly() {
    // Admin diizinkan pada Gate user.viewAny
    assert!(AbilityMatrix::allows_role(
        RoleName::Administrator,
        Ability::UserViewAny
    ));

    // Employee ditolak pada Gate user.viewAny
    assert!(!AbilityMatrix::allows_role(
        RoleName::Employee,
        Ability::UserViewAny
    ));

    // Semua role diizinkan melihat status tiket
    assert!(AbilityMatrix::allows_role(
        RoleName::Employee,
        Ability::TicketStatusViewAny
    ));
    assert!(AbilityMatrix::allows_role(
        RoleName::Technician,
        Ability::TicketStatusViewAny
    ));
}
