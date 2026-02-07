use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::extractors::UserSession;
use crate::railway::services::{CreateServiceRequest as RailwayCreateServiceRequest, RailwayService, ServiceError, ServiceWithDeployment, UpdateServiceRequest};

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

pub async fn list(
    UserSession(session): UserSession,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<ServiceWithDeployment>>, (StatusCode, Json<ErrorResponse>)> {
    RailwayService::list(session.railway_auth.access_token, &project_id)
        .await
        .map(Json)
        .map_err(|e| match e {
            ServiceError::NoProductionEnvironment => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ErrorResponse { error: "No production environment found. Create one on the railway console then click refresh.".into() }),
            ),
            ServiceError::Request(_) => (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse { error: "Failed to fetch services from Railway".into() }),
            ),
        })
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateServiceRequest {
    name: String,
    icon: Option<String>,
}

pub async fn create(
    UserSession(session): UserSession,
    Path(project_id): Path<String>,
    Json(req): Json<CreateServiceRequest>,
) -> Result<(StatusCode, Json<RailwayService>), StatusCode> {
    let req = RailwayCreateServiceRequest {
        project_id,
        image: "hello-world".to_string(),
        name: req.name,
        icon: req.icon,
    };
    RailwayService::create(session.railway_auth.access_token, req)
        .await
        .map(|s| (StatusCode::CREATED, Json(s)))
        .map_err(|_| StatusCode::BAD_GATEWAY)
}

pub async fn update(
    UserSession(session): UserSession,
    Path((_project_id, id)): Path<(String, String)>,
    Json(req): Json<UpdateServiceRequest>,
) -> Result<Json<RailwayService>, StatusCode> {
    RailwayService::update(session.railway_auth.access_token, &id, req)
        .await
        .map(Json)
        .map_err(|_| StatusCode::BAD_GATEWAY)
}

pub async fn delete(
    UserSession(session): UserSession,
    Path((_project_id, id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    RailwayService::delete(session.railway_auth.access_token, &id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::BAD_GATEWAY)
}
