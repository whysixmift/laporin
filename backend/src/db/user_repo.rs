use crate::domain::auth::User;
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct UserRepo;

impl UserRepo {
    pub async fn create_user(
        pool: &Pool<Postgres>,
        email: &str,
        password_hash: &str,
    ) -> Result<User, sqlx::Error> {
        let row = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, password_hash, created_at, updated_at, is_active)
            VALUES ($1, $2, $3, $4, $4, true)
            RETURNING id, email, password_hash, created_at, updated_at, is_active
            "#,
            Uuid::new_v4(),
            email.to_lowercase(),
            password_hash,
            Utc::now()
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    pub async fn find_by_email(
        pool: &Pool<Postgres>,
        email: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, created_at, updated_at, is_active
            FROM users
            WHERE email = $1
            "#,
            email.to_lowercase()
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn find_by_id(pool: &Pool<Postgres>, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, created_at, updated_at, is_active
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn find_or_create_oauth_user(
        pool: &Pool<Postgres>,
        provider: &str,
        provider_user_id: &str,
        email: &str,
    ) -> Result<User, sqlx::Error> {
        let mut tx = pool.begin().await?;

        // 1. Check if oauth identity exists
        let oauth_row = sqlx::query!(
            r#"
            SELECT user_id FROM oauth_identities
            WHERE provider = $1 AND provider_user_id = $2
            "#,
            provider,
            provider_user_id
        )
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(identity) = oauth_row {
            let user = sqlx::query_as!(
                User,
                r#"
                SELECT id, email, password_hash, created_at, updated_at, is_active
                FROM users
                WHERE id = $1
                "#,
                identity.user_id
            )
            .fetch_one(&mut *tx)
            .await?;

            tx.commit().await?;
            return Ok(user);
        }

        // 2. Check if user with this email already exists
        let existing_user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, created_at, updated_at, is_active
            FROM users
            WHERE email = $1
            "#,
            email.to_lowercase()
        )
        .fetch_optional(&mut *tx)
        .await?;

        let user = if let Some(u) = existing_user {
            u
        } else {
            // Create user
            let new_id = Uuid::new_v4();
            let now = Utc::now();
            sqlx::query_as!(
                User,
                r#"
                INSERT INTO users (id, email, password_hash, created_at, updated_at, is_active)
                VALUES ($1, $2, 'oauth_no_password', $3, $3, true)
                RETURNING id, email, password_hash, created_at, updated_at, is_active
                "#,
                new_id,
                email.to_lowercase(),
                now
            )
            .fetch_one(&mut *tx)
            .await?;

            User {
                id: new_id,
                email: email.to_lowercase(),
                password_hash: "oauth_no_password".to_string(),
                created_at: now,
                updated_at: now,
                is_active: true,
            }
        };

        // 3. Insert oauth identity
        sqlx::query!(
            r#"
            INSERT INTO oauth_identities (id, user_id, provider, provider_user_id, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (provider, provider_user_id) DO NOTHING
            "#,
            Uuid::new_v4(),
            user.id,
            provider,
            provider_user_id,
            Utc::now()
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(user)
    }
}
