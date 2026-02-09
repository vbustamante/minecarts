use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use chrono::{Duration, Utc};
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
        println!("UserSession extractor");
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let session_id: Uuid = auth_header.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

        let session = state
            .sessions
            .get(session_id)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

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
            state
                .sessions
                .set(session_id, &updated_session)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            return Ok(UserSession(updated_session));
        }

        Ok(UserSession(session))
    }
}
