mod common;

use axum_test::TestServer;
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
async fn test_secure_download_entitlement() {
    let (router, state) = common::setup_test_app().await;
    let server = TestServer::new(router).unwrap();

    let user_cookie = create_authenticated_user(&server, "buyer").await;
    let other_cookie = create_authenticated_user(&server, "intruder").await;

    // Create Report
    let create_payload = json!({
        "title": "Laporan Magang Cyber Security",
        "student": {
            "full_name": "Raden Saleh",
            "student_id": "55443322",
            "school": "Universitas Gadjah Mada"
        },
        "internship": {
            "company_name": "PT Cyber Defense Indonesia"
        }
    });

    let resp = server
        .post("/reports")
        .add_header("Cookie", &user_cookie)
        .json(&create_payload)
        .await;
    let report: serde_json::Value = resp.json();
    let report_id = report["id"].as_str().unwrap();
    let report_uuid = Uuid::parse_str(report_id).unwrap();

    // 1. Download attempt on draft/unpaid report is rejected (403 Forbidden)
    let resp = server
        .get(&format!("/reports/{}/download", report_id))
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status_forbidden();

    // 2. Setup mock generated DOCX file in storage and DB
    let file_uuid = Uuid::new_v4();
    let docx_path = state.storage.report_docx_path(report_uuid, file_uuid);
    let sample_docx_bytes = b"PK\x03\x04mock_docx_binary_content_stream";
    state
        .storage
        .write_file(&docx_path, sample_docx_bytes)
        .unwrap();
    let docx_str = docx_path.to_str().unwrap().to_string();

    laporin_backend::db::report_repo::ReportRepo::save_file_entry(
        &state.db,
        report_uuid,
        "docx",
        &docx_str,
        sample_docx_bytes.len() as i64,
    )
    .await
    .unwrap();

    // 3. Set report status to unlocked
    laporin_backend::db::report_repo::ReportRepo::update_status(&state.db, report_uuid, "unlocked")
        .await
        .unwrap();

    // 4. Intruder download attempt is rejected (403 Forbidden)
    let resp = server
        .get(&format!("/reports/{}/download", report_id))
        .add_header("Cookie", &other_cookie)
        .await;
    resp.assert_status_forbidden();

    // 5. Authorized owner download succeeds
    let resp = server
        .get(&format!("/reports/{}/download", report_id))
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status_ok();
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    );
    assert_eq!(resp.as_bytes(), &sample_docx_bytes[..]);
}
