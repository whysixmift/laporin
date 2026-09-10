use crate::domain::job::JobInfo;
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
}
