use crate::db::job_repo::JobRepo;
use crate::db::report_repo::ReportRepo;
use crate::domain::job::JobInfo;
use crate::errors::AppError;
use crate::middleware::AuthenticatedUser;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

pub async fn start_research_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<JobInfo>), AppError> {
    // 1. Verify ownership
    let (_report, owner_id) = ReportRepo::get_report_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

    if owner_id != auth.user_id {
        return Err(AppError::Forbidden(
            "You do not have access to this report".into(),
        ));
    }

    // 2. Prevent duplicate active jobs
    if JobRepo::has_active_research_job(&state.db, id).await? {
        return Err(AppError::JobAlreadyActive);
    }

    // 3. Enqueue job
    let job = JobRepo::enqueue_research_job(&state.db, id).await?;

    // 4. Update report status to researching
    ReportRepo::update_status(&state.db, id, "researching").await?;

    Ok((StatusCode::ACCEPTED, Json(job)))
}

pub async fn get_research_status_handler(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<JobInfo>, AppError> {
    let (_, owner_id) = ReportRepo::get_report_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

    if owner_id != auth.user_id {
        return Err(AppError::Forbidden(
            "You do not have access to this report".into(),
        ));
    }

    let job = JobRepo::get_latest_research_job(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("No research job found for this report".into()))?;

    Ok(Json(job))
}
