use axum::extract::Path;
use axum::Json;

use crate::error::AppError;
use crate::extractors::UserSession;
use crate::railway::logs::{self, LogEntry};

pub async fn build_logs(
    UserSession(session): UserSession,
    Path((_project_id, _service_id, deployment_id)): Path<(String, String, String)>,
) -> Result<Json<Vec<LogEntry>>, AppError> {
    let logs = logs::build_logs(&session.railway_auth.access_token, &deployment_id, None).await?;
    Ok(Json(logs))
}

pub async fn deployment_logs(
    UserSession(session): UserSession,
    Path((_project_id, _service_id, deployment_id)): Path<(String, String, String)>,
) -> Result<Json<Vec<LogEntry>>, AppError> {
    let logs = logs::deployment_logs(&session.railway_auth.access_token, &deployment_id, None).await?;
    Ok(Json(logs))
}
