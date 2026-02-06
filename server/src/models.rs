use crate::railway::auth::{RailwayAuthData, RailwayUser};

#[derive(Debug, Clone)]
pub struct Session {
    pub user: RailwayUser,
    pub railway_auth: RailwayAuthData,
}
