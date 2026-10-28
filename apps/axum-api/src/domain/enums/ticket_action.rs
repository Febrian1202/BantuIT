use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TicketAction {
    Assign,
    Unassign,
    Start,
    Resolve,
    Close,
    Cancel,
    Reopen,
    ChangePriority,
    Comment,
    Attach,
    Edit,
}
