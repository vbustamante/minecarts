use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{Json, Redirect};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::Session;
use crate::railway::auth::{self as railway_auth, RailwayUser};
use crate::state::SharedState;

const SESSION_COOKIE: &str = "session_id";

pub async fn login(State(state): State<SharedState>) -> Redirect {
    let csrf_state = Uuid::new_v4().to_string();
    state.csrf_states.write().await.insert(csrf_state.clone());

    let url = railway_auth::build_auth_url(&state.oauth_config, &csrf_state);
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
) -> Result<(CookieJar, Redirect), StatusCode> {
    // Validate CSRF state
    let removed = state.csrf_states.write().await.remove(&params.state);
    if !removed {
        return Err(StatusCode::BAD_REQUEST);
    }

    let client = reqwest::Client::new();

    let access_token = railway_auth::exchange_code(&client, &state.oauth_config, &params.code)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let user = railway_auth::fetch_user(&client, &access_token)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    // Create session
    let session_id = Uuid::new_v4();
    state.sessions.write().await.insert(session_id, Session { user });

    let cookie = Cookie::build((SESSION_COOKIE, session_id.to_string()))
        .path("/")
        .http_only(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax);

    Ok((jar.add(cookie), Redirect::to("/")))
}

pub async fn me(
    State(state): State<SharedState>,
    jar: CookieJar,
) -> Result<Json<RailwayUser>, StatusCode> {
    let session_id = jar
        .get(SESSION_COOKIE)
        .and_then(|c| c.value().parse::<Uuid>().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let sessions = state.sessions.read().await;
    let session = sessions.get(&session_id).ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(Json(session.user.clone()))
}

pub async fn logout(
    State(state): State<SharedState>,
    jar: CookieJar,
) -> (CookieJar, Redirect) {
    if let Some(cookie) = jar.get(SESSION_COOKIE) {
        if let Ok(session_id) = cookie.value().parse::<Uuid>() {
            state.sessions.write().await.remove(&session_id);
        }
    }

    let removal = Cookie::build((SESSION_COOKIE, ""))
        .path("/")
        .http_only(true)
        .removal();

    (jar.remove(removal), Redirect::to("/"))
}
