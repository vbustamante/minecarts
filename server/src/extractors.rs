use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use chrono::{Duration, Utc};
use redis::AsyncCommands;
use uuid::Uuid;

use crate::models::Session;
use crate::state::SharedState;

pub struct UserSession(pub Session);

impl FromRequestParts<SharedState> for UserSession {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let session_id: Uuid = auth_header.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

        let key = format!("session:{session_id}");
        let mut redis = state.redis.clone();

        let session_json: String = redis
            .get(&key)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

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
