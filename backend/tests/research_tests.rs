mod common;

use axum::http::StatusCode;
use axum_test::TestServer;
use laporin_backend::services::CrawlerService;
use serde_json::json;
use std::net::IpAddr;
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
async fn test_research_job_lifecycle_and_provenance() {
    let (router, state) = common::setup_test_app().await;
    let server = TestServer::new(router).unwrap();

    let user_cookie = create_authenticated_user(&server, "researcher").await;

    // Create report
    let create_payload = json!({
        "title": "Laporan Magang PT GoTo",
        "student": {
            "full_name": "Rian Pratama",
            "student_id": "87654321",
            "school": "Universitas Indonesia",
            "major": "Ilmu Komputer",
            "semester": "7"
        },
        "internship": {
            "company_name": "PT GoTo Gojek Tokopedia",
            "company_address": "Gedung Pasaraya Blok M, Jakarta",
            "department": "Core Infrastructure",
            "role": "Software Engineer Intern",
            "start_date": "2026-02-01",
            "end_date": "2026-05-01",
            "company_supervisor": "Mas Andi",
            "school_supervisor": "Prof. Budi",
            "description": "Optimasi service backend"
        }
    });

    let resp = server
        .post("/reports")
        .add_header("Cookie", &user_cookie)
        .json(&create_payload)
        .await;
    resp.assert_status(StatusCode::CREATED);
    let report: serde_json::Value = resp.json();
    let report_id = report["id"].as_str().unwrap();
    let report_uuid = Uuid::parse_str(report_id).unwrap();

    // 1. Start Research Job
    let resp = server
        .post(&format!("/reports/{}/research/start", report_id))
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status(StatusCode::ACCEPTED);
    let job: serde_json::Value = resp.json();
    assert_eq!(job["type"], "research");
    assert_eq!(job["status"], "pending");

    // 2. Starting research again while active returns 409 Conflict
    let resp = server
        .post(&format!("/reports/{}/research/start", report_id))
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status_conflict();

    // 3. Process research job with worker logic
    let (claimed_job_id, _, _) =
        laporin_backend::db::job_repo::JobRepo::claim_research_job(&state.db)
            .await
            .unwrap()
            .expect("Should claim pending research job");

    let facts = state
        .llm
        .extract_research_facts(
            "PT GoTo Gojek Tokopedia",
            Some("Software Engineer Intern"),
            &[(
                "https://example.com/company/goto".into(),
                "GoTo Profile".into(),
                "PT GoTo merupakan ekosistem digital terbesar di Indonesia.".into(),
            )],
        )
        .await
        .unwrap();

    laporin_backend::db::report_repo::ReportRepo::save_research_facts(
        &state.db,
        report_uuid,
        claimed_job_id,
        &facts,
    )
    .await
    .unwrap();
    laporin_backend::db::job_repo::JobRepo::complete_research_job(&state.db, claimed_job_id)
        .await
        .unwrap();
    laporin_backend::db::report_repo::ReportRepo::update_status(
        &state.db,
        report_uuid,
        "research_completed",
    )
    .await
    .unwrap();

    // 4. Verify research facts & provenance in GET /reports/{id}
    let resp = server
        .get(&format!("/reports/{}", report_id))
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status_ok();
    let updated_report: serde_json::Value = resp.json();
    assert_eq!(updated_report["status"], "research_completed");
    let facts_arr = updated_report["research_facts"].as_array().unwrap();
    assert!(!facts_arr.is_empty());
    assert!(!facts_arr[0]["sources"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_ssrf_protection_filters() {
    let crawler = CrawlerService::new();

    // Localhost / Loopback
    let local_v4: IpAddr = "127.0.0.1".parse().unwrap();
    assert!(CrawlerService::is_private_or_restricted(&local_v4));

    // Private Subnets
    let priv_10: IpAddr = "10.1.2.3".parse().unwrap();
    assert!(CrawlerService::is_private_or_restricted(&priv_10));

    let priv_172: IpAddr = "172.16.0.5".parse().unwrap();
    assert!(CrawlerService::is_private_or_restricted(&priv_172));

    let priv_192: IpAddr = "192.168.1.100".parse().unwrap();
    assert!(CrawlerService::is_private_or_restricted(&priv_192));

    // IPv6 Loopback
    let v6_local: IpAddr = "::1".parse().unwrap();
    assert!(CrawlerService::is_private_or_restricted(&v6_local));

    // Public IP
    let public_v4: IpAddr = "93.184.216.34".parse().unwrap(); // example.com
    assert!(!CrawlerService::is_private_or_restricted(&public_v4));

    // HTTP Scheme rejection
    let http_err = crawler.validate_url("http://google.com");
    assert!(http_err.is_err());
}
