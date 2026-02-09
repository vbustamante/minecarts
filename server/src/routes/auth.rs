use axum::extract::{Query, State};
use axum::response::{Json, Redirect};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use redis::AsyncCommands;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::extractors::{UserSession, SESSION_COOKIE};
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
    code: String,
    state: String,
}

pub async fn callback(
    State(state): State<SharedState>,
    Query(params): Query<CallbackParams>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), AppError> {
    // Validate CSRF state
    let removed = state.csrf_states.write().await.remove(&params.state);
    if !removed {
        return Err(AppError::BadRequest("Invalid CSRF state".into()));
    }

    let client = reqwest::Client::new();

    let railway_auth = railway_auth::exchange_code(&client, &state.oauth_config, &params.code)
        .await?;

    let user = railway_auth::fetch_user(&client, &railway_auth.access_token)
        .await?;

    // Create session
    let session_id = Uuid::new_v4();
    let session = Session {
        user,
        railway_auth,
    };
    let session_json = serde_json::to_string(&session)?;

    let key = format!("session:{session_id}");
    let mut redis = state.redis.clone();
    let _ : () = redis
        .set(&key, &session_json)
        .await?;

    let cookie = Cookie::build((SESSION_COOKIE, session_id.to_string()))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::None);

    Ok((jar.add(cookie), Redirect::to(&state.frontend_url)))
}

pub async fn me(UserSession(session): UserSession) -> Json<RailwayUser> {
    Json(session.user)
}

pub async fn logout(
    State(state): State<SharedState>,
    jar: CookieJar,
) -> (CookieJar, Json<serde_json::Value>) {
    if let Some(cookie) = jar.get(SESSION_COOKIE) {
        if let Ok(session_id) = cookie.value().parse::<Uuid>() {
            let key = format!("session:{session_id}");
            let mut redis = state.redis.clone();
            let _: () = redis.del(&key).await.unwrap_or(());
        }
    }

    let removal = Cookie::build((SESSION_COOKIE, ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::None)
        .removal();

    (jar.remove(removal), Json(serde_json::json!({"ok": true})))
}
