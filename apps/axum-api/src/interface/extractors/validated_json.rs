use std::collections::HashMap;

use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::common::AppError;

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + 'static,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state).await.map_err(|_| {
            let mut errs = HashMap::new();
            errs.insert(
                "body".to_string(),
                vec!["Payload JSON tidak valid".to_string()],
            );
            AppError::ValidationError(errs)
        })?;

        data.validate().map_err(|validation_errors| {
            let mut field_errors: HashMap<String, Vec<String>> = HashMap::new();
            for (field, err_vec) in validation_errors.field_errors() {
                let messages: Vec<String> = err_vec
                    .iter()
                    .map(|e| {
                        e.message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| {
                                format!("Field {} tidak memenuhi kriteria validasi.", field)
                            })
                    })
                    .collect();
                field_errors.insert(field.to_string(), messages);
            }
            AppError::ValidationError(field_errors)
        })?;

        Ok(ValidatedJson(data))
    }
}
