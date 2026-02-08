use axum::Json;

use crate::error::AppError;
use crate::extractors::UserSession;
use crate::railway::projects::Project;

pub async fn list(
    UserSession(session): UserSession,
) -> Result<Json<Vec<Project>>, AppError> {
    let projects = Project::list(session.railway_auth.access_token).await?;
    Ok(Json(projects))
}
