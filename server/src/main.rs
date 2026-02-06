mod extractors;
mod models;
mod railway;
mod routes;
mod state;

use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use state::{AppState, OAuthConfig};

pub struct ServiceConfig {
    pub server_host: String,
    pub oauth: OAuthConfig,
}

impl ServiceConfig {
    pub fn from_env() -> Self {
        Self {
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0:3001".to_string()),
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
    let config = ServiceConfig::from_env();
    let server_host = config.server_host.clone();
    let state = Arc::new(AppState::new(config));

    let app = Router::new()
        .route("/projects", get(routes::projects::list))
        .route("/services/{project_id}", get(routes::services::list))
        .route("/auth/login", get(routes::auth::login))
        .route("/auth/callback", get(routes::auth::callback))
        .route("/auth/me", get(routes::auth::me))
        .route("/auth/logout", post(routes::auth::logout))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_host).await.unwrap();
    println!("Server running on http://{server_host}");
    axum::serve(listener, app).await.unwrap();
}
