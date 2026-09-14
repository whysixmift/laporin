use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnUploadResponse {
    pub id: String,
    pub filename: String,
    pub size: i64,
    pub content_type: String,
    pub url: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnQuotaResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub storage_used: i64,
    pub storage_limit: i64,
    pub quota_tier: String,
}

#[derive(Clone)]
pub struct CdnService {
    client: Client,
    pub api_key: Option<String>,
    pub api_base: String,
}

impl CdnService {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        let api_key = std::env::var("HACKCLUB_CDN_API_KEY")
            .or_else(|_| std::env::var("CDN_API_KEY"))
            .ok()
            .filter(|s| !s.trim().is_empty() && !s.contains("placeholder"));

        let api_base = std::env::var("CDN_API_URL")
            .unwrap_or_else(|_| "https://cdn.hackclub.com/api/v4".to_string());

        Self {
            client,
            api_key,
            api_base,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.api_key.is_some()
    }

    pub async fn get_quota(&self) -> Result<CdnQuotaResponse, String> {
        let key = self
            .api_key
            .as_ref()
            .ok_or_else(|| "CDN API key is not configured".to_string())?;

        let url = format!("{}/me", self.api_base);
        let resp = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", key))
            .send()
            .await
            .map_err(|e| format!("Network error connecting to CDN: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("CDN status {}: {}", status, body));
        }

        resp.json::<CdnQuotaResponse>()
            .await
            .map_err(|e| format!("Failed to parse CDN quota response: {}", e))
    }

    pub async fn upload_bytes(
        &self,
        filename: &str,
        data: Vec<u8>,
        mime_type: Option<&str>,
    ) -> Result<CdnUploadResponse, String> {
        let key = self
            .api_key
            .as_ref()
            .ok_or_else(|| "CDN API key is not configured".to_string())?;

        let url = format!("{}/upload", self.api_base);
        let mut part = Part::bytes(data).file_name(filename.to_string());
        if let Some(mime) = mime_type {
            part = part.mime_str(mime).map_err(|e| e.to_string())?;
        }

        let form = Form::new().part("file", part);

        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Network error uploading to CDN: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("CDN upload error {}: {}", status, body));
        }

        let upload_res = resp
            .json::<CdnUploadResponse>()
            .await
            .map_err(|e| format!("Failed to parse CDN upload response: {}", e))?;

        tracing::info!(
            filename = %filename,
            cdn_url = %upload_res.url,
            "Successfully uploaded file to Hack Club CDN"
        );

        Ok(upload_res)
    }

    pub async fn delete_upload(&self, upload_id: &str) -> Result<bool, String> {
        let key = self
            .api_key
            .as_ref()
            .ok_or_else(|| "CDN API key is not configured".to_string())?;

        let url = format!("{}/upload/{}", self.api_base, upload_id);
        let resp = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", key))
            .send()
            .await
            .map_err(|e| format!("Network error deleting from CDN: {}", e))?;

        Ok(resp.status().is_success())
    }
}
