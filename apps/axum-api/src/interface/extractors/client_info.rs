use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, FromRequestParts},
    http::request::Parts,
};

use crate::common::{AppError, Client};

pub struct ClientInfo {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl<S> FromRequestParts<S> for ClientInfo
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let ip_address = Client::extract_client_api(
            &parts.headers,
            parts.extensions.get::<ConnectInfo<SocketAddr>>(),
        );

        let user_agent = Client::extract_user_agent(&parts.headers);

        Ok(Self {
            ip_address,
            user_agent,
        })
    }
}
