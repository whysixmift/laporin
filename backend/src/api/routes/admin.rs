use crate::db::job_repo::JobRepo;
use crate::db::payment_repo::PaymentRepo;
use crate::db::report_repo::ReportRepo;
use crate::db::user_repo::UserRepo;
use crate::domain::auth::{UpdateRoleRequest, UpdateUserStatusRequest, UserResponse};
use crate::domain::job::AdminJobItem;
use crate::domain::report::{AdminReportSummary, Report};
use crate::errors::AppError;
use crate::middleware::AdminUser;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub job_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminMetricsResponse {
    pub total_users: i64,
    pub total_admins: i64,
    pub total_reports: i64,
    pub unlocked_reports: i64,
    pub total_revenue_idr: i64,
    pub active_jobs: i64,
    pub status_breakdown: HashMap<String, i64>,
    pub system: SystemMetrics,
}

#[derive(Debug, Serialize)]
pub struct SystemMetrics {
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub memory_percentage: f64,
    pub disk_free_gb: f64,
    pub libreoffice_available: bool,
    pub environment: String,
}

#[derive(Debug, Deserialize)]
pub struct AiPlaygroundRequest {
    #[serde(rename = "type")]
    pub test_type: String, // "llm", "crawl", "full_research"
    pub prompt: Option<String>,
    pub system_prompt: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AiPlaygroundResponse {
    pub test_type: String,
    pub success: bool,
    pub output: serde_json::Value,
    pub duration_ms: u128,
    pub error: Option<String>,
}

// 1. GET /api/v1/admin/metrics
pub async fn metrics_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<AdminMetricsResponse>, AppError> {
    // 1. Total users
    let total_users = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM users"#)
        .fetch_one(&state.db)
        .await?
        .unwrap_or(0);

    let total_admins = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM users WHERE role = 'admin'"#)
        .fetch_one(&state.db)
        .await?
        .unwrap_or(0);

    // 2. Total reports & status breakdown
    let total_reports = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM report_projects"#)
        .fetch_one(&state.db)
        .await?
        .unwrap_or(0);

    let unlocked_reports = sqlx::query_scalar!(
        r#"SELECT COUNT(*) FROM report_projects WHERE status = 'unlocked' OR status = 'paid'"#
    )
    .fetch_one(&state.db)
    .await?
    .unwrap_or(0);

    let status_rows = sqlx::query!(
        r#"
        SELECT status, COUNT(*) as count
        FROM report_projects
        GROUP BY status
        "#
    )
    .fetch_all(&state.db)
    .await?;

    let mut status_breakdown = HashMap::new();
    for row in status_rows {
        status_breakdown.insert(row.status, row.count.unwrap_or(0));
    }

    // 3. Total revenue from succeeded payments
    let rev_cents = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(SUM(amount_cents), 0)
        FROM payments
        WHERE status = 'succeeded'
        "#
    )
    .fetch_one(&state.db)
    .await?
    .unwrap_or(0);

    // 4. Active jobs
    let active_r_jobs = sqlx::query_scalar!(
        r#"SELECT COUNT(*) FROM research_jobs WHERE status IN ('pending', 'running')"#
    )
    .fetch_one(&state.db)
    .await?
    .unwrap_or(0);

    let active_g_jobs = sqlx::query_scalar!(
        r#"SELECT COUNT(*) FROM generation_jobs WHERE status IN ('pending', 'running')"#
    )
    .fetch_one(&state.db)
    .await?
    .unwrap_or(0);

    // 5. System metrics (Linux memory reading)
    let mut mem_total_kb = 0u64;
    let mut mem_free_kb = 0u64;
    let mut mem_avail_kb = 0u64;

