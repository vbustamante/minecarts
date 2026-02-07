mod extractors;
mod models;
mod railway;
mod routes;
mod state;

use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post, put};
use state::{AppState, OAuthConfig};

#[derive(Debug)]
pub struct ServiceConfig {
    pub server_host: String,
    pub redis_url: String,
    pub oauth: OAuthConfig,
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
        }
    }
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let config = ServiceConfig::from_env();
    let server_host = config.server_host.clone();

    let redis_client = redis::Client::open(config.redis_url.as_str())
        .expect("Invalid REDIS_URL");
    let redis_conn = redis_client
        .get_connection_manager()
        .await
        .expect("Failed to connect to Redis");

    let state = Arc::new(AppState::new(config, redis_conn));

    let variables_router = Router::new()
        .route("/", get(routes::variables::list).put(routes::variables::upsert).delete(routes::variables::delete));

    let services_router = Router::new()
        .route("/", get(routes::services::list).post(routes::services::create))
        .route("/{service_id}", put(routes::services::update).delete(routes::services::delete))
        .nest("/{service_id}/variables", variables_router);

    let app = Router::new()
        .route("/projects", get(routes::projects::list))
        .nest("/projects/{project_id}/services", services_router)
        .route("/auth/login", get(routes::auth::login))
        .route("/auth/callback", get(routes::auth::callback))
        .route("/auth/me", get(routes::auth::me))
        .route("/auth/logout", post(routes::auth::logout))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_host).await.unwrap();
    println!("Server running on http://{server_host}");
    axum::serve(listener, app).await.unwrap();
}
