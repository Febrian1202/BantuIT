use std::collections::HashMap;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::common::ApiResponse;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Unauthenticated.")]
    Unauthorized,

    #[error("Forbidden.")]
    Forbidden,

    #[error("Resource not found.")]
    NotFound,

    #[error("State conflict.")]
    StateConflict(String),

    #[error("The given data was invalid.")]
    ValidationError(HashMap<String, Vec<String>>),

    #[error("Password change required.")]
    PasswordChangeRequired,

    #[error("Value error: {0}.")]
    ValueError(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let default_message = self.to_string();

        let (status, message, errors) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, default_message, None),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden.".to_string(), None),
            AppError::NotFound => (StatusCode::NOT_FOUND, default_message, None),
            AppError::StateConflict(msg) => (StatusCode::CONFLICT, msg, None),
            AppError::PasswordChangeRequired => (StatusCode::FORBIDDEN, default_message, None),
            AppError::ValueError(_msg) => (StatusCode::BAD_REQUEST, default_message, None),
            AppError::ValidationError(errs) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                default_message,
                Some(errs),
            ),
            AppError::Database(err) => {
                tracing::error!("Database query execution error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server error.".to_string(),
                    None,
                )
            }
            AppError::Internal(err) => {
                tracing::error!("Unhandled internal error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server error.".to_string(),
                    None,
                )
            }
        };

        ApiResponse::error(status, message, errors)
    }
}
