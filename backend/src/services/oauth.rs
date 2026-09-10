use crate::config::Config;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone)]
pub struct OAuthService {
    client: reqwest::Client,
    config: Config,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: String,
    #[serde(default)]
    pub email_verified: Option<bool>,
}

#[derive(Deserialize)]
struct GoogleTokenResponse {
    id_token: Option<String>,
    access_token: Option<String>,
}

impl OAuthService {
    pub fn new(config: Config) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            config,
        }
    }

    pub fn get_authorization_url(&self, state: Option<&str>) -> String {
        let state_val = state.unwrap_or_else(|| "auth_state");
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile&state={}",
            urlencoding::encode(&self.config.google_client_id),
            urlencoding::encode(&self.config.google_redirect_uri),
            urlencoding::encode(state_val)
        )
    }

    pub async fn exchange_code(
        &self,
        code: &str,
    ) -> Result<GoogleUserInfo, Box<dyn std::error::Error + Send + Sync>> {
        if self.config.environment == "development" || self.config.environment == "test" {
            // Support test codes in dev/test
            if code.starts_with("test_code_") {
                let suffix = &code["test_code_".len()..];
                return Ok(GoogleUserInfo {
                    sub: format!("google-sub-{}", suffix),
                    email: format!("{}@gmail.com", suffix),
                    email_verified: Some(true),
                });
            }
        }

        let params = [
            ("code", code),
            ("client_id", &self.config.google_client_id),
            ("client_secret", &self.config.google_client_secret),
            ("redirect_uri", &self.config.google_redirect_uri),
            ("grant_type", "authorization_code"),
        ];

        let token_resp = self
            .client
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await?
            .json::<GoogleTokenResponse>()
            .await?;

        if let Some(id_token) = token_resp.id_token {
            // Verify id_token with Google tokeninfo endpoint
            let user_info = self
                .client
                .get(format!(
                    "https://oauth2.googleapis.com/tokeninfo?id_token={}",
                    id_token
                ))
                .send()
                .await?
                .json::<GoogleUserInfo>()
                .await?;

            return Ok(user_info);
        }

        Err("Failed to obtain ID token from Google".into())
    }
}
