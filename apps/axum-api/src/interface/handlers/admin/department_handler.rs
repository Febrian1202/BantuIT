use axum::extract::{Path, State};

use crate::{
    application::dtos::{
        CreateDepartmentRequest, DeleteDepartmentResponse, DepartmentResponse,
        UpdateDepartmentRequest,
    },
    common::{ApiResponse, ApiResult},
    domain::{authorization::gate::Gate, enums::Ability},
    interface::{
        AppState,
        extractors::{AuthUser, ClientInfo, ValidatedJson},
    },
};

pub async fn get_departments(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
) -> ApiResult<Vec<DepartmentResponse>> {
    Gate::check(&user, Ability::DepartmentViewAny)?;
    let departments = state.department_service.get_departments().await?;
    let response = departments
        .into_iter()
        .map(DepartmentResponse::from)
        .collect();
    Ok(ApiResponse::success(response, "Departments retrieved."))
}

pub async fn show_department(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
    Path(id): Path<i32>,
) -> ApiResult<DepartmentResponse> {
    Gate::check(&user, Ability::DepartmentViewAny)?;
    let department = state.department_service.show_department(id).await?;
    let response = DepartmentResponse::from(department);
    Ok(ApiResponse::success(response, "Department retrieved."))
}

pub async fn create_department(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
    client: ClientInfo,
    ValidatedJson(payload): ValidatedJson<CreateDepartmentRequest>,
) -> ApiResult<DepartmentResponse> {
    Gate::check(&user, Ability::DepartmentManage)?;
    let inserted = state
        .department_service
        .create_department(payload, user, client)
        .await?;
    let response = DepartmentResponse::from(inserted);
    Ok(ApiResponse::created(
        response,
        "Department created successfully.",
    ))
}

pub async fn update_department(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
    Path(id): Path<i32>,
    client: ClientInfo,
    ValidatedJson(payload): ValidatedJson<UpdateDepartmentRequest>,
) -> ApiResult<DepartmentResponse> {
    Gate::check(&user, Ability::DepartmentManage)?;
    let updated = state
        .department_service
        .update_department(id, payload, user, client)
        .await?;
    let response = DepartmentResponse::from(updated);

    Ok(ApiResponse::success(
        response,
        "Department updated successfully.",
    ))
}

pub async fn delete_department(
    State(state): State<AppState>,
    AuthUser(user, _): AuthUser,
    Path(id): Path<i32>,
    client: ClientInfo,
) -> ApiResult<DeleteDepartmentResponse> {
    Gate::check(&user, Ability::DepartmentManage)?;
    state
        .department_service
        .delete_department(id, user, client)
        .await?;
    let response = DeleteDepartmentResponse;
    Ok(ApiResponse::success(
        response,
        "Department deleted successfully.",
    ))
}
