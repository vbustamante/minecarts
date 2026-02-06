use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;

use crate::extractors::UserSession;
use crate::railway::services::RailwayService;

pub async fn list(
    UserSession(session): UserSession,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<RailwayService>>, StatusCode> {
    RailwayService::list(session.railway_auth.access_token, &project_id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::BAD_GATEWAY)
}
