use axum::extract::State;

use crate::{
    application::dtos::{LoginRequest, LoginResponse, LogoutResponse},
    common::{ApiResponse, ApiResult},
    interface::{
        AppState,
        extractors::{auth_user::AuthUser, validated_json::ValidatedJson},
    },
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

pub async fn logout(
    State(state): State<AppState>,
    AuthUser(_, token_hash): AuthUser,
) -> ApiResult<LogoutResponse> {
    state.auth_service.logout(token_hash).await?;

    let response = LogoutResponse;

    Ok(ApiResponse::success(response, "Logout successful."))
}
