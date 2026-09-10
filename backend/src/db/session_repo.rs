use crate::domain::auth::Session;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct SessionRepo;

impl SessionRepo {
    pub async fn create_session(
        pool: &Pool<Postgres>,
        user_id: Uuid,
        token_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<Session, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let row = sqlx::query_as!(
            Session,
            r#"
            INSERT INTO sessions (id, user_id, token_hash, expires_at, created_at, revoked)
            VALUES ($1, $2, $3, $4, $5, false)
            RETURNING id, user_id, token_hash, expires_at, created_at, revoked
            "#,
            id,
            user_id,
            token_hash,
            expires_at,
            now
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    pub async fn find_by_token_hash(
        pool: &Pool<Postgres>,
        token_hash: &str,
    ) -> Result<Option<Session>, sqlx::Error> {
        let row = sqlx::query_as!(
            Session,
            r#"
            SELECT id, user_id, token_hash, expires_at, created_at, revoked
            FROM sessions
            WHERE token_hash = $1
            "#,
            token_hash
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn revoke_session(
        pool: &Pool<Postgres>,
        token_hash: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE sessions
            SET revoked = true
            WHERE token_hash = $1
            "#,
            token_hash
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn purge_expired(pool: &Pool<Postgres>) -> Result<u64, sqlx::Error> {
        let res = sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE expires_at < now() OR revoked = true
            "#
        )
        .execute(pool)
        .await?;

        Ok(res.rows_affected())
    }
}
