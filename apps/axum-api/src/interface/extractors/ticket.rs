use std::collections::HashMap;

use axum::{
    RequestPartsExt,
    extract::{FromRequestParts, Path},
};
use uuid::Uuid;

use crate::{common::AppError, domain::models::Ticket, interface::AppState};

impl FromRequestParts<AppState> for Ticket {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let params = parts
            .extract::<Path<HashMap<String, String>>>()
            .await
            .map_err(|_| AppError::bad_request("Parameter ID rute tidak ditemukan."))?;

        let raw_id = params
            .get("id")
            .or_else(|| params.get("ticket_id"))
            .or_else(|| params.get("ticket"))
            .ok_or_else(|| AppError::bad_request("Parameter ID tiket tidak valid."))?;

        let ticket_id = Uuid::parse_str(raw_id)
            .map_err(|_| AppError::bad_request("Format ID tiket tidak valid (harud UUID)."))?;

        state.ticket_service.get_by_id(ticket_id).await
    }
}
