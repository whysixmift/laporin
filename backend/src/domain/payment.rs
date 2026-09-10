use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCreate {
    pub report_id: Uuid,
    pub return_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub payment_id: Uuid,
    pub payment_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub payment_id: Uuid,
    pub report_id: Uuid,
    pub status: String,
    pub amount: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MayarWebhook {
    pub transaction_id: String,
    pub status: String,
    pub amount: i32,
    pub signature: String,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}
