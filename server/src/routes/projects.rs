use axum::http::StatusCode;
use axum::Json;

use crate::extractors::UserSession;
use crate::railway::projects::{Project};

pub async fn list(
    UserSession(session): UserSession,
) -> Result<Json<Vec<Project>>, StatusCode> {

    Project::list(session.railway_auth.access_token)
        .await
        .map(Json)
        .map_err(|_| StatusCode::BAD_GATEWAY)
}
