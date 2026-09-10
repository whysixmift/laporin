use axum::Router;
use laporin_backend::api::create_router;
use laporin_backend::config::Config;
use laporin_backend::db::{create_pool, run_migrations};
use laporin_backend::state::AppState;
use laporin_backend::storage::{Storage, StorageConfig};
use std::path::PathBuf;

pub async fn setup_test_app() -> (Router, AppState) {
    dotenvy::dotenv().ok();
    let mut config = Config::from_env();
    config.environment = "test".to_string();
    config.captcha_provider = "mock".to_string();

    let pool = create_pool(&config.database_url, 5)
        .await
        .expect("Failed to connect to test db");

    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    let temp_storage_dir = PathBuf::from(format!(
        "/tmp/laporin_test_storage_{}",
        uuid::Uuid::new_v4()
    ));
    let storage_cfg = StorageConfig::new(&temp_storage_dir);
    let storage = Storage::new(storage_cfg);
    storage
        .ensure_dirs()
        .expect("Failed to ensure test storage dirs");

    let state = AppState::new(pool, config, storage);
    let router = create_router(state.clone());

    (router, state)
}
