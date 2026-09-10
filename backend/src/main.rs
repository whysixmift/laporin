#![allow(clippy::all)]
#![allow(unused)]

use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::signal;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod auth;
mod config;
mod db;
mod domain;
mod errors;
mod middleware;
mod observability;
mod services;
mod state;
mod storage;
mod workers;

#[tokio::main]
pub async fn main() {
    // Setup tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env();

    // Database connection
    let db_pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            tracing::info!("Connected to database");
            pool
        }
        Err(err) => {
            tracing::error!("Failed to connect to database: {:?}", err);
            std::process::exit(1);
        }
    };

    // Run migrations
    match db::run_migrations(&db_pool).await {
        Ok(_) => tracing::info!("Migrations applied successfully"),
        Err(err) => {
            tracing::error!("Failed to run migrations: {:?}", err);
            std::process::exit(1);
        }
    }

    // Initialize Storage
    let storage_cfg = storage::StorageConfig::new(&config.storage_root_dir);
    let storage = storage::Storage::new(storage_cfg);
    if let Err(e) = storage.ensure_dirs() {
        tracing::error!("Failed to ensure storage directories: {:?}", e);
        std::process::exit(1);
    }

    // Create AppState
    let state = state::AppState::new(db_pool.clone(), config.clone(), storage);

    // Spawn Background Workers
    for i in 0..config.job_research_concurrency {
        let worker_state = state.clone();
        tokio::spawn(async move {
            tracing::info!(worker_index = %i, "Spawning research worker");
            workers::ResearchWorker::run(worker_state).await;
        });
    }

    for i in 0..config.job_generation_concurrency {
        let worker_state = state.clone();
        tokio::spawn(async move {
            tracing::info!(worker_index = %i, "Spawning generation worker");
            workers::GenerationWorker::run(worker_state).await;
        });
    }

    let cleanup_state = state.clone();
    tokio::spawn(async move {
        workers::CleanupWorker::run(cleanup_state).await;
    });

    // Build Router
    let app = api::create_router(state);

    let host_ip: std::net::IpAddr = config.server_host.parse().unwrap_or([0, 0, 0, 0].into());
    let addr = SocketAddr::new(host_ip, config.server_port);
    tracing::info!("Laporin Backend listening on http://{}", addr);

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("Failed to bind to address {}: {:?}", addr, e);
            std::process::exit(1);
        }
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C signal, shutting down gracefully");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM signal, shutting down gracefully");
        },
    }
}
