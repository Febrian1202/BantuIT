use axum::extract::State;

use crate::{
    application::dtos::{UpdatePasswordRequest, UpdatePasswordResponse},
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
