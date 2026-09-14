use crate::domain::job::{AdminJobItem, JobInfo};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct JobRepo;

impl JobRepo {
    pub async fn has_active_research_job(
        pool: &Pool<Postgres>,
        report_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id FROM research_jobs
            WHERE report_id = $1 AND status IN ('pending', 'running')
            LIMIT 1
            "#,
            report_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.is_some())
    }

    pub async fn has_active_generation_job(
        pool: &Pool<Postgres>,
        report_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id FROM generation_jobs
            WHERE report_id = $1 AND status IN ('pending', 'running')
            LIMIT 1
            "#,
            report_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.is_some())
    }

    pub async fn enqueue_research_job(
        pool: &Pool<Postgres>,
        report_id: Uuid,
    ) -> Result<JobInfo, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO research_jobs (id, report_id, status, attempts, max_attempts, created_at)
            VALUES ($1, $2, 'pending', 0, 3, $3)
            "#,
            id,
            report_id,
            now
        )
        .execute(pool)
        .await?;

        Ok(JobInfo {
            job_id: id,
            job_type: "research".into(),
            status: "pending".into(),
            started_at: None,
            finished_at: None,
            error_code: None,
            error_message: None,
        })
    }

    pub async fn enqueue_generation_job(
        pool: &Pool<Postgres>,
        report_id: Uuid,
    ) -> Result<JobInfo, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO generation_jobs (id, report_id, status, attempts, max_attempts, created_at)
            VALUES ($1, $2, 'pending', 0, 3, $3)
            "#,
            id,
            report_id,
            now
        )
        .execute(pool)
        .await?;

        Ok(JobInfo {
            job_id: id,
            job_type: "generation".into(),
            status: "pending".into(),
            started_at: None,
            finished_at: None,
            error_code: None,
            error_message: None,
        })
    }

    pub async fn claim_research_job(
        pool: &Pool<Postgres>,
    ) -> Result<Option<(Uuid, Uuid, i32)>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            UPDATE research_jobs
            SET status = 'running', started_at = now(), attempts = attempts + 1
            WHERE id = (
                SELECT id FROM research_jobs
                WHERE status = 'pending'
                ORDER BY created_at ASC
                FOR UPDATE SKIP LOCKED
                LIMIT 1
            )
            RETURNING id, report_id, attempts
            "#
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| (r.id, r.report_id, r.attempts)))
    }

    pub async fn claim_generation_job(
        pool: &Pool<Postgres>,
    ) -> Result<Option<(Uuid, Uuid, i32)>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            UPDATE generation_jobs
            SET status = 'running', started_at = now(), attempts = attempts + 1
            WHERE id = (
                SELECT id FROM generation_jobs
                WHERE status = 'pending'
                ORDER BY created_at ASC
                FOR UPDATE SKIP LOCKED
                LIMIT 1
            )
            RETURNING id, report_id, attempts
            "#
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| (r.id, r.report_id, r.attempts)))
    }

    pub async fn complete_research_job(
        pool: &Pool<Postgres>,
        job_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE research_jobs
            SET status = 'succeeded', finished_at = now()
            WHERE id = $1
            "#,
            job_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn fail_research_job(
        pool: &Pool<Postgres>,
        job_id: Uuid,
        error_code: &str,
        error_message: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE research_jobs
            SET status = 'failed', finished_at = now(), error_code = $2, error_message = $3
            WHERE id = $1
            "#,
            job_id,
            error_code,
            error_message
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn complete_generation_job(
        pool: &Pool<Postgres>,
        job_id: Uuid,
        doc_path: &str,
        preview_path: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE generation_jobs
            SET status = 'succeeded', finished_at = now(), doc_path = $2, preview_path = $3
            WHERE id = $1
            "#,
            job_id,
            doc_path,
            preview_path
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn fail_generation_job(
        pool: &Pool<Postgres>,
        job_id: Uuid,
        error_code: &str,
        error_message: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE generation_jobs
            SET status = 'failed', finished_at = now(), error_code = $2, error_message = $3
            WHERE id = $1
            "#,
            job_id,
            error_code,
            error_message
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_latest_research_job(
        pool: &Pool<Postgres>,
        report_id: Uuid,
    ) -> Result<Option<JobInfo>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, status, started_at, finished_at, error_code, error_message
            FROM research_jobs
            WHERE report_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            report_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| JobInfo {
            job_id: r.id,
            job_type: "research".into(),
            status: r.status,
            started_at: r.started_at,
            finished_at: r.finished_at,
            error_code: r.error_code,
            error_message: r.error_message,
        }))
    }

    pub async fn get_latest_generation_job(
        pool: &Pool<Postgres>,
        report_id: Uuid,
    ) -> Result<Option<JobInfo>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, status, started_at, finished_at, error_code, error_message
            FROM generation_jobs
            WHERE report_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            report_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| JobInfo {
            job_id: r.id,
            job_type: "generation".into(),
            status: r.status,
            started_at: r.started_at,
            finished_at: r.finished_at,
            error_code: r.error_code,
            error_message: r.error_message,
        }))
    }

    pub async fn list_all_jobs_admin(
        pool: &Pool<Postgres>,
        limit: i64,
        offset: i64,
        job_type: Option<&str>,
        status: Option<&str>,
    ) -> Result<Vec<AdminJobItem>, sqlx::Error> {
        let mut results = Vec::new();

        if job_type.is_none() || job_type == Some("research") {
            let r_jobs = sqlx::query!(
                r#"
                SELECT rj.id, rj.report_id, rp.title as report_title, u.email as user_email,
                       rj.status, rj.attempts, rj.max_attempts, rj.created_at,
                       rj.started_at, rj.finished_at, rj.error_code, rj.error_message
                FROM research_jobs rj
                JOIN report_projects rp ON rp.id = rj.report_id
                JOIN users u ON u.id = rp.user_id
                WHERE ($1::text IS NULL OR rj.status = $1)
                ORDER BY rj.created_at DESC
                LIMIT $2
                "#,
                status,
                limit
            )
            .fetch_all(pool)
            .await?;

            for r in r_jobs {
                results.push(AdminJobItem {
                    job_id: r.id,
                    report_id: r.report_id,
                    report_title: Some(r.report_title),
                    user_email: Some(r.user_email),
                    job_type: "research".to_string(),
                    status: r.status,
                    attempts: r.attempts,
                    max_attempts: r.max_attempts,
                    created_at: r.created_at,
                    started_at: r.started_at,
                    finished_at: r.finished_at,
                    error_code: r.error_code,
                    error_message: r.error_message,
                });
            }
        }

        if job_type.is_none() || job_type == Some("generation") {
            let g_jobs = sqlx::query!(
                r#"
                SELECT gj.id, gj.report_id, rp.title as report_title, u.email as user_email,
                       gj.status, gj.attempts, gj.max_attempts, gj.created_at,
                       gj.started_at, gj.finished_at, gj.error_code, gj.error_message
                FROM generation_jobs gj
                JOIN report_projects rp ON rp.id = gj.report_id
                JOIN users u ON u.id = rp.user_id
                WHERE ($1::text IS NULL OR gj.status = $1)
                ORDER BY gj.created_at DESC
                LIMIT $2
                "#,
                status,
                limit
            )
            .fetch_all(pool)
            .await?;

            for g in g_jobs {
                results.push(AdminJobItem {
                    job_id: g.id,
                    report_id: g.report_id,
                    report_title: Some(g.report_title),
                    user_email: Some(g.user_email),
                    job_type: "generation".to_string(),
                    status: g.status,
                    attempts: g.attempts,
                    max_attempts: g.max_attempts,
                    created_at: g.created_at,
                    started_at: g.started_at,
                    finished_at: g.finished_at,
                    error_code: g.error_code,
                    error_message: g.error_message,
                });
            }
        }

        // Sort combined list descending by created_at
        results.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        let start = offset as usize;
        let end = (offset + limit) as usize;
        if start >= results.len() {
            return Ok(Vec::new());
        }
        let slice = results[start..std::cmp::min(end, results.len())].to_vec();
        Ok(slice)
    }

    pub async fn retry_job(
        pool: &Pool<Postgres>,
        job_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let r_res = sqlx::query!(
            r#"
            UPDATE research_jobs
            SET status = 'pending', attempts = 0, error_code = NULL, error_message = NULL, started_at = NULL, finished_at = NULL
            WHERE id = $1
            "#,
            job_id
        )
        .execute(pool)
        .await?;

        if r_res.rows_affected() > 0 {
            return Ok(true);
        }

        let g_res = sqlx::query!(
            r#"
            UPDATE generation_jobs
            SET status = 'pending', attempts = 0, error_code = NULL, error_message = NULL, started_at = NULL, finished_at = NULL
            WHERE id = $1
            "#,
            job_id
        )
        .execute(pool)
        .await?;

        Ok(g_res.rows_affected() > 0)
    }

    pub async fn cancel_job(
        pool: &Pool<Postgres>,
        job_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let r_res = sqlx::query!(
            r#"
            UPDATE research_jobs
            SET status = 'cancelled', finished_at = now()
            WHERE id = $1 AND status IN ('pending', 'running')
            "#,
            job_id
        )
        .execute(pool)
        .await?;

        if r_res.rows_affected() > 0 {
            return Ok(true);
        }

        let g_res = sqlx::query!(
            r#"
            UPDATE generation_jobs
            SET status = 'cancelled', finished_at = now()
            WHERE id = $1 AND status IN ('pending', 'running')
            "#,
            job_id
        )
        .execute(pool)
        .await?;

        Ok(g_res.rows_affected() > 0)
    }
}

