mod models;
mod routes;
mod state;

use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post, put, delete};
use state::AppState;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/containers", get(routes::containers::list))
        .route("/containers", post(routes::containers::create))
        .route("/containers/{id}", get(routes::containers::get))
        .route("/containers/{id}", put(routes::containers::update))
        .route("/containers/{id}", delete(routes::containers::delete))
        .route("/ws", get(routes::ws::handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Server running on http://0.0.0.0:3001");
    axum::serve(listener, app).await.unwrap();
}
