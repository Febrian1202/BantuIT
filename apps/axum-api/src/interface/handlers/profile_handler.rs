use axum::extract::State;

use crate::{
    application::dtos::{
        MeResponse, UpdatePasswordRequest, UpdatePasswordResponse, UpdateProfileRequest,
    },
    common::{ApiResponse, ApiResult},
    interface::{
        AppState,
        extractors::{auth_user::AuthUser, validated_json::ValidatedJson},
    },
};

pub async fn update_password(
    State(state): State<AppState>,
    auth: AuthUser,
    ValidatedJson(payload): ValidatedJson<UpdatePasswordRequest>,
) -> ApiResult<UpdatePasswordResponse> {
    state.profile_service.update_password(auth, payload).await?;
    let response = UpdatePasswordResponse;
    Ok(ApiResponse::success(
        response,
        "Password updated successfully.",
    ))
}

pub async fn show(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
) -> ApiResult<MeResponse> {
    let response = state.profile_service.show(user).await?;
    Ok(ApiResponse::success(
        response,
        "Profile retrieved successfully.",
    ))
}

pub async fn update(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
    ValidatedJson(payload): ValidatedJson<UpdateProfileRequest>,
) -> ApiResult<MeResponse> {
    let response = state.profile_service.update(user, payload).await?;

    Ok(ApiResponse::success(
        response,
        "Profile updated successfully.",
    ))
}
