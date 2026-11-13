use std::collections::HashMap;

use axum::{
    http::{HeaderValue, StatusCode, header},
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

    #[error("Request doesn't valid: {0}")]
    BadRequest(String),

    #[error("Too many requests.")]
    TooManyRequests(u64),

    #[error("Value error: {0}.")]
    ValueError(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    pub fn state_conflict(msg: impl Into<String>) -> Self {
        Self::StateConflict(msg.into())
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn value_error(msg: impl Into<String>) -> Self {
        Self::ValueError(msg.into())
    }

    /// Helper untuk membuat validation_error
    pub fn validation_error(field: &str, message: &str) -> Self {
        let mut errors = HashMap::new();
        errors.insert(field.to_string(), vec![message.to_string()]);
        AppError::ValidationError(errors)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let default_message = self.to_string();

        let retry_after = match &self {
            AppError::TooManyRequests(secs) => Some(*secs),
            _ => None,
        };

        let (status, message, errors) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, default_message, None),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden.".to_string(), None),
            AppError::NotFound => (StatusCode::NOT_FOUND, default_message, None),
            AppError::StateConflict(msg) => (StatusCode::CONFLICT, msg, None),
            AppError::PasswordChangeRequired => (StatusCode::FORBIDDEN, default_message, None),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.to_string(), None),
            AppError::ValueError(msg) => (StatusCode::BAD_REQUEST, msg, None),
            AppError::ValidationError(errs) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                default_message,
                Some(errs),
            ),
            AppError::TooManyRequests(secs) => (
                StatusCode::TOO_MANY_REQUESTS,
                format!("Too many requests. Please retry in {secs} seconds"),
                None,
            ),
            AppError::Database(err) => match err {
                sqlx::Error::RowNotFound => (
                    StatusCode::NOT_FOUND,
                    "Resource not found.".to_string(),
                    None,
                ),
                _ => {
                    tracing::error!("Database query execution error: {:?}", err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Server error.".to_string(),
                        None,
                    )
                }
            },
            AppError::Internal(err) => {
                tracing::error!("Unhandled internal error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server error.".to_string(),
                    None,
                )
            }
        };

        let mut response = ApiResponse::error(status, message, errors);

        if let Some(secs) = retry_after {
            response
                .headers_mut()
                .insert(header::RETRY_AFTER, HeaderValue::from(secs));
        }

        response
    }
}
