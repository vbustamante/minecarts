use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::{Json, Redirect};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::extractors::UserSession;
use crate::models::Session;
use crate::railway::auth::{self as railway_auth, RailwayUser};
use crate::state::SharedState;

pub async fn login(State(state): State<SharedState>) -> Redirect {
    let csrf_state = Uuid::new_v4().to_string();
    state.csrf_states.write().await.insert(csrf_state.clone());

    let url = railway_auth::build_auth_url(&state.oauth_config, &csrf_state, true);
    Redirect::to(&url)
}

#[derive(Deserialize, Debug)]
pub struct CallbackParams {
    state: String,
    code: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

pub async fn callback(
    State(state): State<SharedState>,
    Query(params): Query<CallbackParams>,
) -> Result<Redirect, AppError> {
    // Validate CSRF state
    let removed = state.csrf_states.write().await.remove(&params.state);
    if !removed {
        return Err(AppError::BadRequest("Invalid CSRF state".into()));
    }

    // Handle OAuth error response
    if let Some(error) = params.error {
        let message = params.error_description.unwrap_or_default();
        let redirect_url = format!(
            "{}/login?error={}&error_message={}",
            state.frontend_url,
            urlencoding::encode(&error),
            urlencoding::encode(&message),
        );
        return Ok(Redirect::to(&redirect_url));
    }

    let code = params.code.ok_or_else(|| {
        AppError::BadRequest("Missing authorization code".into())
    })?;

    let client = reqwest::Client::new();

    let railway_auth = railway_auth::exchange_code(&client, &state.oauth_config, &code)
        .await?;

    let user = railway_auth::fetch_user(&client, &railway_auth.access_token)
        .await?;

    // Create session
    let session_id = Uuid::new_v4();
    let session = Session {
        user,
        railway_auth,
    };
    state.sessions.set(session_id, &session).await?;

    let redirect_url = format!("{}?session_id={}", state.frontend_url, session_id);
    Ok(Redirect::to(&redirect_url))
}

pub async fn me(UserSession(session): UserSession) -> Json<RailwayUser> {
    Json(session.user)
}

pub async fn logout(
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> Result<(), AppError> {
    if let Some(auth_header) = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
    {
        if let Ok(session_id) = auth_header.parse::<Uuid>() {
            state.sessions.delete(session_id).await?;
        }
    }

    Ok(())
}
