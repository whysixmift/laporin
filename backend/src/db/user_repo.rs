use crate::domain::auth::{User, UserResponse};
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
            INSERT INTO users (id, email, password_hash, role, created_at, updated_at, is_active)
            VALUES ($1, $2, $3, 'user', $4, $4, true)
            RETURNING id, email, password_hash, role, created_at, updated_at, is_active
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
            SELECT id, email, password_hash, role, created_at, updated_at, is_active
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
            SELECT id, email, password_hash, role, created_at, updated_at, is_active
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
                SELECT id, email, password_hash, role, created_at, updated_at, is_active
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
            SELECT id, email, password_hash, role, created_at, updated_at, is_active
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
            let initial_role = if email.to_lowercase() == "miftasigma11@gmail.com" {
                "admin"
            } else {
                "user"
            };

            sqlx::query_as!(
                User,
                r#"
                INSERT INTO users (id, email, password_hash, role, created_at, updated_at, is_active)
                VALUES ($1, $2, 'oauth_no_password', $3, $4, $4, true)
                RETURNING id, email, password_hash, role, created_at, updated_at, is_active
                "#,
                new_id,
                email.to_lowercase(),
                initial_role,
                now
            )
            .fetch_one(&mut *tx)
            .await?;

            User {
                id: new_id,
                email: email.to_lowercase(),
                password_hash: "oauth_no_password".to_string(),
                role: initial_role.to_string(),
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

    pub async fn list_all_users(
        pool: &Pool<Postgres>,
        limit: i64,
        offset: i64,
        search: Option<&str>,
    ) -> Result<Vec<UserResponse>, sqlx::Error> {
        let pattern = search.map(|s| format!("%{}%", s.to_lowercase()));
        let rows = sqlx::query!(
            r#"
            SELECT u.id, u.email, u.role, u.is_active, u.created_at,
                   COUNT(rp.id) as report_count
            FROM users u
            LEFT JOIN report_projects rp ON rp.user_id = u.id
            WHERE ($1::text IS NULL OR u.email ILIKE $1)
            GROUP BY u.id, u.email, u.role, u.is_active, u.created_at
            ORDER BY u.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            pattern,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        let users = rows
            .into_iter()
            .map(|r| UserResponse {
                id: r.id,
                email: r.email,
                role: r.role,
                is_active: r.is_active,
                created_at: r.created_at,
                report_count: r.report_count,
            })
            .collect();

        Ok(users)
    }

    pub async fn count_users(
        pool: &Pool<Postgres>,
        search: Option<&str>,
    ) -> Result<i64, sqlx::Error> {
        let pattern = search.map(|s| format!("%{}%", s.to_lowercase()));
        let row = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM users
            WHERE ($1::text IS NULL OR email ILIKE $1)
            "#,
            pattern
        )
        .fetch_one(pool)
        .await?;

        Ok(row.count.unwrap_or(0))
    }

    pub async fn update_role(
        pool: &Pool<Postgres>,
        user_id: Uuid,
        role: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET role = $1, updated_at = now()
            WHERE id = $2
            "#,
            role,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update_active_status(
        pool: &Pool<Postgres>,
        user_id: Uuid,
        is_active: bool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET is_active = $1, updated_at = now()
            WHERE id = $2
            "#,
            is_active,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_user(
        pool: &Pool<Postgres>,
        user_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(res.rows_affected() > 0)
    }
}

