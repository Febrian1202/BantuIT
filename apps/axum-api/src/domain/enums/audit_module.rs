use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "varchar")]
pub enum AuditModule {
    Ticket,
    Asset,
    Article,
    User,
    Role,
    Department,
    TicketCategory,
    TicketPriority,
    Auth,
    KnowledgeCategory,
}
