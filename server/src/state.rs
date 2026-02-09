use std::collections::HashSet;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::ServiceConfig;
use crate::session_store::SessionStore;

pub type SharedState = Arc<AppState>;

#[derive(Debug)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

pub struct AppState {
    pub sessions: SessionStore,
    pub csrf_states: RwLock<HashSet<String>>,
    pub oauth_config: OAuthConfig,
    pub frontend_url: String,
}

impl AppState {
    pub fn new(config: ServiceConfig, sessions: SessionStore) -> Self {
        Self {
            sessions,
            csrf_states: RwLock::new(HashSet::new()),
            oauth_config: config.oauth,
            frontend_url: config.frontend_url,
        }
    }
}
