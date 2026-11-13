use axum::extract::{Path, State};

use crate::{
    application::dtos::{
        CreateTicketCategoryRequest, DeleteTicketCategoryResponse, TicketCategoryResponse,
        UpdateTicketCategoryRequest,
    },
    common::{ApiResponse, ApiResult},
    domain::{authorization::gate::Gate, enums::Ability},
    interface::{
        AppState,
        extractors::{ClientInfo, ValidatedJson, auth_user::AuthUser},
    },
};

pub async fn get_categories(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
) -> ApiResult<Vec<TicketCategoryResponse>> {
    Gate::check(&user, Ability::TicketCategoryViewAny)?;
    let response = state.ticket_category_service.get_categories().await?;
    Ok(ApiResponse::success(
        response,
        "Ticket categories retrieved.",
    ))
}

pub async fn show_category(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
    Path(id): Path<i32>,
) -> ApiResult<TicketCategoryResponse> {
    Gate::check(&user, Ability::TicketCategoryViewAny)?;
    let response = state.ticket_category_service.show_by_id(id).await?;
    Ok(ApiResponse::success(response, "Ticket category retrieved."))
}

pub async fn create_category(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
    client: ClientInfo,
    ValidatedJson(payload): ValidatedJson<CreateTicketCategoryRequest>,
) -> ApiResult<TicketCategoryResponse> {
    Gate::check(&user, Ability::TicketCategoryManage)?;
    let response = state
        .ticket_category_service
        .create_category(payload, user, client)
        .await?;
    Ok(ApiResponse::created(
        response,
        "Ticket category created successfully.",
    ))
}

pub async fn update_category(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
    client: ClientInfo,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTicketCategoryRequest>,
) -> ApiResult<TicketCategoryResponse> {
    Gate::check(&user, Ability::TicketCategoryManage)?;
    let response = state
        .ticket_category_service
        .update_category(id, payload, user, client)
        .await?;
    Ok(ApiResponse::success(
        response,
        "Ticket category updated successfully.",
    ))
}

pub async fn delete_category(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
    client: ClientInfo,
    Path(id): Path<i32>,
) -> ApiResult<DeleteTicketCategoryResponse> {
    Gate::check(&user, Ability::TicketCategoryManage)?;
    state
        .ticket_category_service
        .delete_category(id, user, client)
        .await?;
    let response = DeleteTicketCategoryResponse;
    Ok(ApiResponse::success(
        response,
        "Ticket category deleted successfully.",
    ))
}
