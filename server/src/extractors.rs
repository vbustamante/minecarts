use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum_extra::extract::cookie::CookieJar;
use chrono::{Duration, Utc};
use redis::AsyncCommands;
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

        let key = format!("session:{session_id}");
        let mut redis = state.redis.clone();

        let session_json: String = redis
            .get(&key)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let session: Session =
            serde_json::from_str(&session_json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if session.railway_auth.expires_on < Utc::now() + Duration::minutes(5) {
            let client = reqwest::Client::new();
            let new_auth = crate::railway::auth::refresh_token(
                &client,
                &state.oauth_config,
                &session.railway_auth.refresh_token,
            )
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

            let updated_session = Session {
                user: session.user,
                railway_auth: new_auth,
            };
            let json = serde_json::to_string(&updated_session)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let _: () = redis
                .set(&key, &json)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            return Ok(UserSession(updated_session));
        }

        Ok(UserSession(session))
    }
}
