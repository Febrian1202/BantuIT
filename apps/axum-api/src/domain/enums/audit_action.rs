use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "varchar")]
pub enum AuditAction {
    Create,
    Update,
    Delete,
    Assign,
    Reassign,
    Unassign,
    SelfAssign,
    StatusChange,
    PriorityChange,
    Reopen,
    Resolve,
    Close,
    Cancel,
    Login,
    Logout,
    PasswordReset,
    SlaBreach,
    Release,
    Publish,
    Unpublish,
    Activate,
    Deactivate,
}
