use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

use crate::railway::services::ServiceError;
use crate::railway::variables::VariableError;

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Railway API error: {0}")]
    Railway(#[from] reqwest::Error),

    #[error("No production environment found. Create one on the Railway console then try again.")]
    NoProductionEnvironment,

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("{0}")]
    BadRequest(String),
}

impl From<ServiceError> for AppError {
    fn from(e: ServiceError) -> Self {
        match e {
            ServiceError::Request(e) => AppError::Railway(e),
            ServiceError::NoProductionEnvironment => AppError::NoProductionEnvironment,
        }
    }
}

impl From<VariableError> for AppError {
    fn from(e: VariableError) -> Self {
        match e {
            VariableError::Request(e) => AppError::Railway(e),
            VariableError::NoProductionEnvironment => AppError::NoProductionEnvironment,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Railway(_) => StatusCode::BAD_GATEWAY,
            AppError::NoProductionEnvironment => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Redis(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Serialization(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
        };
        let body = Json(ErrorBody {
            error: self.to_string(),
        });
        (status, body).into_response()
    }
}
