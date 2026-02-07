use serde::{Deserialize, Serialize};

use crate::railway::auth::{RailwayAuthData, RailwayUser};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub user: RailwayUser,
    pub railway_auth: RailwayAuthData,
}
