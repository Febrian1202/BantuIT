use crate::common::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, sqlx::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE", type_name = "varchar")]
pub enum TicketStatusName {
    Open,
    Assigned,
    InProgress,
    Resolved,
    Closed,
}

// 1. Konversi dari i32 ke Enum menggunakan TryFrom
impl TryFrom<i32> for TicketStatusName {
    type Error = AppError;

    fn try_from(id: i32) -> Result<Self, Self::Error> {
        match id {
            1 => Ok(Self::Open),
            2 => Ok(Self::Assigned),
            3 => Ok(Self::InProgress),
            4 => Ok(Self::Resolved),
            5 => Ok(Self::Closed),
            _ => Err(AppError::ValueError(format!("Invalid status ID: {}", id))),
        }
    }
}

// 2. Konversi dari Enum ke i32 menggunakan From
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
    // Helper instan untuk ID jika tetap ingin panggil .id()
    pub fn id(self) -> i32 {
        self.into()
    }

    // Helper konversi jika tetap ingin panggil TicketStatusName::from_id(id)
    pub fn from_id(id: i32) -> Result<Self, AppError> {
        Self::try_from(id)
    }

    // Menggunakan macro matches! yang ringkas
    pub fn is_closed(self) -> bool {
        matches!(self, Self::Closed | Self::Resolved)
    }

    pub fn is_final(self) -> bool {
        matches!(self, Self::Closed)
    }

    // Exhaustive matching tanpa wildcard `_`
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "OPEN",
            Self::Assigned => "ASSIGNED",
            Self::InProgress => "IN_PROGRESS",
            Self::Resolved => "RESOLVED",
            Self::Closed => "CLOSED",
        }
    }

    pub fn label(self) -> &'static str {
        self.as_str()
    }
}
