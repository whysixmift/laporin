use crate::domain::payment::Payment;
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct PaymentRepo;

pub struct PaymentRecord {
    pub id: Uuid,
    pub report_id: Uuid,
    pub status: String,
    pub amount_cents: i32,
    pub provider_tx_id: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct TargetPayment {
    id: Uuid,
    report_id: Uuid,
    status: String,
}

impl PaymentRepo {
    pub async fn create_payment(
        pool: &Pool<Postgres>,
        report_id: Uuid,
        amount_cents: i32,
    ) -> Result<Payment, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO payments (id, report_id, status, amount_cents, created_at, updated_at)
            VALUES ($1, $2, 'pending', $3, $4, $4)
            "#,
            id,
            report_id,
            amount_cents,
            now
        )
        .execute(pool)
        .await?;

        // Update report status to payment_pending
        sqlx::query!(
            r#"
            UPDATE report_projects
            SET status = 'payment_pending', updated_at = $2
            WHERE id = $1
            "#,
            report_id,
            now
        )
        .execute(pool)
        .await?;

        Ok(Payment {
            payment_id: id,
            report_id,
            status: "pending".into(),
            amount: amount_cents,
            created_at: now,
        })
    }

    pub async fn get_payment_by_id(
        pool: &Pool<Postgres>,
        payment_id: Uuid,
    ) -> Result<Option<(Payment, Uuid)>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT p.id, p.report_id, p.status, p.amount_cents, p.created_at, rp.user_id
            FROM payments p
            JOIN report_projects rp ON p.report_id = rp.id
            WHERE p.id = $1
            "#,
            payment_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| {
            (
                Payment {
                    payment_id: r.id,
                    report_id: r.report_id,
                    status: r.status,
                    amount: r.amount_cents,
                    created_at: r.created_at,
                },
                r.user_id,
            )
        }))
    }

    pub async fn find_by_provider_tx_id(
        pool: &Pool<Postgres>,
        tx_id: &str,
    ) -> Result<Option<PaymentRecord>, sqlx::Error> {
        let row = sqlx::query_as!(
            PaymentRecord,
            r#"
            SELECT id, report_id, status, amount_cents, provider_tx_id, created_at, updated_at
            FROM payments
            WHERE provider_tx_id = $1
            "#,
            tx_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn process_webhook_success(
        pool: &Pool<Postgres>,
        transaction_id: &str,
        payment_id_hint: Option<Uuid>,
    ) -> Result<bool, sqlx::Error> {
        let mut tx = pool.begin().await?;

        // 1. Check if payment with this provider_tx_id already succeeded
        let existing = sqlx::query!(
            r#"
            SELECT id, report_id, status
            FROM payments
            WHERE provider_tx_id = $1
            FOR UPDATE
            "#,
            transaction_id
        )
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(p) = existing {
            if p.status == "succeeded" {
                // Idempotent: already succeeded
                tx.commit().await?;
                return Ok(true);
            }
        }

        // 2. Find pending payment (by provider_tx_id or payment_id_hint or latest pending payment)
        let target_payment: Option<TargetPayment> = if let Some(pid) = payment_id_hint {
            sqlx::query_as!(
                TargetPayment,
                r#"
                SELECT id, report_id, status
                FROM payments
                WHERE id = $1
                FOR UPDATE
                "#,
                pid
            )
            .fetch_optional(&mut *tx)
            .await?
        } else {
            sqlx::query_as!(
                TargetPayment,
                r#"
                SELECT id, report_id, status
                FROM payments
                WHERE status = 'pending'
                ORDER BY created_at DESC
                LIMIT 1
                FOR UPDATE
                "#
            )
            .fetch_optional(&mut *tx)
            .await?
        };

        if let Some(p) = target_payment {
            let now = Utc::now();
            // Update payment record
            sqlx::query!(
                r#"
                UPDATE payments
                SET status = 'succeeded', provider_tx_id = $2, updated_at = $3
                WHERE id = $1
                "#,
                p.id,
                transaction_id,
                now
            )
            .execute(&mut *tx)
            .await?;

            // Update report project to unlocked
            sqlx::query!(
                r#"
                UPDATE report_projects
                SET status = 'unlocked', updated_at = $2
                WHERE id = $1
                "#,
                p.report_id,
                now
            )
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;
            Ok(true)
        } else {
            tx.rollback().await?;
            Ok(false)
        }
    }

    pub async fn process_webhook_failure(
        pool: &Pool<Postgres>,
        transaction_id: &str,
        payment_id_hint: Option<Uuid>,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        if let Some(pid) = payment_id_hint {
            sqlx::query!(
                r#"
                UPDATE payments
                SET status = 'failed', provider_tx_id = $2, updated_at = now()
                WHERE id = $1
                "#,
                pid,
                transaction_id
            )
            .execute(&mut *tx)
            .await?;
        } else {
            sqlx::query!(
                r#"
                UPDATE payments
                SET status = 'failed', updated_at = now()
                WHERE provider_tx_id = $1
                "#,
                transaction_id
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
