use std::{
    net::{IpAddr, SocketAddr},
    num::NonZeroU32,
    sync::Arc,
    time::Duration,
};

use axum::{
    extract::{ConnectInfo, Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use governor::{
    Quota, RateLimiter,
    clock::{Clock, DefaultClock},
    state::keyed::DefaultKeyedStateStore,
};

use crate::{
    common::AppError,
    domain::enums::RateLimitTier,
    interface::{AppState, extractors::auth_user::AuthUser},
};

pub type KeyedLimiter = Arc<RateLimiter<String, DefaultKeyedStateStore<String>, DefaultClock>>;

fn quota_for(tier: RateLimitTier) -> Quota {
    let n = |v: u32| NonZeroU32::new(v).expect("quota non-zero.");
    match tier {
        RateLimitTier::Login => Quota::per_minute(n(5)),
        RateLimitTier::Upload => Quota::per_minute(n(20)),
        RateLimitTier::Search => Quota::per_minute(n(60)),
        RateLimitTier::Export => Quota::per_minute(n(10)),
        RateLimitTier::Api => Quota::per_minute(n(120)),
    }
}

#[derive(Clone)]
pub struct RateLimiterRegistry {
    pub login: KeyedLimiter,
    pub upload: KeyedLimiter,
    pub search: KeyedLimiter,
    pub export: KeyedLimiter,
    pub api: KeyedLimiter,
    pub app_env: String,
}

impl RateLimiterRegistry {
    pub fn new(app_env: String) -> Self {
        let registry = Self {
            login: Arc::new(RateLimiter::keyed(quota_for(RateLimitTier::Login))),
            upload: Arc::new(RateLimiter::keyed(quota_for(RateLimitTier::Upload))),
            search: Arc::new(RateLimiter::keyed(quota_for(RateLimitTier::Search))),
            export: Arc::new(RateLimiter::keyed(quota_for(RateLimitTier::Export))),
            api: Arc::new(RateLimiter::keyed(quota_for(RateLimitTier::Api))),
            app_env,
        };

        let cleaner = registry.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(900));
            loop {
                interval.tick().await;
                cleaner.login.retain_recent();
                cleaner.upload.retain_recent();
                cleaner.search.retain_recent();
                cleaner.export.retain_recent();
                cleaner.api.retain_recent();
                tracing::debug!("Rate limiter memory pruned.");
            }
        });

        registry
    }

    pub fn limiter(&self, tier: RateLimitTier) -> &KeyedLimiter {
        match tier {
            RateLimitTier::Login => &self.login,
            RateLimitTier::Upload => &self.upload,
            RateLimitTier::Search => &self.search,
            RateLimitTier::Export => &self.export,
            RateLimitTier::Api => &self.api,
        }
    }

    pub fn is_testing_or_local(&self) -> bool {
        matches!(self.app_env.as_str(), "local" | "test" | "testing")
    }
}

// -- Helper resolusi key
fn extract_client_api(headers: &HeaderMap, info: Option<&ConnectInfo<SocketAddr>>) -> String {
    if let Some(forwarded) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(ip) = forwarded.split(',').next().map(str::trim) {
            if ip.parse::<IpAddr>().is_ok() {
                return ip.to_string();
            }
        }
    }

    if let Some(ip) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        if ip.parse::<IpAddr>().is_ok() {
            return ip.to_string();
        }
    }

    info.map(|ConnectInfo(addr)| addr.ip().to_string())
        .unwrap_or_else(|| "127.0.0.1".to_string())
}

fn resolve_key(req: &Request, tier: RateLimitTier) -> String {
    if tier == RateLimitTier::Login {
        let ip = extract_client_api(
            req.headers(),
            req.extensions().get::<ConnectInfo<SocketAddr>>(),
        );
        return format!("ip:{}", ip);
    }
    if let Some(auth) = req.extensions().get::<AuthUser>() {
        return format!("usr: {}", auth.0.id);
    }

    let ip = extract_client_api(
        req.headers(),
        req.extensions().get::<ConnectInfo<SocketAddr>>(),
    );
    format!("ip: {}", ip)
}

// Inti middleware
async fn enforce(
    tier: RateLimitTier,
    state: &AppState,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    if tier == RateLimitTier::Login && state.rate_limit.is_testing_or_local() {
        return Ok(next.run(req).await);
    }

    let limiter = state.rate_limit.limiter(tier);
    let key = resolve_key(&req, tier);

    match limiter.check_key(&key) {
        Ok(_) => Ok(next.run(req).await),
        Err(not_until) => {
            let wait = not_until.wait_time_from(limiter.clock().now());
            let wait_secs = wait.as_secs() + u64::from(wait.subsec_nanos() > 0);
            Err(AppError::TooManyRequests(wait_secs))
        }
    }
}

// Handler
pub async fn rate_limit_login(
    State(s): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    enforce(RateLimitTier::Login, &s, req, next).await
}
pub async fn rate_limit_upload(
    State(s): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    enforce(RateLimitTier::Upload, &s, req, next).await
}
pub async fn rate_limit_search(
    State(s): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    enforce(RateLimitTier::Search, &s, req, next).await
}
pub async fn rate_limit_export(
    State(s): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    enforce(RateLimitTier::Export, &s, req, next).await
}
pub async fn rate_limit_api(
    State(s): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    enforce(RateLimitTier::Api, &s, req, next).await
}
