use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::Session;
use crate::ServiceConfig;

pub type SharedState = Arc<AppState>;

pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

pub struct AppState {
    pub sessions: RwLock<HashMap<Uuid, Session>>,
    pub csrf_states: RwLock<HashSet<String>>,
    pub oauth_config: OAuthConfig,
}

impl AppState {
    pub fn new(config: ServiceConfig) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            csrf_states: RwLock::new(HashSet::new()),
            oauth_config: config.oauth,
        }
    }
}
