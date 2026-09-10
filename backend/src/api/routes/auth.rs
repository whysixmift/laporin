use crate::auth::{
    generate_otp, generate_session_token, hash_otp, hash_password, hash_token, verify_password,
};
use crate::db::otp_repo::OtpRepo;
use crate::db::session_repo::SessionRepo;
use crate::db::user_repo::UserRepo;
use crate::domain::auth::{
    GoogleCallback, LoginRequest, LoginResponse, RegisterRequest, RegistrationResponse,
    VerifyOtpRequest,
};
use crate::errors::AppError;
use crate::middleware::AuthenticatedUser;
use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::{Duration, Utc};
use serde_json::json;

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegistrationResponse>), AppError> {
    // 1. Basic validation
    if payload.email.trim().is_empty() || !payload.email.contains('@') {
        return Err(AppError::ValidationError("Valid email is required".into()));
    }
    if payload.password.len() < 8 {
        return Err(AppError::ValidationError(
            "Password must be at least 8 characters long".into(),
        ));
    }

    // 2. Verify CAPTCHA
    if !state.captcha.verify(&payload.captcha_token).await {
        return Err(AppError::InvalidCaptcha);
    }

    // 3. Check if user already exists
    if let Some(_) = UserRepo::find_by_email(&state.db, &payload.email).await? {
        return Err(AppError::Conflict(
            "User with this email already exists".into(),
        ));
    }

    // 4. Hash password with Argon2id
    let password_hash =
        hash_password(&payload.password).map_err(|e| AppError::Argon2(e.to_string()))?;

    // 5. Create user
    let user = UserRepo::create_user(&state.db, &payload.email, &password_hash).await?;

    // 6. Generate and store OTP
    let otp = generate_otp();
    let otp_hash = hash_otp(&otp);
    OtpRepo::create_otp(&state.db, user.id, &otp_hash).await?;

    tracing::info!(user_id = %user.id, "User registered successfully, OTP generated");

    Ok((
        StatusCode::CREATED,
        Json(RegistrationResponse {
            user_id: user.id,
            otp_sent: true,
        }),
    ))
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Response, AppError> {
    // 1. Verify CAPTCHA
    if !state.captcha.verify(&payload.captcha_token).await {
        return Err(AppError::InvalidCaptcha);
    }

    // 2. Find user
    let user = UserRepo::find_by_email(&state.db, &payload.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".into()))?;

    // 3. Verify password
    let valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|e| AppError::Argon2(e.to_string()))?;
    if !valid {
        return Err(AppError::Unauthorized("Invalid email or password".into()));
    }

    // 4. Create session
    let session_token = generate_session_token();
    let token_hash = hash_token(&session_token);
    let expires_at = Utc::now() + Duration::hours(state.config.session_expiry_hours);

    SessionRepo::create_session(&state.db, user.id, &token_hash, expires_at).await?;

    // 5. Build secure session cookie
    let is_prod = state.config.environment == "production";
    let cookie = Cookie::build((
        state.config.session_cookie_name.clone(),
        session_token.clone(),
    ))
    .path("/")
    .http_only(true)
    .same_site(SameSite::Strict)
    .secure(is_prod)
    .max_age(time::Duration::hours(state.config.session_expiry_hours))
    .build();

    let mut response = (
        StatusCode::OK,
        Json(LoginResponse {
            user_id: user.id,
            expires_at,
        }),
    )
        .into_response();

    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn google_oauth_url_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    let url = state.oauth.get_authorization_url(None);
    Json(json!({ "url": url }))
}

pub async fn google_callback_handler(
    State(state): State<AppState>,
    Json(payload): Json<GoogleCallback>,
) -> Result<Response, AppError> {
    // 1. Exchange code with Google
    let user_info = state
        .oauth
        .exchange_code(&payload.code)
        .await
        .map_err(|e| AppError::InvalidRequest(format!("Google OAuth error: {}", e)))?;

    // 2. Find or create linked user
    let user =
        UserRepo::find_or_create_oauth_user(&state.db, "google", &user_info.sub, &user_info.email)
            .await?;

    // 3. Create session
    let session_token = generate_session_token();
    let token_hash = hash_token(&session_token);
    let expires_at = Utc::now() + Duration::hours(state.config.session_expiry_hours);

    SessionRepo::create_session(&state.db, user.id, &token_hash, expires_at).await?;

    // 4. Set session cookie
    let is_prod = state.config.environment == "production";
    let cookie = Cookie::build((state.config.session_cookie_name.clone(), session_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(is_prod)
        .max_age(time::Duration::hours(state.config.session_expiry_hours))
        .build();

    let mut response = (StatusCode::OK, Json(json!({ "message": "OK" }))).into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn logout_handler(
    State(state): State<AppState>,
    auth_user: AuthenticatedUser,
) -> Result<Response, AppError> {
    let token_hash = hash_token(&auth_user.token);
    SessionRepo::revoke_session(&state.db, &token_hash).await?;

    // Clear cookie
    let cookie = Cookie::build((state.config.session_cookie_name.clone(), ""))
        .path("/")
        .http_only(true)
        .max_age(time::Duration::seconds(0))
        .build();

    let mut response = StatusCode::NO_CONTENT.into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn verify_otp_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyOtpRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = UserRepo::find_by_email(&state.db, &payload.email)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    let latest_otp = OtpRepo::find_latest_otp(&state.db, user.id)
        .await?
        .ok_or_else(|| AppError::InvalidOtp("No active OTP found".into()))?;

    if latest_otp.expires_at < Utc::now() {
        return Err(AppError::InvalidOtp("OTP has expired".into()));
    }

    if latest_otp.attempts >= 3 {
        return Err(AppError::InvalidOtp(
            "Max OTP verification attempts exceeded".into(),
        ));
    }

    let input_hash = hash_otp(&payload.otp);
    if input_hash != latest_otp.otp_hash {
        OtpRepo::increment_attempts(&state.db, latest_otp.id).await?;
        return Err(AppError::InvalidOtp("Invalid OTP".into()));
    }

    // OTP verified successfully, clean it up
    OtpRepo::delete_otp(&state.db, latest_otp.id).await?;

    Ok(Json(json!({ "message": "OTP verified successfully" })))
}
