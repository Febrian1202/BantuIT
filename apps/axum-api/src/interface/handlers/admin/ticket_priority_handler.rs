use axum::extract::{Path, State};

use crate::{
    application::dtos::{
        CreateTicketPriorityRequest, DeleteTicketPriorityResponse, TicketPriorityResponse,
        UpdateTicketPriorityRequest,
    },
    common::{ApiResponse, ApiResult},
    domain::{
        authorization::gate::Gate,
        enums::Ability::{self, TicketPriorityManage, TicketPriorityViewAny},
    },
    interface::{
        AppState,
        extractors::{AuthUser, ClientInfo, ValidatedJson},
    },
};

pub async fn get_priorities(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
) -> ApiResult<Vec<TicketPriorityResponse>> {
    Gate::check(&user, TicketPriorityViewAny)?;
    let priorities = state.ticket_priority_service.get_priorities().await?;
    Ok(ApiResponse::success(
        priorities
            .into_iter()
            .map(TicketPriorityResponse::from)
            .collect(),
        "Ticket priorities retrieved.",
    ))
}

pub async fn show_priority(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
    Path(id): Path<i32>,
) -> ApiResult<TicketPriorityResponse> {
    Gate::check(&user, Ability::TicketPriorityViewAny)?;
    let priority = state.ticket_priority_service.show_priority(id).await?;
    Ok(ApiResponse::success(
        TicketPriorityResponse::from(priority),
        "Ticket priority retrieved.",
    ))
}

pub async fn create_priority(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
    client: ClientInfo,
    ValidatedJson(payload): ValidatedJson<CreateTicketPriorityRequest>,
) -> ApiResult<TicketPriorityResponse> {
    Gate::check(&user, Ability::TicketPriorityManage)?;
    let priority = state
        .ticket_priority_service
        .create_priority(payload, user, client)
        .await?;
    Ok(ApiResponse::created(
        TicketPriorityResponse::from(priority),
        "Ticket priority created successfully.",
    ))
}

pub async fn update_priority(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
    Path(id): Path<i32>,
    client: ClientInfo,
    ValidatedJson(payload): ValidatedJson<UpdateTicketPriorityRequest>,
) -> ApiResult<TicketPriorityResponse> {
    Gate::check(&user, TicketPriorityManage)?;
    let priority = state
        .ticket_priority_service
        .update_priority(id, payload, user, client)
        .await?;
    Ok(ApiResponse::success(
        TicketPriorityResponse::from(priority),
        "Ticket priority updated successfully.",
    ))
}

pub async fn delete_priority(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
    Path(id): Path<i32>,
    client: ClientInfo,
) -> ApiResult<DeleteTicketPriorityResponse> {
    Gate::check(&user, TicketPriorityManage)?;
    state
        .ticket_priority_service
        .delete_priority(id, user, client)
        .await?;
    Ok(ApiResponse::success(
        DeleteTicketPriorityResponse,
        "Ticket priority deleted successfully.",
    ))
}
