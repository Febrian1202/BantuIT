use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, IntoStaticStr, PartialOrd, Ord,
)]
pub enum Ability {
    // Autentikasi dan Profil
    #[serde(rename = "auth.login")]
    #[strum(serialize = "auth.login")]
    AuthLogin,

    #[serde(rename = "auth.logout")]
    #[strum(serialize = "auth.logout")]
    AuthLogout,

    #[serde(rename = "profile.view-own")]
    #[strum(serialize = "profile.view-own")]
    ProfileViewOwn,

    #[serde(rename = "profile.change-password")]
    #[strum(serialize = "profile.change-password")]
    ProfileChangePassword,

    // Dashboard
    #[serde(rename = "dashboard.employee")]
    #[strum(serialize = "dashboard.employee")]
    DashboardEmployee,

    #[serde(rename = "dashboard.technician")]
    #[strum(serialize = "dashboard.technician")]
    DashboardTechnician,

    #[serde(rename = "dashboard.manager")]
    #[strum(serialize = "dashboard.manager")]
    DashboardManager,

    #[serde(rename = "dashboard.admin")]
    #[strum(serialize = "dashboard.admin")]
    DashboardAdmin,

    #[serde(rename = "analytics.technician-performance")]
    #[strum(serialize = "analytics.technician-performance")]
    AnalyticsTechnicianPerformance,

    // Administrasi
    #[serde(rename = "user.viewAny")]
    #[strum(serialize = "user.viewAny")]
    UserViewAny,

    #[serde(rename = "user.view")]
    #[strum(serialize = "user.view")]
    UserView,

    #[serde(rename = "user.create")]
    #[strum(serialize = "user.create")]
    UserCreate,

    #[serde(rename = "user.update")]
    #[strum(serialize = "user.update")]
    UserUpdate,

    #[serde(rename = "user.delete")]
    #[strum(serialize = "user.delete")]
    UserDelete,

    #[serde(rename = "user.activate")]
    #[strum(serialize = "user.activate")]
    UserActivate,

    #[serde(rename = "user.deactivate")]
    #[strum(serialize = "user.deactivate")]
    UserDeactivate,

    #[serde(rename = "user.reset-password")]
    #[strum(serialize = "user.reset-password")]
    UserResetPassword,

    #[serde(rename = "user.lookup")]
    #[strum(serialize = "user.lookup")]
    UserLookup,

    #[serde(rename = "technician.list")]
    #[strum(serialize = "technician.list")]
    TechnicianList,

    #[serde(rename = "department.viewAny")]
    #[strum(serialize = "department.viewAny")]
    DepartmentViewAny,

    #[serde(rename = "department.manage")]
    #[strum(serialize = "department.manage")]
    DepartmentManage,

    #[serde(rename = "ticket-category.viewAny")]
    #[strum(serialize = "ticket-category.viewAny")]
    TicketCategoryViewAny,

    #[serde(rename = "ticket-category.manage")]
    #[strum(serialize = "ticket-category.manage")]
    TicketCategoryManage,

    #[serde(rename = "ticket-priority.viewAny")]
    #[strum(serialize = "ticket-priority.viewAny")]
    TicketPriorityViewAny,

    #[serde(rename = "ticket-priority.manage")]
    #[strum(serialize = "ticket-priority.manage")]
    TicketPriorityManage,

    #[serde(rename = "ticket-status.viewAny")]
    #[strum(serialize = "ticket-status.viewAny")]
    TicketStatusViewAny,

    #[serde(rename = "knowledge-category.viewAny")]
    #[strum(serialize = "knowledge-category.viewAny")]
    KnowledgeCategoryViewAny,

    #[serde(rename = "knowledge-category.manage")]
    #[strum(serialize = "knowledge-category.manage")]
    KnowledgeCategoryManage,

    #[serde(rename = "audit-log.viewAny")]
    #[strum(serialize = "audit-log.viewAny")]
    AuditLogViewAny,

    #[serde(rename = "audit-log.view")]
    #[strum(serialize = "audit-log.view")]
    AuditLogView,

    /// Notifikasi
    #[serde(rename = "notification.viewAny")]
    #[strum(serialize = "notification.viewAny")]
    NotificationViewAny,

    #[serde(rename = "notification.markAsRead")]
    #[strum(serialize = "notification.markAsRead")]
    NotificationMarkAsRead,