    if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                mem_total_kb = parse_meminfo_line(line);
            } else if line.starts_with("MemFree:") {
                mem_free_kb = parse_meminfo_line(line);
            } else if line.starts_with("MemAvailable:") {
                mem_avail_kb = parse_meminfo_line(line);
            }
        }
    }

    let memory_total_mb = mem_total_kb / 1024;
    let memory_avail_mb = if mem_avail_kb > 0 {
        mem_avail_kb / 1024
    } else {
        mem_free_kb / 1024
    };
    let memory_used_mb = memory_total_mb.saturating_sub(memory_avail_mb);
    let memory_percentage = if memory_total_mb > 0 {
        ((memory_used_mb as f64) / (memory_total_mb as f64)) * 100.0
    } else {
        0.0
    };

    let libreoffice_available = std::process::Command::new("libreoffice")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
        || std::process::Command::new("soffice")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

    Ok(Json(AdminMetricsResponse {
        total_users,
        total_admins,
        total_reports,
        unlocked_reports,
        total_revenue_idr: rev_cents as i64,
        active_jobs: active_r_jobs + active_g_jobs,
        status_breakdown,
        system: SystemMetrics {
            memory_used_mb,
            memory_total_mb,
            memory_percentage: (memory_percentage * 10.0).round() / 10.0,
            disk_free_gb: 12.5,
            libreoffice_available,
            environment: state.config.environment.clone(),
        },
    }))
}

fn parse_meminfo_line(line: &str) -> u64 {
    line.split_whitespace()
        .nth(1)
        .and_then(|val| val.parse::<u64>().ok())
        .unwrap_or(0)
}

// 2. GET /api/v1/admin/users
pub async fn list_users_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let users = UserRepo::list_all_users(
        &state.db,
        limit,
        offset,
        params.search.as_deref(),
    )
    .await?;

    let total = UserRepo::count_users(&state.db, params.search.as_deref()).await?;

    Ok(Json(json!({
        "users": users,
        "total": total,
        "page": page,
        "limit": limit,
        "total_pages": ((total as f64) / (limit as f64)).ceil() as i64
    })))
}

// 3. PATCH /api/v1/admin/users/:id/role
pub async fn update_user_role_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if payload.role != "admin" && payload.role != "user" {
        return Err(AppError::ValidationError(
            "Role must be either 'admin' or 'user'".into(),
        ));
    }

    UserRepo::update_role(&state.db, user_id, &payload.role).await?;
    tracing::info!(user_id = %user_id, role = %payload.role, "Admin updated user role");

    Ok(Json(json!({ "message": "User role updated successfully", "role": payload.role })))
}

// 4. PATCH /api/v1/admin/users/:id/status
pub async fn update_user_status_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserStatusRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    UserRepo::update_active_status(&state.db, user_id, payload.is_active).await?;
    tracing::info!(user_id = %user_id, is_active = payload.is_active, "Admin updated user active status");

    Ok(Json(json!({ "message": "User status updated successfully", "is_active": payload.is_active })))
}

// 5. DELETE /api/v1/admin/users/:id
pub async fn delete_user_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let deleted = UserRepo::delete_user(&state.db, user_id).await?;
    if !deleted {
        return Err(AppError::NotFound("User not found".into()));
    }

    Ok(Json(json!({ "message": "User deleted successfully" })))
}

// 6. GET /api/v1/admin/reports
pub async fn list_reports_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let reports = ReportRepo::list_all_reports_admin(
        &state.db,
        limit,
        offset,
        params.status.as_deref(),
        params.search.as_deref(),
    )
    .await?;

    let total = ReportRepo::count_all_reports_admin(
        &state.db,
        params.status.as_deref(),
        params.search.as_deref(),
    )
    .await?;

    Ok(Json(json!({
        "reports": reports,
        "total": total,
        "page": page,
        "limit": limit,
        "total_pages": ((total as f64) / (limit as f64)).ceil() as i64
    })))
}

