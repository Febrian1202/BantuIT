use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::ConnectInfo,
    http::{HeaderMap, header::USER_AGENT},
};

pub struct Client;

impl Client {
    pub fn extract_client_api(
        headers: &HeaderMap,
        info: Option<&ConnectInfo<SocketAddr>>,
    ) -> Option<String> {
        if let Some(forwarded) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok())
            && let Some(ip) = forwarded.split(',').next_back().map(str::trim)
            && ip.parse::<IpAddr>().is_ok()
        {
            return Some(ip.to_string());
        }

        if let Some(ip) = headers.get("x-real-ip").and_then(|v| v.to_str().ok())
            && ip.parse::<IpAddr>().is_ok()
        {
            return Some(ip.to_string());
        }

        info.map(|ConnectInfo(addr)| addr.ip().to_string())
    }

    pub fn extract_client_api_or_default(
        headers: &HeaderMap,
        info: Option<&ConnectInfo<SocketAddr>>,
    ) -> String {
        Self::extract_client_api(headers, info).unwrap_or_else(|| "127.0.0.1".to_string())
    }

    pub fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
        headers
            .get(USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
    }
}
