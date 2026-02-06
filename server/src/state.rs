use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::models::{Container, ContainerEvent, Session};

pub type SharedState = Arc<AppState>;

pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

pub struct AppState {
    pub containers: RwLock<HashMap<Uuid, Container>>,
    pub event_tx: broadcast::Sender<ContainerEvent>,
    pub sessions: RwLock<HashMap<Uuid, Session>>,
    pub csrf_states: RwLock<HashSet<String>>,
    pub oauth_config: OAuthConfig,
}

impl AppState {
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(64);

        let client_id = std::env::var("RAILWAY_CLIENT_ID")
            .expect("RAILWAY_CLIENT_ID must be set");
        let client_secret = std::env::var("RAILWAY_CLIENT_SECRET")
            .expect("RAILWAY_CLIENT_SECRET must be set");
        let redirect_uri = std::env::var("RAILWAY_REDIRECT_URI")
            .unwrap_or_else(|_| "http://localhost:5173/api/auth/callback".to_string());

        Self {
            containers: RwLock::new(HashMap::new()),
            event_tx,
            sessions: RwLock::new(HashMap::new()),
            csrf_states: RwLock::new(HashSet::new()),
            oauth_config: OAuthConfig {
                client_id,
                client_secret,
                redirect_uri,
            },
        }
    }
}
