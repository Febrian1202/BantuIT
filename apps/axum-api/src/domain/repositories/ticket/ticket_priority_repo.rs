use std::sync::Arc;

use async_trait::async_trait;

#[async_trait]
pub trait TicketPriorityRepo: Send + Sync {}

pub type DynTicketPriorityRepo = Arc<dyn TicketPriorityRepo>;
