use crate::domain::repositories::ticket::DynTicketPriorityRepo;

#[derive(Clone)]
pub struct TicketPriorityService {
    priority_repo: DynTicketPriorityRepo,
}

impl TicketPriorityService {
    pub fn new(priority_repo: DynTicketPriorityRepo) -> Self {
        Self { priority_repo }
    }
}
