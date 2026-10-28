use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "varchar")]
pub enum RoleName {
    Administrator,
    Manager,
    Technician,
    Employee,
}

impl RoleName {
    pub fn has_role(&self, roles: &[RoleName]) -> bool {
        roles.contains(self)
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, RoleName::Administrator)
    }

    pub fn is_manager(&self) -> bool {
        matches!(self, RoleName::Manager)
    }

    pub fn is_technician(&self) -> bool {
        matches!(self, RoleName::Technician)
    }

    pub fn is_employee(&self) -> bool {
        matches!(self, RoleName::Employee)
    }
}
