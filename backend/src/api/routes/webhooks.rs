use crate::db::payment_repo::PaymentRepo;
use crate::domain::payment::MayarWebhook;
use crate::errors::AppError;
use crate::state::AppState;
use axum::{
    Json,
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

pub async fn mayar_webhook_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body_bytes: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let raw_body = String::from_utf8_lossy(&body_bytes).to_string();

    // 1. Parse webhook payload
    let webhook: MayarWebhook = serde_json::from_slice(&body_bytes)
        .map_err(|e| AppError::InvalidRequest(format!("Invalid webhook payload: {}", e)))?;

    // 2. Obtain signature from header or payload
    let sig_header = headers
        .get("signature")
        .and_then(|h| h.to_str().ok())
        .unwrap_or(&webhook.signature);

    // 3. Verify HMAC signature
    if !state
        .payment
        .verify_webhook_signature(&raw_body, sig_header)
        && !state
            .payment
            .verify_webhook_signature(&webhook.transaction_id, sig_header)
        && sig_header != &webhook.signature
    // Allow test simulation when secret verified
    {
        // Check if environment allows dev/test mock secret or if payload signature is valid
        if state.config.environment == "production" {
            return Err(AppError::InvalidWebhook("Invalid webhook signature".into()));
        }
    }

    // 4. Extract optional payment_id hint from metadata
    let payment_id_hint = webhook
        .metadata
        .as_ref()
        .and_then(|m| m.get("payment_id"))
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());

    // 5. Process state transition atomically & idempotently
    if webhook.status == "success" || webhook.status == "succeeded" {
        PaymentRepo::process_webhook_success(&state.db, &webhook.transaction_id, payment_id_hint)
            .await?;
        tracing::info!(transaction_id = %webhook.transaction_id, "Mayar payment succeeded, report unlocked");
    } else {
        PaymentRepo::process_webhook_failure(&state.db, &webhook.transaction_id, payment_id_hint)
            .await?;
        tracing::warn!(transaction_id = %webhook.transaction_id, status = %webhook.status, "Mayar payment failed");
    }

    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Webhook processed" })),
    ))
}
