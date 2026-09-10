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

    let path = request.uri().path().to_string();
    let method = request.method().to_string();
    let start = std::time::Instant::now();

    let mut response = next.run(request).await;

    let duration_ms = start.elapsed().as_millis();
    let status = response.status().as_u16();

    if let Ok(header_val) = request_id.parse() {
        response.headers_mut().insert(REQUEST_ID_HEADER, header_val);
    }

    tracing::info!(
        request_id = %request_id,
        method = %method,
        path = %path,
        status = %status,
        duration_ms = %duration_ms,
        "HTTP request completed"
    );

    response
}
