use crate::db::report_repo::ReportRepo;
use crate::domain::report::{
    GeneratedSections, GeneratedSectionsUpdate, LogbookEntry, LogbookEntryCreate, Report,
    ReportCreate, ReportUpdate,
};
use crate::errors::AppError;
use crate::middleware::AuthenticatedUser;
use crate::services::{DocxService, PreviewService};
use crate::state::AppState;
use axum::{
    Json,
    body::Body,
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ListReportQuery {
    pub status: Option<String>,
}

pub async fn create_report_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(payload): Json<ReportCreate>,
) -> Result<(StatusCode, Json<Report>), AppError> {
    // 1. Check disk space (< 1GB reject with 503)
    let free_disk = state.storage.check_available_disk_space();
    if free_disk < 1024 * 1024 * 1024 {
        return Err(AppError::InsufficientStorage);
    }

    // 2. Validate input
    if payload.title.trim().is_empty() {
        return Err(AppError::ValidationError("Title is required".into()));
    }
    if payload.student.full_name.trim().is_empty()
        || payload.student.student_id.trim().is_empty()
        || payload.student.school.trim().is_empty()
    {
        return Err(AppError::ValidationError(
            "Complete student info is required".into(),
        ));
    }
    if payload.internship.company_name.trim().is_empty() {
        return Err(AppError::ValidationError("Company name is required".into()));
    }

    // 3. Create report
    let report = ReportRepo::create_report(&state.db, auth.user_id, payload).await?;

    Ok((StatusCode::CREATED, Json(report)))
}

pub async fn list_reports_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Query(query): Query<ListReportQuery>,
) -> Result<Json<Vec<Report>>, AppError> {
    let reports =
        ReportRepo::list_reports_by_user(&state.db, auth.user_id, query.status.as_deref()).await?;

    Ok(Json(reports))
}

pub async fn get_report_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Report>, AppError> {
    let (report, owner_id) = ReportRepo::get_report_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

    if owner_id != auth.user_id && auth.role != "admin" {
        return Err(AppError::Forbidden(
            "You do not have access to this report".into(),
        ));
    }

    Ok(Json(report))
}

pub async fn update_report_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(update): Json<ReportUpdate>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (_, owner_id) = ReportRepo::get_report_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

    if owner_id != auth.user_id && auth.role != "admin" {
        return Err(AppError::Forbidden(
            "You do not have access to this report".into(),
        ));
    }

    ReportRepo::update_report(&state.db, id, update).await?;

    Ok(Json(json!({ "message": "OK" })))
}

pub async fn get_preview_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
    let (report, owner_id) = ReportRepo::get_report_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

    if owner_id != auth.user_id && auth.role != "admin" {
        return Err(AppError::Forbidden(
            "You do not have access to this preview".into(),
        ));
    }

    // Must be in ready states
    let allowed_status = ["preview_ready", "payment_pending", "paid", "unlocked"];
    if !allowed_status.contains(&report.status.as_str()) {
        return Err(AppError::InvalidRequest(format!(
            "Preview not available for report with status '{}'",
            report.status
        )));
    }

    let file_path_str = ReportRepo::get_latest_file(&state.db, id, "preview")
        .await?
        .ok_or_else(|| AppError::NotFound("Preview file not found".into()))?;

    let file_path = PathBuf::from(file_path_str);
    if !state.storage.file_exists(&file_path) {
        return Err(AppError::NotFound("Physical preview file not found".into()));
    }

    let file = tokio::fs::File::open(&file_path)
        .await
        .map_err(|e| AppError::Io(e))?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(
            header::CONTENT_DISPOSITION,
            format!("inline; filename=\"preview-{}.pdf\"", id),
        )
        .header(
            header::CACHE_CONTROL,
            "private, no-cache, no-store, must-revalidate",
        )
        .header("X-Content-Type-Options", "nosniff")
        .body(body)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(response)
}

pub async fn download_report_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
    let (report, owner_id) = ReportRepo::get_report_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

    if owner_id != auth.user_id && auth.role != "admin" {
        return Err(AppError::Forbidden(
            "You do not have access to this report".into(),
        ));
    }

    // Only unlocked or paid reports can download final DOCX (admins can download anytime if file exists)
    if report.status != "unlocked" && report.status != "paid" && auth.role != "admin" {
        return Err(AppError::Forbidden(
            "Report must be unlocked/paid to download the final document".into(),
        ));
    }

    let file_path_str = ReportRepo::get_latest_file(&state.db, id, "docx")
        .await?
        .ok_or_else(|| AppError::NotFound("DOCX document file not found".into()))?;

    let file_path = PathBuf::from(file_path_str);
    if !state.storage.file_exists(&file_path) {
        return Err(AppError::NotFound(
            "Physical document file not found".into(),
        ));
    }

    let file = tokio::fs::File::open(&file_path)
        .await
        .map_err(|e| AppError::Io(e))?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let filename = format!("laporan-{}.docx", id);
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        )
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(body)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(response)
}

