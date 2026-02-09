mod error;
mod extractors;
mod models;
mod railway;
mod session_store;
mod routes;
mod state;

use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post, put};
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use state::{AppState, OAuthConfig};

#[derive(Debug)]
pub struct ServiceConfig {
    pub server_host: String,
    pub redis_url: String,
    pub oauth: OAuthConfig,
    pub frontend_url: String,
}

impl ServiceConfig {
    pub fn from_env() -> Self {
        Self {
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0:3001".to_string()),
            redis_url: std::env::var("REDIS_URL")
                .expect("REDIS_URL must be set"),
            oauth: OAuthConfig {
                client_id: std::env::var("RAILWAY_CLIENT_ID")
                    .expect("RAILWAY_CLIENT_ID must be set"),
                client_secret: std::env::var("RAILWAY_CLIENT_SECRET")
                    .expect("RAILWAY_CLIENT_SECRET must be set"),
                redirect_uri: std::env::var("RAILWAY_REDIRECT_URI")
                    .unwrap_or_else(|_| "http://localhost:5173/api/auth/callback".to_string()),
            },
            frontend_url: std::env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
        }
    }
}

#[tokio::main]
async fn main() {
    println!("hm");
    tracing_subscriber::fmt()
        .with_target(false)
        .init();
    tracing::info!("Booted up");

    let _ = dotenvy::from_filename(".env");
    let config = ServiceConfig::from_env();
    let server_host = config.server_host.clone();

    tracing::info!("Connecting to Redis");
    let sessions = session_store::SessionStore::connect(&config.redis_url)
        .await
        .expect("Failed to connect to Redis");

    tracing::info!("Instantiating state");
    let state = Arc::new(AppState::new(config, sessions));

    let variables_router = Router::new()
        .route("/", get(routes::variables::list).put(routes::variables::upsert).delete(routes::variables::delete));

    let services_router = Router::new()
        .route("/", get(routes::services::list).post(routes::services::create))
        .route("/{service_id}", put(routes::services::update).delete(routes::services::delete))
        .nest("/{service_id}/variables", variables_router);

    let cors = CorsLayer::new()
        .allow_origin(
            state.frontend_url.parse::<axum::http::HeaderValue>()
                .expect("FRONTEND_URL must be a valid header value"),
        )
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
        ])
        .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION])
        .allow_credentials(true);

    let app = Router::new()
        .route("/", get(|| async { "hello from minecarts server" }))
        .route("/projects", get(routes::projects::list))
        .nest("/projects/{project_id}/services", services_router)
        .route("/auth/login", get(routes::auth::login))
        .route("/auth/callback", get(routes::auth::callback))
        .route("/auth/me", get(routes::auth::me))
        .route("/auth/logout", post(routes::auth::logout))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_host).await.unwrap();
    tracing::info!("Server running on http://{server_host}");
    axum::serve(listener, app).await.unwrap();
}
