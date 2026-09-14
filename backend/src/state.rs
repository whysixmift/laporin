use crate::config::Config;
use crate::db::DbPool;
use crate::services::{
    CaptchaService, CrawlerService, EmailService, LlmService, OAuthService, PaymentService,
};
use crate::storage::Storage;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Clone)]
pub struct RateLimiter {
    // Key -> (count, window_start)
    entries: Arc<Mutex<HashMap<String, (u32, Instant)>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn check_and_increment(
        &self,
        key: &str,
        max_requests: u32,
        window_duration: std::time::Duration,
    ) -> (bool, u32, u64) {
        let mut map = self.entries.lock().unwrap();
        let now = Instant::now();

        let entry = map.entry(key.to_string()).or_insert((0, now));
        if now.duration_since(entry.1) >= window_duration {
            entry.0 = 0;
            entry.1 = now;
        }

        entry.0 += 1;
        let remaining = if max_requests >= entry.0 {
            max_requests - entry.0
        } else {
            0
        };

        let elapsed = now.duration_since(entry.1);
        let reset_secs = if window_duration > elapsed {
            (window_duration - elapsed).as_secs() + 1
        } else {
            1
        };

        let allowed = entry.0 <= max_requests;
        (allowed, remaining, reset_secs)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub config: Config,
    pub storage: Storage,
    pub captcha: CaptchaService,
    pub oauth: OAuthService,
    pub crawler: CrawlerService,
    pub llm: LlmService,
    pub payment: PaymentService,
    pub email: EmailService,
    pub rate_limiter: RateLimiter,
}

impl AppState {
    pub fn new(db: DbPool, config: Config, storage: Storage) -> Self {
        Self {
            captcha: CaptchaService::new(config.clone()),
            oauth: OAuthService::new(config.clone()),
            crawler: CrawlerService::new(),
            llm: LlmService::new(config.clone()),
            payment: PaymentService::new(config.clone()),
            email: EmailService::new(&config),
            rate_limiter: RateLimiter::new(),
            db,
            config,
            storage,
        }
    }
}
