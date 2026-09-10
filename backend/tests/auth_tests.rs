mod common;

use axum::http::StatusCode;
use axum_test::TestServer;
use serde_json::json;
use uuid::Uuid;

#[tokio::test]
async fn test_registration_and_login_flow() {
    let (router, _) = common::setup_test_app().await;
    let server = TestServer::new(router).unwrap();

    let email = format!("user_{}@example.com", Uuid::new_v4());
    let password = "SuperSecretPassword123!";

    // 1. Invalid CAPTCHA token rejected
    let resp = server
        .post("/auth/register")
        .json(&json!({
            "email": email,
            "password": password,
            "captcha_token": "invalid_token"
        }))
        .await;
    resp.assert_status_bad_request();

    // 2. Successful Registration
    let resp = server
        .post("/auth/register")
        .json(&json!({
            "email": email,
            "password": password,
            "captcha_token": "valid_token"
        }))
        .await;
    resp.assert_status(StatusCode::CREATED);
    let body: serde_json::Value = resp.json();
    assert!(body["user_id"].is_string());
    assert_eq!(body["otp_sent"], true);

    // 3. Duplicate Registration Rejected
    let resp = server
        .post("/auth/register")
        .json(&json!({
            "email": email,
            "password": password,
            "captcha_token": "valid_token"
        }))
        .await;
    resp.assert_status_conflict();

    // 4. Login with Wrong Password
    let resp = server
        .post("/auth/login")
        .json(&json!({
            "email": email,
            "password": "WrongPassword123!",
            "captcha_token": "valid_token"
        }))
        .await;
    resp.assert_status_unauthorized();

    // 5. Successful Login
    let resp = server
        .post("/auth/login")
        .json(&json!({
            "email": email,
            "password": password,
            "captcha_token": "valid_token"
        }))
        .await;
    resp.assert_status_ok();
    assert!(resp.headers().contains_key("set-cookie"));
    let cookie_val = resp.headers().get("set-cookie").unwrap().to_str().unwrap();
    assert!(cookie_val.contains("session_id="));

    // Extract session cookie
    let cookie_part = cookie_val.split(';').next().unwrap();

    // 6. Logout
    let resp = server
        .post("/auth/logout")
        .add_header("Cookie", cookie_part)
        .await;
    resp.assert_status(StatusCode::NO_CONTENT);

    // 7. Request with revoked session is unauthorized
    let resp = server
        .get("/reports")
        .add_header("Cookie", cookie_part)
        .await;
    resp.assert_status_unauthorized();
}

#[tokio::test]
async fn test_otp_verification() {
    let (router, state) = common::setup_test_app().await;
    let server = TestServer::new(router).unwrap();

    let email = format!("user_{}@example.com", Uuid::new_v4());
    let password = "SuperSecretPassword123!";

    // Register
    let reg_resp = server
        .post("/auth/register")
        .json(&json!({
            "email": email,
            "password": password,
            "captcha_token": "valid_token"
        }))
        .await;
    reg_resp.assert_status(StatusCode::CREATED);
    let reg_body: serde_json::Value = reg_resp.json();
    let user_id = Uuid::parse_str(reg_body["user_id"].as_str().unwrap()).unwrap();

    // Look up OTP
    let _latest_otp = laporin_backend::db::otp_repo::OtpRepo::find_latest_otp(&state.db, user_id)
        .await
        .unwrap()
        .unwrap();

    // Wrong OTP
    let resp = server
        .post("/auth/verify-otp")
        .json(&json!({
            "email": email,
            "otp": "000000"
        }))
        .await;
    resp.assert_status_bad_request();

    // Check attempts incremented
    let updated_otp = laporin_backend::db::otp_repo::OtpRepo::find_latest_otp(&state.db, user_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated_otp.attempts, 1);
}

#[tokio::test]
async fn test_google_oauth_endpoints() {
    let (router, _) = common::setup_test_app().await;
    let server = TestServer::new(router).unwrap();

    // 1. Get OAuth URL
    let resp = server.get("/auth/google/oauth_url").await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(
        body["url"]
            .as_str()
            .unwrap()
            .contains("accounts.google.com")
    );

    // 2. Callback
    let resp = server
        .post("/auth/google/callback")
        .json(&json!({
            "code": "test_code_oauth_user_1",
            "state": "test_state"
        }))
        .await;
    resp.assert_status_ok();
    assert!(resp.headers().contains_key("set-cookie"));
}
