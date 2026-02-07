use std::collections::HashSet;
use std::sync::Arc;

use redis::aio::ConnectionManager;
use tokio::sync::RwLock;

use crate::ServiceConfig;

pub type SharedState = Arc<AppState>;

#[derive(Debug)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

pub struct AppState {
    pub redis: ConnectionManager,
    pub csrf_states: RwLock<HashSet<String>>,
    pub oauth_config: OAuthConfig,
}

impl AppState {
    pub fn new(config: ServiceConfig, redis: ConnectionManager) -> Self {
        Self {
            redis,
            csrf_states: RwLock::new(HashSet::new()),
            oauth_config: config.oauth,
        }
    }
}
