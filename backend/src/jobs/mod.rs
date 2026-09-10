use sqlx::{Pool, Postgres};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub enum JobType {
    Research,
    Generation,
}

pub struct JobQueue {
    pool: Pool<Postgres>,
}

impl JobQueue {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn enqueue_research_job(&self, report_id: Uuid) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO research_jobs (id, report_id, status) VALUES ($1, $2, 'pending')",
            id,
            report_id
        )
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn enqueue_generation_job(&self, report_id: Uuid) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO generation_jobs (id, report_id, status) VALUES ($1, $2, 'pending')",
            id,
            report_id
        )
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn claim_research_job(&self, worker_id: &str) -> Result<Option<Uuid>, sqlx::Error> {
        let row = sqlx::query!(
            "UPDATE research_jobs 
             SET status = 'running', started_at = now() 
             WHERE id = (
                 SELECT id FROM research_jobs 
                 WHERE status = 'pending' 
                 ORDER BY created_at ASC 
                 FOR UPDATE SKIP LOCKED 
                 LIMIT 1
             ) 
             RETURNING id"
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| r.id))
    }

    pub async fn claim_generation_job(&self, worker_id: &str) -> Result<Option<Uuid>, sqlx::Error> {
        let row = sqlx::query!(
            "UPDATE generation_jobs 
             SET status = 'running', started_at = now() 
             WHERE id = (
                 SELECT id FROM generation_jobs 
                 WHERE status = 'pending' 
                 ORDER BY created_at ASC 
                 FOR UPDATE SKIP LOCKED 
                 LIMIT 1
             ) 
             RETURNING id"
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| r.id))
    }

    pub async fn update_job_status(
        &self, 
        job_id: Uuid, 
        status: &str, 
        error_code: Option<&str>, 
        error_message: Option<&str>
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE research_jobs SET status = $1, finished_at = now(), error_code = $2, error_message = $3 WHERE id = $4",
            status,
            error_code,
            error_message,
            job_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}