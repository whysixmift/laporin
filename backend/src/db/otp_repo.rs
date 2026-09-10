use chrono::{DateTime, Duration, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct EmailOtp {
    pub id: Uuid,
    pub user_id: Uuid,
    pub otp_hash: String,
    pub expires_at: DateTime<Utc>,
    pub attempts: i32,
    pub created_at: DateTime<Utc>,
}

pub struct OtpRepo;

impl OtpRepo {
    pub async fn count_otps_in_last_hour(
        pool: &Pool<Postgres>,
        user_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        let one_hour_ago = Utc::now() - Duration::hours(1);
        let row = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM email_otps
            WHERE user_id = $1 AND created_at > $2
            "#,
            user_id,
            one_hour_ago
        )
        .fetch_one(pool)
        .await?;

        Ok(row.count.unwrap_or(0))
    }

    pub async fn create_otp(
        pool: &Pool<Postgres>,
        user_id: Uuid,
        otp_hash: &str,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let expires_at = now + Duration::minutes(10);

        sqlx::query!(
            r#"
            INSERT INTO email_otps (id, user_id, otp_hash, expires_at, attempts, created_at)
            VALUES ($1, $2, $3, $4, 0, $5)
            "#,
            id,
            user_id,
            otp_hash,
            expires_at,
            now
        )
        .execute(pool)
        .await?;

        Ok(id)
    }

    pub async fn find_latest_otp(
        pool: &Pool<Postgres>,
        user_id: Uuid,
    ) -> Result<Option<EmailOtp>, sqlx::Error> {
        let row = sqlx::query_as!(
            EmailOtp,
            r#"
            SELECT id, user_id, otp_hash, expires_at, attempts, created_at
            FROM email_otps
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn increment_attempts(pool: &Pool<Postgres>, id: Uuid) -> Result<i32, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            UPDATE email_otps
            SET attempts = attempts + 1
            WHERE id = $1
            RETURNING attempts
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(row.attempts)
    }

    pub async fn delete_otp(pool: &Pool<Postgres>, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM email_otps
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
