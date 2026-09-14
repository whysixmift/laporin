mod common;

use axum::http::StatusCode;
use axum_test::TestServer;
use laporin_backend::domain::report::{GeneratedSections, InternshipInfo, StudentInfo};
use laporin_backend::services::{DocxService, PreviewService};
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
async fn test_docx_placeholder_replacement() {
    let temp_dir = tempfile::tempdir().unwrap();
    let template_path = temp_dir.path().join("template.docx");

    DocxService::ensure_default_template(&template_path).unwrap();
    let template_bytes = std::fs::read(&template_path).unwrap();

    let student = StudentInfo {
        full_name: "Ahmad Dahlan".into(),
        student_id: "11223344".into(),
        school: "SMK Negeri 2 Bandung".into(),
        major: Some("Teknik Komputer dan Jaringan".into()),
        semester: Some("4".into()),
    };

    let internship = InternshipInfo {
        company_name: "PT Telkom Indonesia".into(),
        company_address: Some("Jl. Japati No. 1, Bandung".into()),
        department: Some("Network Operations".into()),
        role: Some("Network Technician".into()),
        start_date: None,
        end_date: None,
        company_supervisor: Some("Pak Budi".into()),
        school_supervisor: Some("Bu Maya".into()),
        description: Some("Maintenance jaringan optik".into()),
    };

    let sections = GeneratedSections {
        cover: "COVER TEXT SAMPLE".into(),
        introduction: "INTRO TEXT SAMPLE".into(),
        company_profile: "COMPANY PROFILE SAMPLE".into(),
        activities: "ACTIVITIES SAMPLE".into(),
        conclusion: "CONCLUSION SAMPLE".into(),
        is_user_edited: Some(false),
    };

    let rendered_bytes = DocxService::render_report(
        &template_bytes,
        "Laporan PKL Ahmad Dahlan",
        &Some(student),
        &Some(internship),
        &Some(sections),
    )
    .unwrap();

    assert!(!rendered_bytes.is_empty());

    // Validate generated zip archive contains replaced strings
    let reader = std::io::Cursor::new(rendered_bytes);
    let mut zip = zip::ZipArchive::new(reader).unwrap();
    let mut doc_xml = String::new();
    std::io::Read::read_to_string(&mut zip.by_name("word/document.xml").unwrap(), &mut doc_xml)
        .unwrap();

    assert!(doc_xml.contains("Ahmad Dahlan"));
    assert!(doc_xml.contains("PT Telkom Indonesia"));
    assert!(doc_xml.contains("INTRO TEXT SAMPLE"));
    assert!(!doc_xml.contains("{{student_name}}"));
}

#[tokio::test]
async fn test_generation_and_preview_flow() {
    let (router, state) = common::setup_test_app().await;
    let server = TestServer::new(router).unwrap();

    let user_cookie = create_authenticated_user(&server, "author").await;

    // Create Report
    let create_payload = json!({
        "title": "Laporan Magang IT Support",
        "student": {
            "full_name": "Siti Nurhaliza",
            "student_id": "99887766",
            "school": "Politeknik Negeri Jakarta"
        },
        "internship": {
            "company_name": "PT Sumber Alfaria Trijaya"
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

    // 1. Preview request before generation returns 400 Bad Request
    let resp = server
        .get(&format!("/reports/{}/preview", report_id))
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status_bad_request();

    // 2. Start Generation Job
    let resp = server
        .post(&format!("/reports/{}/generation/start", report_id))
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status(StatusCode::ACCEPTED);

    // 3. Process Generation Job
    let (claimed_job_id, _, _) =
        laporin_backend::db::job_repo::JobRepo::claim_generation_job(&state.db)
            .await
            .unwrap()
            .expect("Should claim generation job");

    // Perform generation directly
    let template_path = state.storage.template_path();
    DocxService::ensure_default_template(&template_path).unwrap();
    let template_bytes = state.storage.read_file(&template_path).unwrap();

    let student = StudentInfo {
        full_name: "Siti Nurhaliza".into(),
        student_id: "99887766".into(),
        school: "Politeknik Negeri Jakarta".into(),
        major: None,
        semester: None,
    };
    let internship = InternshipInfo {
        company_name: "PT Sumber Alfaria Trijaya".into(),
        company_address: None,
        department: None,
        role: None,
        start_date: None,
        end_date: None,
        company_supervisor: None,
        school_supervisor: None,
        description: None,
    };

    let docx_bytes = DocxService::render_report(
        &template_bytes,
        "Laporan Magang IT Support",
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
    let scratch_dir = state.storage.temp_job_dir(claimed_job_id);
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
        claimed_job_id,
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

    // 4. Download PDF Preview
    let resp = server
        .get(&format!("/reports/{}/preview", report_id))
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status_ok();
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "application/pdf"
    );
    assert!(!resp.as_bytes().is_empty());
}
