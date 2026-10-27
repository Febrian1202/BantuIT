use std::collections::HashMap;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::common::AppError;

pub type ApiResult<T> = Result<(StatusCode, Json<ApiResponse<T>>), AppError>;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<HashMap<String, Vec<String>>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PaginationMeta {
    pub current_page: i64,
    pub per_page: i64,
    pub total: i64,
    pub last_page: i64,
    pub from: Option<i64>,
    pub to: Option<i64>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T, message: impl Into<String>) -> (StatusCode, Json<Self>) {
        (
            StatusCode::OK,
            Json(Self {
                success: true,
                message: message.into(),
                data: Some(data),
                meta: None,
                errors: None,
            }),
        )
    }

    pub fn created(data: T, message: impl Into<String>) -> (StatusCode, Json<Self>) {
        (
            StatusCode::CREATED,
            Json(Self {
                success: true,
                message: message.into(),
                data: Some(data),
                meta: None,
                errors: None,
            }),
        )
    }

    pub fn paginated(
        data: T,
        meta: PaginationMeta,
        message: impl Into<String>,
    ) -> (StatusCode, Json<Self>) {
        (
            StatusCode::OK,
            Json(Self {
                success: true,
                message: message.into(),
                data: Some(data),
                meta: Some(meta),
                errors: None,
            }),
        )
    }

    pub fn with_status_code(
        data: T,
        message: impl Into<String>,
        status: StatusCode,
        success: bool,
    ) -> (StatusCode, Json<Self>) {
        (
            status,
            Json(Self {
                success,
                message: message.into(),
                data: Some(data),
                meta: None,
                errors: None,
            }),
        )
    }
}

impl ApiResponse<()> {
    pub fn error(
        status: StatusCode,
        message: impl Into<String>,
        error: Option<HashMap<String, Vec<String>>>,
    ) -> Response {
        let body = Self {
            success: false,
            message: message.into(),
            data: None,
            meta: None,
            errors: error,
        };

        (status, Json(body)).into_response()
    }
}
