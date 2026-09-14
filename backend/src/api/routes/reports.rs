use crate::db::report_repo::ReportRepo;
use crate::domain::report::{Report, ReportCreate, ReportUpdate};
use crate::errors::AppError;
use crate::middleware::AuthenticatedUser;
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

