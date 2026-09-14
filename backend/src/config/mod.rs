use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub session_secret: String,
    pub session_expiry_hours: i64,
    pub session_cookie_name: String,
    pub captcha_provider: String,
    pub hcaptcha_secret_key: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub l9router_api_key: String,
    pub l9router_api_url: String,
    pub l9router_default_model: String,
    pub mayar_api_key: String,
    pub mayar_webhook_secret: String,
    pub mayar_api_url: String,
    pub storage_root_dir: PathBuf,
    pub search_provider: String,
    pub search_api_key: String,
    pub search_api_url: String,
    pub job_research_concurrency: usize,
    pub job_generation_concurrency: usize,
    pub auth_rate_limit_per_min: u32,
    pub otp_rate_limit_per_hour: u32,
    pub research_start_rate_limit_per_hour: u32,
    pub gen_start_rate_limit_per_hour: u32,
    pub api_rate_limit_per_min: u32,
    pub public_rate_limit_per_min: u32,
    pub environment: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres@localhost:5432/laporin".to_string()),
            session_secret: env::var("SESSION_SECRET").unwrap_or_else(|_| {
                "dev_session_secret_change_me_in_production_32bytes_min".to_string()
            }),
            session_expiry_hours: env::var("SESSION_EXPIRY_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
            session_cookie_name: env::var("SESSION_COOKIE_NAME")
                .unwrap_or_else(|_| "session_id".to_string()),
            captcha_provider: env::var("CAPTCHA_PROVIDER")
                .unwrap_or_else(|_| "hcaptcha".to_string()),
            hcaptcha_secret_key: env::var("HCAPTCHA_SECRET_KEY")
                .unwrap_or_else(|_| "dev_hcaptcha_secret".to_string()),
            google_client_id: env::var("GOOGLE_CLIENT_ID")
                .unwrap_or_else(|_| "dev_google_client_id".to_string()),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET")
                .unwrap_or_else(|_| "dev_google_client_secret".to_string()),
            google_redirect_uri: env::var("GOOGLE_REDIRECT_URI").unwrap_or_else(|_| {
                "https://app.laporin.example.com/auth/google/callback".to_string()
            }),
            l9router_api_key: env::var("L9ROUTER_API_KEY")
                .unwrap_or_else(|_| "dev_l9router_key".to_string()),
            l9router_api_url: env::var("L9ROUTER_API_URL")
                .unwrap_or_else(|_| "https://api.9router.ai/v1".to_string()),
            l9router_default_model: env::var("L9ROUTER_DEFAULT_MODEL")
                .unwrap_or_else(|_| "meta-llama/llama-3.1-8b-instruct".to_string()),
            mayar_api_key: env::var("MAYAR_API_KEY")
                .unwrap_or_else(|_| "dev_mayar_key".to_string()),
            mayar_webhook_secret: env::var("MAYAR_WEBHOOK_SECRET")
                .unwrap_or_else(|_| "dev_mayar_webhook_secret".to_string()),
            mayar_api_url: env::var("MAYAR_API_URL")
                .unwrap_or_else(|_| "https://api.mayar.id/v1".to_string()),
            storage_root_dir: PathBuf::from(
                env::var("STORAGE_ROOT_DIR")
                    .unwrap_or_else(|_| "/home/avrjulian/laporin/storage".to_string()),
            ),
            search_provider: env::var("SEARCH_PROVIDER")
                .unwrap_or_else(|_| "duckduckgo_html".to_string()),
            search_api_key: env::var("SEARCH_API_KEY")
                .unwrap_or_else(|_| "".to_string()),
            search_api_url: env::var("SEARCH_API_URL")
                .unwrap_or_else(|_| "https://html.duckduckgo.com/html/".to_string()),
            job_research_concurrency: env::var("JOB_RESEARCH_CONCURRENCY")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .unwrap_or(2),
            job_generation_concurrency: env::var("JOB_GENERATION_CONCURRENCY")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .unwrap_or(1),
            auth_rate_limit_per_min: env::var("AUTH_RATE_LIMIT_PER_MIN")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            otp_rate_limit_per_hour: env::var("OTP_RATE_LIMIT_PER_HOUR")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            research_start_rate_limit_per_hour: env::var("RESEARCH_START_RATE_LIMIT_PER_HOUR")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            gen_start_rate_limit_per_hour: env::var("GEN_START_RATE_LIMIT_PER_HOUR")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            api_rate_limit_per_min: env::var("API_RATE_LIMIT_PER_MIN")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .unwrap_or(60),
            public_rate_limit_per_min: env::var("PUBLIC_RATE_LIMIT_PER_MIN")
                .unwrap_or_else(|_| "120".to_string())
                .parse()
                .unwrap_or(120),
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
        }
    }
}
