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

#[tokio::test]
async fn test_logbook_crud_and_synthesis() {
    let (router, _) = common::setup_test_app().await;
    let server = TestServer::new(router).unwrap();

    let user_cookie = create_authenticated_user(&server, "charlie").await;

    // 1. Create Report
    let create_payload = json!({
        "title": "Laporan PKL TKJ",
        "student": {
            "full_name": "Charlie Chaplin",
            "student_id": "99887766",
            "school": "SMK Negeri 4 Bandung",
            "major": "Teknik Komputer Jaringan",
            "semester": "5"
        },
        "internship": {
            "company_name": "PT Jaringan Nusantara",
            "company_address": "Jl. Merdeka 45, Bandung",
            "department": "Network Ops",
            "role": "Intern",
            "start_date": "2026-02-01",
            "end_date": "2026-05-01",
            "company_supervisor": "Pak Ahmad",
            "school_supervisor": "Bu Siti",
            "description": "Konfigurasi routing dan switching"
        }
    });

    let resp = server
        .post("/reports")
        .add_header("Cookie", &user_cookie)
        .json(&create_payload)
        .await;
    resp.assert_status(StatusCode::CREATED);
    let created: serde_json::Value = resp.json();
    let report_id = created["id"].as_str().unwrap();

    // 2. Add Logbook Entry
    let logbook_payload = json!({
        "entry_date": "2026-02-05",
        "activity_title": "Konfigurasi VLAN Switch Core",
        "tasks_performed": "Membuat VLAN 10 (Management) dan VLAN 20 (Users) pada Cisco Catalyst 2960",
        "tools_technologies": "Cisco CLI, Console Cable, Putty",
        "problems_encountered": "Trunk port tidak melewatkan VLAN 20",
        "solutions_applied": "Menambahkan switchport trunk allowed vlan add 20",
        "skills_learned": "VLAN Trunking Protocol 802.1Q",
        "evidence_notes": "Screenshot konfigurasi terminal tersimpan"
    });

    let resp = server
        .post(&format!("/reports/{}/logbook", report_id))
        .add_header("Cookie", &user_cookie)
        .json(&logbook_payload)
        .await;
    resp.assert_status(StatusCode::CREATED);
    let entry: serde_json::Value = resp.json();
    let entry_id = entry["id"].as_str().unwrap();
    assert_eq!(entry["activity_title"], "Konfigurasi VLAN Switch Core");
    assert_eq!(entry["tools_technologies"], "Cisco CLI, Console Cable, Putty");

    // 3. List Logbook Entries
    let resp = server
        .get(&format!("/reports/{}/logbook", report_id))
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status_ok();
    let entries: Vec<serde_json::Value> = resp.json();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0]["id"], entry_id);

    // 4. Verify logbook_count on report
    let resp = server
        .get(&format!("/reports/{}", report_id))
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status_ok();
    let rep: serde_json::Value = resp.json();
    assert_eq!(rep["logbook_count"], 1);

    // 5. In-Place Chapter Edit
    let sections_payload = json!({
        "activities": "BAB III HASIL EDIT MANUAL PENGGUNA\n3.1 Aktivitas Jaringan VLAN...",
        "conclusion": "BAB IV KESIMPULAN DARI PENGGUNA..."
    });

    let resp = server
        .patch(&format!("/reports/{}/sections", report_id))
        .add_header("Cookie", &user_cookie)
        .json(&sections_payload)
        .await;
    resp.assert_status_ok();
    let updated_sections: serde_json::Value = resp.json();
    assert_eq!(updated_sections["is_user_edited"], true);
    assert!(updated_sections["activities"].as_str().unwrap().contains("HASIL EDIT MANUAL PENGGUNA"));

    // 6. Delete Logbook Entry
    let resp = server
        .delete(&format!("/reports/{}/logbook/{}", report_id, entry_id))
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status_ok();

    // Verify empty list
    let resp = server
        .get(&format!("/reports/{}/logbook", report_id))
        .add_header("Cookie", &user_cookie)
        .await;
    resp.assert_status_ok();
    let entries_after: Vec<serde_json::Value> = resp.json();
    assert_eq!(entries_after.len(), 0);
}
