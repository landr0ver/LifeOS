mod auth;
mod config;
mod db;
mod error;
mod models;
mod routes;
mod services;

use axum::middleware;
use axum::Router;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
    pub auth_password: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env();
    let pool = db::create_pool(&config.database_url).await;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Migrations applied successfully");

    let state = AppState {
        auth_password: config.auth_password.clone(),
        pool,
        config: config.clone(),
    };

    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/api/auth/login", axum::routing::post(auth::login));

    // Protected routes (auth required)
    let protected_routes = Router::new()
        .route("/api/auth/logout", axum::routing::post(auth::logout))
        .merge(routes::kanban::router())
        .merge(routes::habits::router())
        .merge(routes::notes::router())
        .merge(routes::calendar::router())
        .merge(routes::github::router())
        .merge(routes::ai::router())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::auth_middleware,
        ));

    let app = Router::new()
        .merge(protected_routes)
        .merge(public_routes)
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Starting LifeOS backend on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
