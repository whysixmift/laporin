use crate::config::Config;
use serde::Deserialize;

#[derive(Clone)]
pub struct CaptchaService {
    client: reqwest::Client,
    config: Config,
}

#[derive(Deserialize)]
struct HCaptchaVerifyResponse {
    success: bool,
}

impl CaptchaService {
    pub fn new(config: Config) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap_or_default(),
            config,
        }
    }

    pub async fn verify(&self, token: &str) -> bool {
        if token.is_empty() {
            return false;
        }

        if self.config.captcha_provider == "mock"
            || self.config.environment == "development"
            || self.config.environment == "test"
        {
            if token == "invalid_token" || token == "test_captcha_invalid" {
                return false;
            }
            return true;
        }

        // Real hCaptcha validation
        let params = [
            ("response", token),
            ("secret", &self.config.hcaptcha_secret_key),
        ];

        match self
            .client
            .post("https://api.hcaptcha.com/siteverify")
            .form(&params)
            .send()
            .await
        {
            Ok(resp) => {
                if let Ok(body) = resp.json::<HCaptchaVerifyResponse>().await {
                    body.success
                } else {
                    false
                }
            }
            Err(e) => {
                tracing::error!("CAPTCHA verification error: {:?}", e);
                false
            }
        }
    }
}
