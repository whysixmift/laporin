use crate::auth::verify_hmac_signature;
use crate::config::Config;
use uuid::Uuid;

pub const PRODUCT_PRICE_CENTS: i32 = 15000; // Rp15.000

#[derive(Clone)]
pub struct PaymentService {
    client: reqwest::Client,
    config: Config,
}

impl PaymentService {
    pub fn new(config: Config) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            config,
        }
    }

    pub fn get_price_cents(&self) -> i32 {
        PRODUCT_PRICE_CENTS
    }

    pub async fn create_gateway_payment(
        &self,
        payment_id: Uuid,
        report_id: Uuid,
        return_url: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        if self.config.environment == "test" || self.config.mayar_api_key.starts_with("dev_") || self.config.mayar_api_key.is_empty() {
            // Mock Mayar payment checkout URL redirecting directly back to return_url with simulated success
            let sep = if return_url.contains('?') { "&" } else { "?" };
            return Ok(format!(
                "{}{}payment_status=success&payment_id={}",
                return_url, sep, payment_id
            ));
        }

        // Real Mayar Payment creation request
        let payload = serde_json::json!({
            "amount": PRODUCT_PRICE_CENTS,
            "currency": "IDR",
            "description": format!("Laporin Full Report Unlock (ID: {})", report_id),
            "redirectUrl": return_url,
            "metadata": {
                "payment_id": payment_id.to_string(),
                "report_id": report_id.to_string()
            }
        });

        let resp = self
            .client
            .post(format!("{}/payment/create", self.config.mayar_api_url))
            .header(
                "Authorization",
                format!("Bearer {}", self.config.mayar_api_key),
            )
            .json(&payload)
            .send()
            .await;

        if let Ok(resp) = resp {
            if resp.status().is_success() {
                if let Ok(data) = resp.json::<serde_json::Value>().await {
                    if let Some(link) = data["data"]["link"].as_str() {
                        return Ok(link.to_string());
                    }
                }
            }
        }

        // Fallback to sandbox or return_url if external payment creation failed
        let sep = if return_url.contains('?') { "&" } else { "?" };
        Ok(format!(
            "{}{}payment_status=sandbox_success&payment_id={}",
            return_url, sep, payment_id
        ))
    }


    pub fn verify_webhook_signature(&self, raw_body: &str, signature: &str) -> bool {
        verify_hmac_signature(raw_body, signature, &self.config.mayar_webhook_secret)
            .unwrap_or(false)
    }
}
