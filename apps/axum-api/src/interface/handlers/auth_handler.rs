use axum::extract::State;

use crate::{
    application::dtos::{LoginRequest, LoginResponse},
    common::{ApiResponse, ApiResult},
    interface::{AppState, extractors::validated_json::ValidatedJson},
};

pub async fn login(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> ApiResult<LoginResponse> {
    let response = state.auth_service.login(payload).await?;

    Ok(ApiResponse::success(
        response,
        "Login successful.".to_string(),
    ))
}
