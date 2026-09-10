use crate::errors::{AppError, ErrorResponse};
use crate::state::AppState;
use axum::{
    extract::{Request, State},
    http::HeaderValue,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::time::Duration;

pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let path = request.uri().path().to_string();
    let client_ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let (limit, window_dur) = if path.starts_with("/api/v1/auth") || path.starts_with("/auth") {
        (
            state.config.auth_rate_limit_per_min,
            Duration::from_secs(60),
        )
    } else if path.contains("/research/start") {
        (
            state.config.research_start_rate_limit_per_hour,
            Duration::from_secs(3600),
        )
    } else if path.contains("/generation/start") {
        (
            state.config.gen_start_rate_limit_per_hour,
            Duration::from_secs(3600),
        )
    } else {
        (state.config.api_rate_limit_per_min, Duration::from_secs(60))
    };

    let key = format!("{}:{}", client_ip, path);
    let (allowed, remaining, reset_secs) = state
        .rate_limiter
        .check_and_increment(&key, limit, window_dur);

    if !allowed {
        let mut resp = AppError::RateLimitExceeded.into_response();
        let headers = resp.headers_mut();
        if let Ok(v) = HeaderValue::from_str(&limit.to_string()) {
            headers.insert("X-RateLimit-Limit", v);
        }
        if let Ok(v) = HeaderValue::from_str(&remaining.to_string()) {
            headers.insert("X-RateLimit-Remaining", v);
        }
        if let Ok(v) = HeaderValue::from_str(&reset_secs.to_string()) {
            headers.insert("X-RateLimit-Reset", v);
        }
        return resp;
    }

    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    if let Ok(v) = HeaderValue::from_str(&limit.to_string()) {
        headers.insert("X-RateLimit-Limit", v);
    }
    if let Ok(v) = HeaderValue::from_str(&remaining.to_string()) {
        headers.insert("X-RateLimit-Remaining", v);
    }
    if let Ok(v) = HeaderValue::from_str(&reset_secs.to_string()) {
        headers.insert("X-RateLimit-Reset", v);
    }

    response
}
