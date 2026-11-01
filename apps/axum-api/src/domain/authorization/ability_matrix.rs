use crate::domain::enums::{Ability, RoleName};

pub struct AbilityMatrix;

impl AbilityMatrix {
    pub const ROLE_ABILITES: &'static [(Ability, &'static [RoleName])] = &[
        (
            // -- Autentikasi dan Profil --
            Ability::AuthLogin,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::AuthLogout,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::ProfileViewOwn,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::ProfileChangePassword,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        // -- Dashboard --
        (
            Ability::DashboardEmployee,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::DashboardTechnician,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        (
            Ability::DashboardManager,
            &[RoleName::Administrator, RoleName::Manager],
        ),
        (Ability::DashboardAdmin, &[RoleName::Administrator]),
        (
            Ability::AnalyticsTechnicianPerformance,
            &[RoleName::Administrator, RoleName::Manager],
        ),
        // -- Administrasi --
        // - User
        (Ability::UserViewAny, &[RoleName::Administrator]),
        (Ability::UserView, &[RoleName::Administrator]),
        (Ability::UserCreate, &[RoleName::Administrator]),
        (Ability::UserUpdate, &[RoleName::Administrator]),
        (Ability::UserDelete, &[RoleName::Administrator]),
        (Ability::UserActivate, &[RoleName::Administrator]),
        (Ability::UserDeactivate, &[RoleName::Administrator]),
        (Ability::UserResetPassword, &[RoleName::Administrator]),
        (
            Ability::UserLookup,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        (
            Ability::TechnicianList,
            &[RoleName::Administrator, RoleName::Manager],
        ),
        // - Department
        (
            Ability::DepartmentViewAny,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (Ability::DepartmentManage, &[RoleName::Administrator]),
        // - Ticket
        (
            Ability::TicketCategoryViewAny,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (Ability::TicketCategoryManage, &[RoleName::Administrator]),
        (
            Ability::TicketPriorityViewAny,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (Ability::TicketPriorityManage, &[RoleName::Administrator]),
        (
            Ability::TicketStatusViewAny,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        // - Knowledge
        (
            Ability::KnowledgeCategoryViewAny,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (Ability::KnowledgeCategoryManage, &[RoleName::Administrator]),
        // - Audit log
        (
            Ability::AuditLogView,
            &[RoleName::Administrator, RoleName::Manager],
        ),
        (
            Ability::AuditLogViewAny,
            &[RoleName::Administrator, RoleName::Manager],
        ),
    ];

    pub const ADMIN_GATE_EXCEPTIONS: &'static [Ability] = &[
        Ability::NotificationViewAny,
        Ability::NotificationMarkAsRead,
        Ability::NotificationMarkAllAsRead,
        Ability::UserDeactivate,
        Ability::UserDelete,
    ];

    /// Cek apakah role tertentu memiliki hak aksespada ability tertentu
    pub fn allows_role(role: RoleName, ability: Ability) -> bool {
        for (ab, roles) in Self::ROLE_ABILITES {
            if *ab == ability && roles.contains(&role) {
                return true;
            }
        }
        false
    }

    /// Daftar seluruh permissions string untuk response profile pengguna (GET /api/me)
    pub fn permission_for(role: RoleName) -> Vec<&'static str> {
        let mut permissions: Vec<&'static str> = Vec::new();
        for (ability, roles) in Self::ROLE_ABILITES {
            if roles.contains(&role) {
                let s: &'static str = (*ability).into();
                permissions.push(s);
            }
        }
        permissions
    }
}
