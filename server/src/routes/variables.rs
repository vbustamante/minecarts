use std::collections::HashMap;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::error::AppError;
use crate::extractors::UserSession;
use crate::railway::variables;

pub async fn list(
    UserSession(session): UserSession,
    Path((project_id, service_id)): Path<(String, String)>,
) -> Result<Json<HashMap<String, String>>, AppError> {
    let vars = variables::list(&session.railway_auth.access_token, &project_id, &service_id).await?;
    Ok(Json(vars))
}

#[derive(Deserialize)]
pub struct UpsertVariableRequest {
    name: String,
    value: String,
}

pub async fn upsert(
    UserSession(session): UserSession,
    Path((project_id, service_id)): Path<(String, String)>,
    Json(req): Json<UpsertVariableRequest>,
) -> Result<StatusCode, AppError> {
    variables::upsert(
        &session.railway_auth.access_token,
        &project_id,
        &service_id,
        &req.name,
        &req.value,
    )
    .await?;
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct DeleteVariableRequest {
    name: String,
}

pub async fn delete(
    UserSession(session): UserSession,
    Path((project_id, service_id)): Path<(String, String)>,
    Json(req): Json<DeleteVariableRequest>,
) -> Result<StatusCode, AppError> {
    variables::delete(
        &session.railway_auth.access_token,
        &project_id,
        &service_id,
        &req.name,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT)
}
