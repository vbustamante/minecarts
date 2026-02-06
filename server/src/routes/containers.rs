use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use crate::models::*;
use crate::state::SharedState;

pub async fn list(State(state): State<SharedState>) -> Json<Vec<Container>> {
    let containers = state.containers.read().await;
    Json(containers.values().cloned().collect())
}

pub async fn get(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Container>, StatusCode> {
    let containers = state.containers.read().await;
    containers
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn create(
    State(state): State<SharedState>,
    Json(req): Json<CreateContainerRequest>,
) -> (StatusCode, Json<Container>) {
    let container = Container {
        id: Uuid::new_v4(),
        name: req.name,
        image: req.image,
        status: ContainerStatus::Created,
    };

    state
        .containers
        .write()
        .await
        .insert(container.id, container.clone());

    let _ = state.event_tx.send(ContainerEvent {
        action: EventAction::Created,
        container: container.clone(),
    });

    (StatusCode::CREATED, Json(container))
}

pub async fn  update(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateContainerRequest>,
) -> Result<Json<Container>, StatusCode> {
    let mut containers = state.containers.write().await;
    let container = containers.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;

    if let Some(name) = req.name {
        container.name = name;
    }
    if let Some(image) = req.image {
        container.image = image;
    }
    if let Some(status) = req.status {
        container.status = status;
    }

    let container = container.clone();

    let _ = state.event_tx.send(ContainerEvent {
        action: EventAction::Updated,
        container: container.clone(),
    });

    Ok(Json(container))
}

pub async fn delete(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut containers = state.containers.write().await;
    let container = containers.remove(&id).ok_or(StatusCode::NOT_FOUND)?;

    let _ = state.event_tx.send(ContainerEvent {
        action: EventAction::Deleted,
        container,
    });

    Ok(StatusCode::NO_CONTENT)
}
