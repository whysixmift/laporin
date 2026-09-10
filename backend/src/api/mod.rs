pub mod routes;

use crate::middleware::{rate_limit_middleware, request_id_middleware};
use crate::state::AppState;
use axum::{
    Router,
    middleware::from_fn,
    middleware::from_fn_with_state,
    routing::{get, patch, post},
};
use tower_http::cors::{Any, CorsLayer};

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let auth_routes = Router::new()
        .route("/register", post(routes::auth::register_handler))
        .route("/login", post(routes::auth::login_handler))
        .route(
            "/google/oauth_url",
            get(routes::auth::google_oauth_url_handler),
        )
        .route(
            "/google/callback",
            post(routes::auth::google_callback_handler),
        )
        .route("/logout", post(routes::auth::logout_handler))
        .route("/verify-otp", post(routes::auth::verify_otp_handler));

    let report_routes = Router::new()
        .route(
            "/",
            post(routes::reports::create_report_handler).get(routes::reports::list_reports_handler),
        )
        .route(
            "/:id",
            get(routes::reports::get_report_handler).patch(routes::reports::update_report_handler),
        )
        .route("/:id/preview", get(routes::reports::get_preview_handler))
        .route(
            "/:id/download",
            get(routes::reports::download_report_handler),
        )
        .route(
            "/:id/research/start",
            post(routes::research::start_research_handler),
        )
        .route(
            "/:id/research/status",
            get(routes::research::get_research_status_handler),
        )
        .route(
            "/:id/generation/start",
            post(routes::generation::start_generation_handler),
        )
        .route(
            "/:id/generation/status",
            get(routes::generation::get_generation_status_handler),
        );

    let payment_routes = Router::new()
        .route("/", post(routes::payments::create_payment_handler))
        .route("/:id", get(routes::payments::get_payment_handler));

    let webhook_routes =
        Router::new().route("/mayar", post(routes::webhooks::mayar_webhook_handler));

    let api_v1 = Router::new()
        .nest("/auth", auth_routes.clone())
        .nest("/reports", report_routes.clone())
        .nest("/payments", payment_routes.clone())
        .nest("/webhook", webhook_routes.clone());

    Router::new()
        .route("/health", get(health_handler))
        // Direct OpenAPI paths
        .nest("/auth", auth_routes)
        .nest("/reports", report_routes)
        .nest("/payments", payment_routes)
        .nest("/webhook", webhook_routes)
        // Nested under /api/v1 as defined in servers info
        .nest("/api/v1", api_v1)
        .layer(from_fn_with_state(state.clone(), rate_limit_middleware))
        .layer(from_fn(request_id_middleware))
        .layer(cors)
        .with_state(state)
}

async fn health_handler() -> &'static str {
    "OK"
}