    #[serde(rename = "notification.markAllAsRead")]
    #[strum(serialize = "notification.markAllAsRead")]
    NotificationMarkAllAsRead,

    // Ticket
    #[serde(rename = "ticket.viewAny")]
    #[strum(serialize = "ticket.viewAny")]
    TicketViewAny,
    #[serde(rename = "ticket.view")]
    #[strum(serialize = "ticket.view")]
    TicketView,
    #[serde(rename = "ticket.create")]
    #[strum(serialize = "ticket.create")]
    TicketCreate,
    #[serde(rename = "ticket.update")]
    #[strum(serialize = "ticket.update")]
    TicketUpdate,
    #[serde(rename = "ticket.delete")]
    #[strum(serialize = "ticket.delete")]
    TicketDelete,
    #[serde(rename = "ticket.assign")]
    #[strum(serialize = "ticket.assign")]
    TicketAssign,
    #[serde(rename = "ticket.unassign")]
    #[strum(serialize = "ticket.unassign")]
    TicketUnassign,
    #[serde(rename = "ticket.changeStatus")]
    #[strum(serialize = "ticket.changeStatus")]
    TicketChangeStatus,
    #[serde(rename = "ticket.selfAssign")]
    #[strum(serialize = "ticket.selfAssign")]
    TicketSelfAssign,
    #[serde(rename = "ticket.changePriority")]
    #[strum(serialize = "ticket.changePriority")]
    TicketChangePriority,
    #[serde(rename = "ticket.comment")]
    #[strum(serialize = "ticket.comment")]
    TicketComment,
    #[serde(rename = "ticket.viewHistory")]
    #[strum(serialize = "ticket.viewHistory")]
    TicketViewHistory,
    #[serde(rename = "ticket.attach")]
    #[strum(serialize = "ticket.attach")]
    TicketAttach,

    // Attachment
    #[serde(rename = "attachment.view")]
    #[strum(serialize = "attachment.view")]
    AttachmentView,
    #[serde(rename = "attachment.download")]
    #[strum(serialize = "attachment.download")]
    AttachmentDownload,
    #[serde(rename = "attachment.create")]
    #[strum(serialize = "attachment.create")]
    AttachmentCreate,
    #[serde(rename = "attachment.delete")]
    #[strum(serialize = "attachment.delete")]
    AttachmentDelete,

    // Asset
    #[serde(rename = "asset.viewAny")]
    #[strum(serialize = "asset.viewAny")]
    AssetViewAny,
    #[serde(rename = "asset.view")]
    #[strum(serialize = "asset.view")]
    AssetView,
    #[serde(rename = "asset.viewOwn")]
    #[strum(serialize = "asset.viewOwn")]
    AssetViewOwn,
    #[serde(rename = "asset.viewAssignable")]
    #[strum(serialize = "asset.viewAssignable")]
    AssetViewAssignable,
    #[serde(rename = "asset.create")]
    #[strum(serialize = "asset.create")]
    AssetCreate,
    #[serde(rename = "asset.update")]
    #[strum(serialize = "asset.update")]
    AssetUpdate,
    #[serde(rename = "asset.delete")]
    #[strum(serialize = "asset.delete")]
    AssetDelete,
    #[serde(rename = "asset.assign")]
    #[strum(serialize = "asset.assign")]
    AssetAssign,
    #[serde(rename = "asset.release")]
    #[strum(serialize = "asset.release")]
    AssetRelease,
    #[serde(rename = "asset.viewHistory")]
    #[strum(serialize = "asset.viewHistory")]
    AssetViewHistory,

    // Article
    #[serde(rename = "article.viewAny")]
    #[strum(serialize = "article.viewAny")]
    ArticleViewAny,
    #[serde(rename = "article.view")]
    #[strum(serialize = "article.view")]
    ArticleView,
    #[serde(rename = "article.create")]
    #[strum(serialize = "article.create")]
    ArticleCreate,
    #[serde(rename = "article.update")]
    #[strum(serialize = "article.update")]
    ArticleUpdate,
    #[serde(rename = "article.publish")]
    #[strum(serialize = "article.publish")]
    ArticlePublish,
    #[serde(rename = "article.unpublish")]
    #[strum(serialize = "article.unpublish")]
    ArticleUnpublish,
    #[serde(rename = "article.delete")]
    #[strum(serialize = "article.delete")]
    ArticleDelete,
}
