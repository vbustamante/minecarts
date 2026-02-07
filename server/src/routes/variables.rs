use std::collections::HashMap;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::extractors::UserSession;
use crate::railway::variables::{self, VariableError};

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

fn map_error(e: VariableError) -> (StatusCode, Json<ErrorResponse>) {
    match e {
        VariableError::NoProductionEnvironment => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse {
                error: "No production environment found. Create one on the Railway console then try again.".into(),
            }),
        ),
        VariableError::Request(_) => (
            StatusCode::BAD_GATEWAY,
            Json(ErrorResponse {
                error: "Failed to communicate with Railway API".into(),
            }),
        ),
    }
}

pub async fn list(
    UserSession(session): UserSession,
    Path((project_id, service_id)): Path<(String, String)>,
) -> Result<Json<HashMap<String, String>>, (StatusCode, Json<ErrorResponse>)> {
    variables::list(&session.railway_auth.access_token, &project_id, &service_id)
        .await
        .map(Json)
        .map_err(map_error)
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
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    variables::upsert(
        &session.railway_auth.access_token,
        &project_id,
        &service_id,
        &req.name,
        &req.value,
    )
    .await
    .map(|_| StatusCode::OK)
    .map_err(map_error)
}

#[derive(Deserialize)]
pub struct DeleteVariableRequest {
    name: String,
}

pub async fn delete(
    UserSession(session): UserSession,
    Path((project_id, service_id)): Path<(String, String)>,
    Json(req): Json<DeleteVariableRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    variables::delete(
        &session.railway_auth.access_token,
        &project_id,
        &service_id,
        &req.name,
    )
    .await
    .map(|_| StatusCode::NO_CONTENT)
    .map_err(map_error)
}