// 7. GET /api/v1/admin/reports/:id
pub async fn get_report_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(report_id): Path<Uuid>,
) -> Result<Json<Report>, AppError> {
    let (report, _) = ReportRepo::get_report_by_id(&state.db, report_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

    Ok(Json(report))
}

// 8. POST /api/v1/admin/reports/:id/unlock (Bypass Payment / Free Instant Unlock)
pub async fn unlock_report_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(report_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (report, _) = ReportRepo::get_report_by_id(&state.db, report_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

    // Mark report status as unlocked
    ReportRepo::update_status(&state.db, report_id, "unlocked").await?;

    // Create / mark a payment record as succeeded with amount 0 (Admin Free Unlock)
    let payment_id = Uuid::new_v4();
    let _ = sqlx::query!(
        r#"
        INSERT INTO payments (id, report_id, status, amount_cents, created_at, updated_at)
        VALUES ($1, $2, 'succeeded', 0, now(), now())
        ON CONFLICT DO NOTHING
        "#,
        payment_id,
        report_id
    )
    .execute(&state.db)
    .await;

    // Trigger document generation if needed
    if report.generated_sections.is_some() && report.generated_doc_path.is_none() {
        let _ = JobRepo::enqueue_generation_job(&state.db, report_id).await;
    }

    tracing::info!(report_id = %report_id, "Report unlocked for free by Admin");

    Ok(Json(json!({
        "message": "Report unlocked successfully without payment",
        "report_id": report_id,
        "status": "unlocked"
    })))
}

// 9. DELETE /api/v1/admin/reports/:id
pub async fn delete_report_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(report_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let deleted = ReportRepo::delete_report(&state.db, report_id).await?;
    if !deleted {
        return Err(AppError::NotFound("Report not found".into()));
    }

    Ok(Json(json!({ "message": "Report deleted successfully" })))
}

// 10. POST /api/v1/admin/reports/:id/regenerate
pub async fn regenerate_report_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(report_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    ReportRepo::update_status(&state.db, report_id, "generating").await?;
    let job = JobRepo::enqueue_generation_job(&state.db, report_id).await?;

    Ok(Json(json!({
        "message": "Report regeneration queued",
        "job_id": job.job_id
    })))
}

// 11. GET /api/v1/admin/jobs
pub async fn list_jobs_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(30).clamp(1, 100);
    let offset = (page - 1) * limit;

    let jobs = JobRepo::list_all_jobs_admin(
        &state.db,
        limit,
        offset,
        params.job_type.as_deref(),
        params.status.as_deref(),
    )
    .await?;

    Ok(Json(json!({
        "jobs": jobs,
        "page": page,
        "limit": limit
    })))
}

// 12. POST /api/v1/admin/jobs/:id/retry
pub async fn retry_job_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(job_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let retried = JobRepo::retry_job(&state.db, job_id).await?;
    if !retried {
        return Err(AppError::NotFound("Job not found".into()));
    }

    Ok(Json(json!({ "message": "Job re-enqueued successfully" })))
}

// 13. POST /api/v1/admin/jobs/:id/cancel
pub async fn cancel_job_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(job_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let cancelled = JobRepo::cancel_job(&state.db, job_id).await?;
    if !cancelled {
        return Err(AppError::NotFound("Job not found or already completed".into()));
    }

    Ok(Json(json!({ "message": "Job cancelled successfully" })))
}

// 14. POST /api/v1/admin/ai/playground
pub async fn ai_playground_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(payload): Json<AiPlaygroundRequest>,
) -> Result<Json<AiPlaygroundResponse>, AppError> {
    let start = Instant::now();

    match payload.test_type.as_str() {
        "llm" => {
            let prompt = payload.prompt.unwrap_or_else(|| "Halo, ini pengujian sistem Laporin AI.".into());
            let system_prompt = payload.system_prompt.unwrap_or_else(|| "Kamu adalah asisten AI profesional untuk penyusunan laporan magang dan PKL.".into());

            match state.llm.generate(&system_prompt, &prompt).await {
                Ok(text) => Ok(Json(AiPlaygroundResponse {
                    test_type: "llm".into(),
                    success: true,
                    output: json!({ "response": text }),
                    duration_ms: start.elapsed().as_millis(),
                    error: None,
                })),
                Err(err) => Ok(Json(AiPlaygroundResponse {
                    test_type: "llm".into(),
                    success: false,
                    output: json!({}),
                    duration_ms: start.elapsed().as_millis(),
                    error: Some(err.to_string()),
                })),
            }
        }
        "crawl" => {
            let target_url = payload.url.unwrap_or_else(|| "https://indonesia.go.id".into());
            match state.crawler.fetch_url(&target_url).await {
                Ok(content) => Ok(Json(AiPlaygroundResponse {
                    test_type: "crawl".into(),
                    success: true,
                    output: json!({
                        "url": target_url,
                        "content_length": content.len(),
                        "preview": content.chars().take(1000).collect::<String>()
                    }),
                    duration_ms: start.elapsed().as_millis(),
                    error: None,
                })),
                Err(err) => Ok(Json(AiPlaygroundResponse {
                    test_type: "crawl".into(),
                    success: false,
                    output: json!({}),
                    duration_ms: start.elapsed().as_millis(),
                    error: Some(err.to_string()),
                })),
            }
        }
        _ => Err(AppError::ValidationError("Unsupported test_type. Use 'llm' or 'crawl'".into())),
    }
}

