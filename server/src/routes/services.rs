use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use crate::error::AppError;
use crate::extractors::UserSession;
use crate::railway::services::{CreateServiceRequest as RailwayCreateServiceRequest, RailwayService, ServiceWithDeployment, UpdateServiceRequest};

pub async fn list(
    UserSession(session): UserSession,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<ServiceWithDeployment>>, AppError> {
    let services = RailwayService::list(session.railway_auth.access_token, &project_id).await?;
    Ok(Json(services))
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateServiceRequest {
    name: String,
    image: String,
    icon: Option<String>,
}

pub async fn create(
    UserSession(session): UserSession,
    Path(project_id): Path<String>,
    Json(req): Json<CreateServiceRequest>,
) -> Result<(StatusCode, Json<RailwayService>), AppError> {
    let req = RailwayCreateServiceRequest {
        project_id,
        image: req.image,
        name: req.name,
        icon: req.icon,
    };
    let service = RailwayService::create(session.railway_auth.access_token, req).await?;
    Ok((StatusCode::CREATED, Json(service)))
}

pub async fn update(
    UserSession(session): UserSession,
    Path((_project_id, id)): Path<(String, String)>,
    Json(req): Json<UpdateServiceRequest>,
) -> Result<Json<RailwayService>, AppError> {
    let service = RailwayService::update(session.railway_auth.access_token, &id, req).await?;
    Ok(Json(service))
}

pub async fn delete(
    UserSession(session): UserSession,
    Path((_project_id, id)): Path<(String, String)>,
) -> Result<StatusCode, AppError> {
    RailwayService::delete(session.railway_auth.access_token, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}
