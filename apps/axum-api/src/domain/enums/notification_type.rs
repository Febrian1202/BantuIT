use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE", type_name = "varchar")]
pub enum NotificationType {
    TicketAssigned,
    TicketReassigned,
    TicketUnassigned,
    TicketStatusChanged,
    TicketSelfAssigned,
    TicketReopened,
    TicketResolved,
    TicketClosed,
    TicketCancelled,
    TicketCommented,
    TicketSlaBreached,
}
