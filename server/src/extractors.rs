use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum_extra::extract::cookie::CookieJar;
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::models::Session;
use crate::state::SharedState;

pub const SESSION_COOKIE: &str = "session_id";

pub struct UserSession(pub Session);

impl FromRequestParts<SharedState> for UserSession {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let session_id = jar
            .get(SESSION_COOKIE)
            .and_then(|c| c.value().parse::<Uuid>().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let session = {
            let sessions = state.sessions.read().await;
            let session = sessions.get(&session_id).ok_or(StatusCode::UNAUTHORIZED)?;
            session.clone()
        };

        if session.railway_auth.expires_on < Utc::now() + Duration::minutes(5) {
            let client = reqwest::Client::new();
            let new_auth = crate::railway::auth::refresh_token(
                &client,
                &state.oauth_config,
                &session.railway_auth.refresh_token,
            )
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

            let mut sessions = state.sessions.write().await;
            if let Some(s) = sessions.get_mut(&session_id) {
                s.railway_auth = new_auth;
                return Ok(UserSession(s.clone()));
            }
            return Err(StatusCode::UNAUTHORIZED);
        }

        Ok(UserSession(session))
    }
}
