use axum::extract::State;

use crate::{
    application::dtos::TicketCategoryResponse,
    common::{ApiResponse, ApiResult},
    domain::{authorization::gate::Gate, enums::Ability},
    interface::{AppState, extractors::auth_user::AuthUser},
};

pub async fn index(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
) -> ApiResult<Vec<TicketCategoryResponse>> {
    Gate::check(&user, Ability::TicketCategoryViewAny)?;

    let response = state.ticket_category_service.index().await?;

    Ok(ApiResponse::success(
        response,
        "Ticket categories retrieved.",
    ))
}