pub async fn free_unlock_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    if auth.role != "admin" {
        return Err(AppError::Forbidden("Only administrators can perform free report unlocks".into()));
    }

    let (report, _) = ReportRepo::get_report_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

    ReportRepo::update_status(&state.db, id, "unlocked").await?;

    let payment_id = Uuid::new_v4();
    let _ = sqlx::query!(
        r#"
        INSERT INTO payments (id, report_id, status, amount_cents, created_at, updated_at)
        VALUES ($1, $2, 'succeeded', 0, now(), now())
        ON CONFLICT DO NOTHING
        "#,
        payment_id,
        id
    )
    .execute(&state.db)
    .await;

    if report.generated_sections.is_some() && report.generated_doc_path.is_none() {
        let _ = crate::db::job_repo::JobRepo::enqueue_generation_job(&state.db, id).await;
    }

    tracing::info!(report_id = %id, admin_email = %auth.email, "Admin performed Free Instant Unlock on report");

    Ok(Json(json!({
        "message": "Report unlocked for free successfully",
        "status": "unlocked",
        "report_id": id
    })))
}

pub async fn update_report_sections_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(update): Json<GeneratedSectionsUpdate>,
) -> Result<Json<GeneratedSections>, AppError> {
    let (report, owner_id) = ReportRepo::get_report_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

    if owner_id != auth.user_id && auth.role != "admin" {
        return Err(AppError::Forbidden(
            "You do not have access to edit this report".into(),
        ));
    }

    let updated_sections = ReportRepo::update_generated_sections(&state.db, id, update).await?;

    // Re-render DOCX and preview if report has student and internship details
    if let (Some(student), Some(internship)) = (&report.student, &report.internship) {
        let template_path = state.storage.template_path();
        let _ = DocxService::ensure_default_template(&template_path);
        if let Ok(template_bytes) = state.storage.read_file(&template_path) {
            if let Ok(docx_bytes) = DocxService::render_report(
                &template_bytes,
                &report.title,
                &Some(student.clone()),
                &Some(internship.clone()),
                &Some(updated_sections.clone()),
            ) {
                let file_uuid = Uuid::new_v4();
                let docx_out_path = state.storage.report_docx_path(id, file_uuid);
                if state.storage.write_file(&docx_out_path, &docx_bytes).is_ok() {
                    let docx_str = docx_out_path.to_str().unwrap_or_default().to_string();
                    let _ = ReportRepo::save_file_entry(
                        &state.db,
                        id,
                        "docx",
                        &docx_str,
                        docx_bytes.len() as i64,
                    )
                    .await;

                    // Convert to preview PDF as well
                    let preview_out_path = state.storage.report_preview_path(id, file_uuid);
                    let scratch_dir = state.storage.temp_job_dir(file_uuid);
                    if PreviewService::convert_docx_to_pdf(&docx_out_path, &preview_out_path, &scratch_dir)
                        .await
                        .is_ok()
                    {
                        let _ = state.storage.delete_dir(&scratch_dir);
                        if let Ok(preview_bytes) = state.storage.read_file(&preview_out_path) {
                            let preview_str = preview_out_path.to_str().unwrap_or_default().to_string();
                            let _ = ReportRepo::save_file_entry(
                                &state.db,
                                id,
                                "preview",
                                &preview_str,
                                preview_bytes.len() as i64,
                            )
                            .await;
                        }
                    }
                }
            }
        }
    }

    Ok(Json(updated_sections))
}

pub async fn create_logbook_entry_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<LogbookEntryCreate>,
) -> Result<(StatusCode, Json<LogbookEntry>), AppError> {
    let (_, owner_id) = ReportRepo::get_report_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

    if owner_id != auth.user_id && auth.role != "admin" {
        return Err(AppError::Forbidden(
            "You do not have access to this report".into(),
        ));
    }

    if payload.activity_title.trim().is_empty() || payload.tasks_performed.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Judul kegiatan dan rincian tugas wajib diisi".into(),
        ));
    }

    let entry = ReportRepo::create_logbook_entry(&state.db, id, payload).await?;
    Ok((StatusCode::CREATED, Json(entry)))
}

pub async fn list_logbook_entries_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<LogbookEntry>>, AppError> {
    let (_, owner_id) = ReportRepo::get_report_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

    if owner_id != auth.user_id && auth.role != "admin" {
        return Err(AppError::Forbidden(
            "You do not have access to this report".into(),
        ));
    }

    let entries = ReportRepo::list_logbook_entries(&state.db, id).await?;
    Ok(Json(entries))
}

pub async fn delete_logbook_entry_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path((id, entry_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (_, owner_id) = ReportRepo::get_report_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

    if owner_id != auth.user_id && auth.role != "admin" {
        return Err(AppError::Forbidden(
            "You do not have access to this report".into(),
        ));
    }

    let deleted = ReportRepo::delete_logbook_entry(&state.db, id, entry_id).await?;
    if !deleted {
        return Err(AppError::NotFound("Logbook entry not found".into()));
    }

    Ok(Json(json!({ "message": "Logbook entry deleted successfully" })))
}

