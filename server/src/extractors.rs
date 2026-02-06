use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum_extra::extract::cookie::CookieJar;
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

        let sessions = state.sessions.read().await;
        let session = sessions.get(&session_id).ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(UserSession(session.clone()))
    }
}
