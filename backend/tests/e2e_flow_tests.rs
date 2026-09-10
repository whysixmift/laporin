mod common;

use axum::http::StatusCode;
use axum_test::TestServer;
use laporin_backend::auth::create_hmac_signature;
use laporin_backend::domain::report::{InternshipInfo, StudentInfo};
use laporin_backend::services::{DocxService, PreviewService};
use serde_json::json;
use uuid::Uuid;

#[tokio::test]
async fn test_complete_laporin_e2e_flow() {
    let (router, state) = common::setup_test_app().await;
    let server = TestServer::new(router).unwrap();

    let email = format!("e2e_student_{}@example.com", Uuid::new_v4());
    let password = "SecureStudentPass123!";

    // 1. Register
    let reg_resp = server
        .post("/auth/register")
        .json(&json!({
            "email": email,
            "password": password,
            "captcha_token": "valid_token"
        }))
        .await;
    reg_resp.assert_status(StatusCode::CREATED);
    let reg_data: serde_json::Value = reg_resp.json();
    let user_id = Uuid::parse_str(reg_data["user_id"].as_str().unwrap()).unwrap();

    // 2. Verify OTP
    let _otp = laporin_backend::db::otp_repo::OtpRepo::find_latest_otp(&state.db, user_id)
        .await
        .unwrap()
        .expect("OTP should exist");

    // 3. Login
    let login_resp = server
        .post("/auth/login")
        .json(&json!({
            "email": email,
            "password": password,
            "captcha_token": "valid_token"
        }))
        .await;
    login_resp.assert_status_ok();
    let cookie_hdr = login_resp
        .headers()
        .get("set-cookie")
        .unwrap()
        .to_str()
        .unwrap();
    let session_cookie = cookie_hdr.split(';').next().unwrap();

    // 4. Create Report Project (Draft)
    let report_payload = json!({
        "title": "Laporan Praktik Kerja Lapangan di PT Inovasi Maju",
        "student": {
            "full_name": "Maya Anggraini",
            "student_id": "20261001",
            "school": "SMK Negeri 1 Surabaya",
            "major": "Pengembangan Perangkat Lunak dan Gim",
            "semester": "5"
        },
        "internship": {
            "company_name": "PT Inovasi Maju Nusantara",
            "company_address": "Jl. Basuki Rahmat No. 45, Surabaya",
            "department": "Software R&D",
            "role": "Quality Assurance Intern",
            "start_date": "2026-01-15",
            "end_date": "2026-04-15",
            "company_supervisor": "Pak Dimas",
            "school_supervisor": "Bu Ratna",
            "description": "Pengujian fungsional dan otomatisasi sistem informasi"
        }
    });

    let resp = server
        .post("/reports")
        .add_header("Cookie", session_cookie)
        .json(&report_payload)
        .await;
    resp.assert_status(StatusCode::CREATED);
    let report: serde_json::Value = resp.json();
    let report_id = report["id"].as_str().unwrap();
    let report_uuid = Uuid::parse_str(report_id).unwrap();

    // 5. Start Research Job
    let resp = server
        .post(&format!("/reports/{}/research/start", report_id))
        .add_header("Cookie", session_cookie)
        .await;
    resp.assert_status(StatusCode::ACCEPTED);

    // Execute research job
    let (research_job_id, _, _) =
        laporin_backend::db::job_repo::JobRepo::claim_research_job(&state.db)
            .await
            .unwrap()
            .unwrap();

    let facts = state
        .llm
        .extract_research_facts(
            "PT Inovasi Maju Nusantara",
            Some("Quality Assurance Intern"),
            &[(
                "https://example.com/company/inovasi".into(),
                "Profil PT Inovasi Maju Nusantara".into(),
                "PT Inovasi Maju Nusantara adalah penyedia solusi perangkat lunak terintegrasi."
                    .into(),
            )],
        )
        .await
        .unwrap();

    laporin_backend::db::report_repo::ReportRepo::save_research_facts(
        &state.db,
        report_uuid,
        research_job_id,
        &facts,
    )
    .await
    .unwrap();
    laporin_backend::db::job_repo::JobRepo::complete_research_job(&state.db, research_job_id)
        .await
        .unwrap();
    laporin_backend::db::report_repo::ReportRepo::update_status(
        &state.db,
        report_uuid,
        "research_completed",
    )
    .await
    .unwrap();

    // 6. Start Generation Job
    let resp = server
        .post(&format!("/reports/{}/generation/start", report_id))
        .add_header("Cookie", session_cookie)
        .await;
    resp.assert_status(StatusCode::ACCEPTED);

    // Execute generation job
    let (gen_job_id, _, _) =
        laporin_backend::db::job_repo::JobRepo::claim_generation_job(&state.db)
            .await
            .unwrap()
            .unwrap();

    let template_path = state.storage.template_path();
    DocxService::ensure_default_template(&template_path).unwrap();
    let template_bytes = state.storage.read_file(&template_path).unwrap();

    let student = StudentInfo {
        full_name: "Maya Anggraini".into(),
        student_id: "20261001".into(),
        school: "SMK Negeri 1 Surabaya".into(),
        major: Some("Pengembangan Perangkat Lunak dan Gim".into()),
        semester: Some("5".into()),
    };
    let internship = InternshipInfo {
        company_name: "PT Inovasi Maju Nusantara".into(),
        company_address: Some("Jl. Basuki Rahmat No. 45, Surabaya".into()),
        department: Some("Software R&D".into()),
        role: Some("Quality Assurance Intern".into()),
        start_date: None,
        end_date: None,
        company_supervisor: Some("Pak Dimas".into()),
        school_supervisor: Some("Bu Ratna".into()),
        description: Some("Pengujian fungsional dan otomatisasi sistem informasi".into()),
    };

    let docx_bytes = DocxService::render_report(
        &template_bytes,
        "Laporan Praktik Kerja Lapangan di PT Inovasi Maju",
        &Some(student),
        &Some(internship),
        &None,
    )
    .unwrap();

    let file_uuid = Uuid::new_v4();
    let docx_path = state.storage.report_docx_path(report_uuid, file_uuid);
    state.storage.write_file(&docx_path, &docx_bytes).unwrap();
    let docx_str = docx_path.to_str().unwrap().to_string();

    let preview_path = state.storage.report_preview_path(report_uuid, file_uuid);
    let scratch_dir = state.storage.temp_job_dir(gen_job_id);
    PreviewService::convert_docx_to_pdf(&docx_path, &preview_path, &scratch_dir)
        .await
        .unwrap();
    let _ = state.storage.read_file(&preview_path).unwrap();
    let preview_str = preview_path.to_str().unwrap().to_string();

    laporin_backend::db::report_repo::ReportRepo::save_file_entry(
        &state.db,
        report_uuid,
        "docx",
        &docx_str,
        docx_bytes.len() as i64,
    )
    .await
    .unwrap();

    laporin_backend::db::report_repo::ReportRepo::save_file_entry(
        &state.db,
        report_uuid,
        "preview",
        &preview_str,
        preview_str.len() as i64,
    )
    .await
    .unwrap();

    laporin_backend::db::job_repo::JobRepo::complete_generation_job(
        &state.db,
        gen_job_id,
        &docx_str,
        &preview_str,
    )
    .await
    .unwrap();

    laporin_backend::db::report_repo::ReportRepo::update_status(
        &state.db,
        report_uuid,
        "preview_ready",
    )
    .await
    .unwrap();

    // 7. Preview PDF
    let resp = server
        .get(&format!("/reports/{}/preview", report_id))
        .add_header("Cookie", session_cookie)
        .await;
    resp.assert_status_ok();

    // 8. Create Payment
    let resp = server
        .post("/payments")
        .add_header("Cookie", session_cookie)
        .json(&json!({
            "report_id": report_id,
            "return_url": "https://app.laporin.example.com/checkout-return"
        }))
        .await;
    resp.assert_status(StatusCode::CREATED);
    let pay_data: serde_json::Value = resp.json();
    let payment_id = pay_data["payment_id"].as_str().unwrap();

    // 9. Mayar Webhook Verification & Processing
    let webhook_json = json!({
        "transaction_id": format!("mayar_e2e_{}", Uuid::new_v4()),
        "status": "success",
        "amount": 15000,
        "signature": "mock",
        "metadata": {
            "payment_id": payment_id,
            "report_id": report_id
        }
    });

    let raw_body = webhook_json.to_string();
    let sig = create_hmac_signature(&raw_body, &state.config.mayar_webhook_secret).unwrap();

    let resp = server
        .post("/webhook/mayar")
        .add_header("signature", &sig)
        .content_type("application/json")
        .bytes(raw_body.as_bytes().to_vec().into())
        .await;
    resp.assert_status_ok();

    // 10. Verify Unlocked Status & Download Final DOCX
    let resp = server
        .get(&format!("/reports/{}", report_id))
        .add_header("Cookie", session_cookie)
        .await;
    resp.assert_status_ok();
    let final_report: serde_json::Value = resp.json();
    assert_eq!(final_report["status"], "unlocked");

    let resp = server
        .get(&format!("/reports/{}/download", report_id))
        .add_header("Cookie", session_cookie)
        .await;
    resp.assert_status_ok();
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    );
    assert!(!resp.as_bytes().is_empty());
}
