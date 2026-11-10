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

    pub const POLICY_ABILITES: &'static [(Ability, &'static [RoleName])] = &[
        // Ticket (13 Abilities)
        (
            Ability::TicketViewAny,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::TicketView,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::TicketCreate,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::TicketUpdate,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (Ability::TicketDelete, &[RoleName::Administrator]),
        (
            Ability::TicketAssign,
            &[RoleName::Administrator, RoleName::Manager],
        ),
        (
            Ability::TicketUnassign,
            &[RoleName::Administrator, RoleName::Manager],
        ),
        (
            Ability::TicketChangeStatus,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::TicketSelfAssign,
            &[RoleName::Administrator, RoleName::Technician],
        ),
        (
            Ability::TicketChangePriority,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        (
            Ability::TicketComment,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::TicketViewHistory,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::TicketAttach,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        // Attachment 4 (Abilities)
        (
            Ability::AttachmentView,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::AttachmentCreate,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::AttachmentDownload,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::AttachmentDelete,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        // Asset (10 Abilities)
        (
            Ability::AssetViewAny,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        (
            Ability::AssetView,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        (
            Ability::AssetViewOwn,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::AssetViewAssignable,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::AssetCreate,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        (
            Ability::AssetUpdate,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        (
            Ability::AssetDelete,
            &[RoleName::Administrator, RoleName::Manager],
        ),
        (
            Ability::AssetAssign,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        (
            Ability::AssetRelease,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        (
            Ability::AssetViewHistory,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        // Article (7 Abilities)
        (
            Ability::ArticleViewAny,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::ArticleView,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::ArticleCreate,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        (
            Ability::ArticleUpdate,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        (
            Ability::ArticlePublish,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        (
            Ability::ArticleUnpublish,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        (
            Ability::ArticleDelete,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
            ],
        ),
        // Notification (3 Abilites)
        (
            Ability::NotificationViewAny,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::NotificationMarkAsRead,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
        ),
        (
            Ability::NotificationMarkAllAsRead,
            &[
                RoleName::Administrator,
                RoleName::Manager,
                RoleName::Technician,
                RoleName::Employee,
            ],
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

        // Gate / Role Abilites
        for (ability, roles) in Self::ROLE_ABILITES {
            if roles.contains(&role) {
                let s: &'static str = (*ability).into();
                permissions.push(s);
            }
        }

        // Policy Abilities
        for (ability, roles) in Self::POLICY_ABILITES {
            if roles.contains(&role) {
                let s: &'static str = (*ability).into();
                permissions.push(s);
            }
        }
        permissions
    }
}
