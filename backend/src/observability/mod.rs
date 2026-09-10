use axum::{extract::Request, middleware::Next, response::Response};
use uuid::Uuid;

pub const REQUEST_ID_HEADER: &str = "x-request-id";

pub async fn request_id_middleware(request: Request, next: Next) -> Response {
    let request_id = request
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|h| h.to_str().ok())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(REQUEST_ID_HEADER, request_id.parse().unwrap());
    response
}

pub fn log_request(request_id: &str, method: &str, path: &str, status: u16, duration_ms: u128) {
    tracing::info!(
        request_id = %request_id,
        method = %method,
        path = %path,
        status = %status,
        duration_ms = %duration_ms,
        "request completed"
    );
}
