use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub id: Uuid,
    pub name: String,
    pub image: String,
    pub status: ContainerStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContainerStatus {
    Running,
    Stopped,
    Created,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContainerRequest {
    pub name: String,
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContainerRequest {
    pub name: Option<String>,
    pub image: Option<String>,
    pub status: Option<ContainerStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerEvent {
    pub action: EventAction,
    pub container: Container,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventAction {
    Created,
    Updated,
    Deleted,
}
