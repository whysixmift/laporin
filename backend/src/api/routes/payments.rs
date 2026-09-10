use crate::db::payment_repo::PaymentRepo;
use crate::db::report_repo::ReportRepo;
use crate::domain::payment::{Payment, PaymentCreate, PaymentResponse};
use crate::errors::AppError;
use crate::middleware::AuthenticatedUser;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

pub async fn create_payment_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(payload): Json<PaymentCreate>,
) -> Result<(StatusCode, Json<PaymentResponse>), AppError> {
    // 1. Verify report ownership
    let (_report, owner_id) = ReportRepo::get_report_by_id(&state.db, payload.report_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

    if owner_id != auth.user_id {
        return Err(AppError::Forbidden(
            "You do not have access to this report".into(),
        ));
    }

    // 2. Fixed price from backend (Rp15.000)
    let price_cents = state.payment.get_price_cents();

    // 3. Create payment record
    let payment = PaymentRepo::create_payment(&state.db, payload.report_id, price_cents).await?;

    // 4. Create gateway checkout link
    let payment_url = state
        .payment
        .create_gateway_payment(payment.payment_id, payload.report_id, &payload.return_url)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to initialize payment gateway: {}", e)))?;

    Ok((
        StatusCode::CREATED,
        Json(PaymentResponse {
            payment_id: payment.payment_id,
            payment_url,
        }),
    ))
}

pub async fn get_payment_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Payment>, AppError> {
    let (payment, owner_id) = PaymentRepo::get_payment_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Payment not found".into()))?;

    if owner_id != auth.user_id {
        return Err(AppError::Forbidden(
            "You do not have access to this payment".into(),
        ));
    }

    Ok(Json(payment))
}
