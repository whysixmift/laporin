mod common;

use axum::http::StatusCode;
use axum_test::TestServer;
use laporin_backend::auth::create_hmac_signature;
use serde_json::json;
use uuid::Uuid;

async fn create_authenticated_user(server: &TestServer, email_prefix: &str) -> String {
    let email = format!("{}_{}@example.com", email_prefix, Uuid::new_v4());
    let password = "Password123456!";

    server
        .post("/auth/register")
        .json(&json!({
            "email": email,
            "password": password,
            "captcha_token": "valid_token"
        }))
        .await;

    let login_resp = server
        .post("/auth/login")
        .json(&json!({
            "email": email,
            "password": password,
            "captcha_token": "valid_token"
        }))
        .await;

    let cookie = login_resp
        .headers()
        .get("set-cookie")
        .unwrap()
        .to_str()
        .unwrap();
    cookie.split(';').next().unwrap().to_string()
}

#[tokio::test]
async fn test_payment_creation_and_webhook_idempotency() {
    let (router, state) = common::setup_test_app().await;
    let server = TestServer::new(router).unwrap();

    let user_cookie = create_authenticated_user(&server, "payer").await;

    // Create Report
    let create_payload = json!({
        "title": "Laporan Magang Data Science",
        "student": {
            "full_name": "Dewi Sartika",
            "student_id": "77665544",
            "school": "Institut Teknologi Bandung"
        },
        "internship": {
            "company_name": "PT Astra Digital"
        }
    });

    let resp = server
        .post("/reports")
        .add_header("Cookie", &user_cookie)
        .json(&create_payload)
        .await;
    let report: serde_json::Value = resp.json();
    let report_id = report["id"].as_str().unwrap();

    // 1. Create Payment
    let pay_req = json!({
        "report_id": report_id,
        "return_url": "https://app.laporin.example.com/reports/checkout-return"
    });

    let resp = server
        .post("/payments")
        .add_header("Cookie", &user_cookie)
        .json(&pay_req)
        .await;
    resp.assert_status(StatusCode::CREATED);
    let pay_resp: serde_json::Value = resp.json();
    let payment_id = pay_resp["payment_id"].as_str().unwrap();
    let payment_url = pay_resp["payment_url"].as_str().unwrap();
    assert!(payment_url.contains(payment_id));

    // 2. Webhook payload
    let tx_id = format!("mayar_tx_{}", Uuid::new_v4());
    let webhook_body = json!({
        "transaction_id": tx_id,
        "status": "success",
        "amount": 15000,
        "signature": "mock_sig",
        "metadata": {
            "payment_id": payment_id,
            "report_id": report_id
        }
    });

    let raw_payload = webhook_body.to_string();
    let valid_signature =
        create_hmac_signature(&raw_payload, &state.config.mayar_webhook_secret).unwrap();

    // 3. Process Webhook
    let resp = server
        .post("/webhook/mayar")
        .add_header("signature", &valid_signature)
        .content_type("application/json")
        .bytes(raw_payload.as_bytes().to_vec().into())
        .await;
    resp.assert_status_ok();

    // 4. Verify report is now unlocked
    let resp = server
        .get(&format!("/reports/{}", report_id))
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status_ok();
    let updated_report: serde_json::Value = resp.json();
    assert_eq!(updated_report["status"], "unlocked");

    // 5. Send Duplicate Webhook (Idempotency Test)
    let resp = server
        .post("/webhook/mayar")
        .add_header("signature", &valid_signature)
        .content_type("application/json")
        .bytes(raw_payload.as_bytes().to_vec().into())
        .await;
    resp.assert_status_ok();
}
