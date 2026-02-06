use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::models::{Container, ContainerEvent};

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub containers: RwLock<HashMap<Uuid, Container>>,
    pub event_tx: broadcast::Sender<ContainerEvent>,
}

impl AppState {
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(64);
        Self {
            containers: RwLock::new(HashMap::new()),
            event_tx,
        }
    }
}
