use crate::common::AppError;
use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, sqlx::Type, IntoStaticStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TicketStatusName {
    Open,
    Assigned,
    InProgress,
    Resolved,
    Closed,
}

impl TryFrom<i32> for TicketStatusName {
    type Error = AppError;

    fn try_from(id: i32) -> Result<Self, Self::Error> {
        match id {
            1 => Ok(Self::Open),
            2 => Ok(Self::Assigned),
            3 => Ok(Self::InProgress),
            4 => Ok(Self::Resolved),
            5 => Ok(Self::Closed),
            _ => Err(AppError::value_error(format!("Invalid status ID: {id}"))),
        }
    }
}

impl From<TicketStatusName> for i32 {
    fn from(status: TicketStatusName) -> Self {
        match status {
            TicketStatusName::Open => 1,
            TicketStatusName::Assigned => 2,
            TicketStatusName::InProgress => 3,
            TicketStatusName::Resolved => 4,
            TicketStatusName::Closed => 5,
        }
    }
}

impl TicketStatusName {
    pub fn id(self) -> i32 {
        self as i32
    }

    pub fn from_id(id: i32) -> Result<Self, AppError> {
        Self::try_from(id)
    }

    pub fn is_closed(self) -> bool {
        matches!(self, Self::Closed | Self::Resolved)
    }

    pub fn is_final(self) -> bool {
        matches!(self, Self::Closed)
    }

    // Menggunakan IntoStaticStr bawaan strum (Zero Allocation)
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    pub fn label(self) -> &'static str {
        self.as_str()
    }
}
