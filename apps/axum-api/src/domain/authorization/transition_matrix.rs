use crate::domain::enums::{TicketActor, TicketStatusName};

pub struct TicketTransitionMatrix;

impl TicketTransitionMatrix {
    pub fn allowed_actors(from: TicketStatusName, to: TicketStatusName) -> &'static [TicketActor] {
        match (from, to) {
            // from OPEN
            (TicketStatusName::Open, TicketStatusName::Assigned) => {
                &[TicketActor::Manager, TicketActor::Admin]
            }
            (TicketStatusName::Open, TicketStatusName::InProgress) => &[
                TicketActor::AnyTechnician,
                TicketActor::Manager,
                TicketActor::Admin,
            ],
            (TicketStatusName::Open, TicketStatusName::Closed) => {
                &[TicketActor::Manager, TicketActor::Admin]
            }

            // from ASSIGNED
            (TicketStatusName::Assigned, TicketStatusName::Open) => {
                &[TicketActor::Admin, TicketActor::Manager]
            }
            (TicketStatusName::Assigned, TicketStatusName::InProgress) => &[
                TicketActor::Technician,
                TicketActor::Manager,
                TicketActor::Admin,
            ],
            (TicketStatusName::Assigned, TicketStatusName::Closed) => {
                &[TicketActor::Manager, TicketActor::Admin]
            }

            // from IN_PROGRESS
            (TicketStatusName::InProgress, TicketStatusName::Assigned) => {
                &[TicketActor::Manager, TicketActor::Admin]
            }
            (TicketStatusName::InProgress, TicketStatusName::Resolved) => &[
                TicketActor::Technician,
                TicketActor::Manager,
                TicketActor::Admin,
            ],
            (TicketStatusName::InProgress, TicketStatusName::Closed) => {
                &[TicketActor::Manager, TicketActor::Admin]
            }

            // from RESOLVED
            (TicketStatusName::Resolved, TicketStatusName::InProgress) => &[
                TicketActor::Reporter,
                TicketActor::Technician,
                TicketActor::Manager,
                TicketActor::Admin,
            ],
            (TicketStatusName::Resolved, TicketStatusName::Closed) => &[
                TicketActor::Reporter,
                TicketActor::Manager,
                TicketActor::Admin,
            ],

            // from CLOSED
            (TicketStatusName::Closed, _) => &[],
            _ => &[],
        }
    }

    pub fn allows(from: TicketStatusName, to: TicketStatusName, actors: &[TicketActor]) -> bool {
        if from == to {
            return false;
        }
        let allowed = Self::allowed_actors(from, to);
        actors.iter().any(|actor| allowed.contains(actor))
    }
}
