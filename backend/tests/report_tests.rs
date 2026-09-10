mod common;

use axum::http::StatusCode;
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
async fn test_report_crud_and_ownership() {
    let (router, _) = common::setup_test_app().await;
    let server = TestServer::new(router).unwrap();

    let user_cookie = create_authenticated_user(&server, "alice").await;
    let other_cookie = create_authenticated_user(&server, "bob").await;

    // 1. Create Report
    let create_payload = json!({
        "title": "Laporan PKL Software Engineering",
        "student": {
            "full_name": "Budi Santoso",
            "student_id": "12345678",
            "school": "SMK Negeri 1 Jakarta",
            "major": "Rekayasa Perangkat Lunak",
            "semester": "5"
        },
        "internship": {
            "company_name": "PT Teknologi Maju",
            "company_address": "Jl. Sudirman No. 10, Jakarta",
            "department": "Engineering",
            "role": "Frontend Intern",
            "start_date": "2026-01-10",
            "end_date": "2026-04-10",
            "company_supervisor": "Pak Hendra",
            "school_supervisor": "Bu Siti",
            "description": "Mengembangkan antarmuka aplikasi web"
        }
    });

    let resp = server
        .post("/reports")
        .add_header("Cookie", &user_cookie)
        .json(&create_payload)
        .await;
    resp.assert_status(StatusCode::CREATED);
    let created_report: serde_json::Value = resp.json();
    let report_id = created_report["id"].as_str().unwrap();
    assert_eq!(created_report["status"], "draft");
    assert_eq!(created_report["title"], "Laporan PKL Software Engineering");

    // 2. Get Report
    let resp = server
        .get(&format!("/reports/{}", report_id))
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status_ok();
    let fetched: serde_json::Value = resp.json();
    assert_eq!(fetched["id"], report_id);
    assert_eq!(fetched["student"]["full_name"], "Budi Santoso");

    // 3. Other User Cannot Access Alice's Report
    let resp = server
        .get(&format!("/reports/{}", report_id))
        .add_header("Cookie", &other_cookie)
        .await;
    resp.assert_status_forbidden();

    // 4. Update Report
    let update_payload = json!({
        "student": {
            "full_name": "Budi Santoso Updated",
            "student_id": "12345678",
            "school": "SMK Negeri 1 Jakarta",
            "major": "RPL",
            "semester": "6"
        }
    });

    let resp = server
        .patch(&format!("/reports/{}", report_id))
        .add_header("Cookie", &user_cookie)
        .json(&update_payload)
        .await;
    resp.assert_status_ok();

    // Verify update persisted
    let resp = server
        .get(&format!("/reports/{}", report_id))
        .add_header("Cookie", &user_cookie)
        .await;
    let updated: serde_json::Value = resp.json();
    assert_eq!(updated["student"]["full_name"], "Budi Santoso Updated");

    // 5. List Reports
    let resp = server
        .get("/reports")
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status_ok();
    let list: Vec<serde_json::Value> = resp.json();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0]["id"], report_id);
}
