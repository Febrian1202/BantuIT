use crate::domain::{
    enums::{RoleName, TicketActor},
    models::{Ticket, User},
};

pub fn resolve_ticket_actors(ticket: &Ticket, user: &User) -> Vec<TicketActor> {
    let mut roles: Vec<TicketActor> = Vec::new();

    if user.role_name.is_admin() {
        roles.push(TicketActor::Admin);
    }

    if user.role_name.is_manager() {
        roles.push(TicketActor::Manager);
    }

    if user.role_name.is_technician() && ticket.technician_id == Some(user.id) {
        roles.push(TicketActor::Technician);
    }

    if user.role_name.is_technician() {
        roles.push(TicketActor::AnyTechnician);
    }

    if ticket.reporter_id == user.id
        && !user.role_name.is_admin()
        && !user
            .role_name
            .has_role(&[RoleName::Manager, RoleName::Technician])
    {
        roles.push(TicketActor::Reporter);
    }

    roles
}