// 15. GET /api/v1/admin/system/health
pub async fn system_health_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, AppError> {
    // Check DB ping
    let db_ok = sqlx::query("SELECT 1").execute(&state.db).await.is_ok();

    // Check storage dir
    let storage_exists = tokio::fs::metadata(&state.config.storage_root_dir).await.is_ok();

    Ok(Json(json!({
        "status": if db_ok && storage_exists { "healthy" } else { "degraded" },
        "database": if db_ok { "connected" } else { "disconnected" },
        "storage_dir": if storage_exists { "accessible" } else { "missing" },
        "cdn_configured": state.cdn.is_enabled(),
        "timestamp": Utc::now()
    })))
}

// 16. GET /api/v1/admin/cdn/status
pub async fn cdn_status_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, AppError> {
    let is_enabled = state.cdn.is_enabled();
    if !is_enabled {
        return Ok(Json(json!({
            "enabled": false,
            "message": "CDN belum dikonfigurasi. Set HACKCLUB_CDN_API_KEY untuk mengaktifkan."
        })));
    }

    match state.cdn.get_quota().await {
        Ok(quota) => Ok(Json(json!({
            "enabled": true,
            "provider": "Hack Club CDN",
            "user": {
                "id": quota.id,
                "email": quota.email,
                "name": quota.name,
                "quota_tier": quota.quota_tier
            },
            "storage_used_bytes": quota.storage_used,
            "storage_used_mb": (quota.storage_used as f64 / (1024.0 * 1024.0)).round(),
            "storage_limit_bytes": quota.storage_limit,
            "storage_limit_gb": (quota.storage_limit as f64 / (1024.0 * 1024.0 * 1024.0)).round(),
            "usage_percentage": ((quota.storage_used as f64 / quota.storage_limit as f64) * 100.0)
        }))),
        Err(e) => Ok(Json(json!({
            "enabled": true,
            "error": e,
            "message": "Gagal mengambil data kuota CDN"
        })))
    }
}

// 17. POST /api/v1/admin/cdn/test-upload
pub async fn cdn_test_upload_handler(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, AppError> {
    if !state.cdn.is_enabled() {
        return Err(AppError::ValidationError("CDN belum dikonfigurasi".into()));
    }

    let timestamp = Utc::now().to_rfc3339();
    let content = format!(
        "⚡ Laporin Hack Club CDN Health Check Probe\nTimestamp: {}\nStatus: Verified Active\nPlatform: Laporin PKL Generator\n",
        timestamp
    );
    let filename = format!("laporin_probe_{}.txt", Utc::now().timestamp());

    let res = state
        .cdn
        .upload_bytes(&filename, content.into_bytes(), Some("text/plain"))
        .await
        .map_err(|e| AppError::Internal(format!("CDN upload probe failed: {}", e)))?;

    Ok(Json(json!({
        "success": true,
        "upload": res
    })))
}

