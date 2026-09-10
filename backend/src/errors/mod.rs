use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl ErrorResponse {
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
        request_id: Option<String>,
    ) -> Self {
        Self {
            error: ErrorDetail {
                code: code.into(),
                message: message.into(),
                request_id,
            },
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = match self.error.code.as_str() {
            "VALIDATION_ERROR" | "INVALID_REQUEST" | "INVALID_CAPTCHA" | "INVALID_OTP"
            | "INVALID_WEBHOOK" => StatusCode::BAD_REQUEST,
            "UNAUTHORIZED" | "INVALID_CREDENTIALS" | "SESSION_EXPIRED" | "SESSION_REVOKED" => {
                StatusCode::UNAUTHORIZED
            }
            "FORBIDDEN" => StatusCode::FORBIDDEN,
            "NOT_FOUND" => StatusCode::NOT_FOUND,
            "CONFLICT" | "JOB_ALREADY_ACTIVE" => StatusCode::CONFLICT,
            "RATE_LIMIT_EXCEEDED" | "TOO_MANY_REQUESTS" => StatusCode::TOO_MANY_REQUESTS,
            "INSUFFICIENT_STORAGE" => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(self)).into_response()
    }
}

#[derive(Debug)]
pub enum AppError {
    ValidationError(String),
    InvalidRequest(String),
    InvalidCaptcha,
    InvalidOtp(String),
    Unauthorized(String),
    SessionExpired,
    SessionRevoked,
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    JobAlreadyActive,
    RateLimitExceeded,
    InsufficientStorage,
    InvalidWebhook(String),
    Database(sqlx::Error),
    Argon2(String),
    Io(std::io::Error),
    Config(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            AppError::InvalidCaptcha => write!(f, "Invalid CAPTCHA token"),
            AppError::InvalidOtp(msg) => write!(f, "Invalid OTP: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::SessionExpired => write!(f, "Session expired"),
            AppError::SessionRevoked => write!(f, "Session revoked"),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::JobAlreadyActive => write!(f, "Active job already running"),
            AppError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            AppError::InsufficientStorage => write!(f, "Insufficient storage"),
            AppError::InvalidWebhook(msg) => write!(f, "Invalid webhook: {}", msg),
            AppError::Database(err) => write!(f, "Database error: {}", err),
            AppError::Argon2(err) => write!(f, "Argon2 error: {}", err),
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::Config(msg) => write!(f, "Config error: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, message) = match &self {
            AppError::ValidationError(msg) => ("VALIDATION_ERROR", msg.clone()),
            AppError::InvalidRequest(msg) => ("INVALID_REQUEST", msg.clone()),
            AppError::InvalidCaptcha => {
                ("INVALID_CAPTCHA", "CAPTCHA verification failed".to_string())
            }
            AppError::InvalidOtp(msg) => ("INVALID_OTP", msg.clone()),
            AppError::Unauthorized(msg) => ("UNAUTHORIZED", msg.clone()),
            AppError::SessionExpired => ("SESSION_EXPIRED", "Session has expired".to_string()),
            AppError::SessionRevoked => ("SESSION_REVOKED", "Session has been revoked".to_string()),
            AppError::Forbidden(msg) => ("FORBIDDEN", msg.clone()),
            AppError::NotFound(msg) => ("NOT_FOUND", msg.clone()),
            AppError::Conflict(msg) => ("CONFLICT", msg.clone()),
            AppError::JobAlreadyActive => (
                "JOB_ALREADY_ACTIVE",
                "A job is already active for this report".to_string(),
            ),
            AppError::RateLimitExceeded => ("RATE_LIMIT_EXCEEDED", "Too many requests".to_string()),
            AppError::InsufficientStorage => (
                "INSUFFICIENT_STORAGE",
                "Insufficient storage space".to_string(),
            ),
            AppError::InvalidWebhook(msg) => ("INVALID_WEBHOOK", msg.clone()),
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                ("INTERNAL_ERROR", "Database error".to_string())
            }
            AppError::Argon2(err) => {
                tracing::error!("Argon2 error: {:?}", err);
                ("INTERNAL_ERROR", "Password hashing error".to_string())
            }
            AppError::Io(err) => {
                tracing::error!("IO error: {:?}", err);
                ("INTERNAL_ERROR", "IO error".to_string())
            }
            AppError::Config(msg) => {
                tracing::error!("Config error: {}", msg);
                ("INTERNAL_ERROR", "Configuration error".to_string())
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                ("INTERNAL_ERROR", msg.clone())
            }
        };
        ErrorResponse::new(code, message, None).into_response()
    }
}
