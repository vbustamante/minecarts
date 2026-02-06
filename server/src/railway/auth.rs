use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::state::OAuthConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RailwayUser {
    pub sub: String,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    // refresh_token: String,
    // expires_in: u64,
    // id_token: String,
    // scope: String,
}

pub fn build_auth_url(config: &OAuthConfig, csrf_state: &str) -> String {
    let params = [
        ("response_type", "code"),
        ("client_id", config.client_id.as_str()),
        ("redirect_uri", config.redirect_uri.as_str()),
        ("scope", "openid profile email offline_access project:member"),
        ("state", csrf_state),
        ("prompt", "consent"),
    ];

    let query = params
        .iter()
        .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    format!("https://backboard.railway.com/oauth/auth?{query}")
}

pub async fn exchange_code(
    client: &Client,
    config: &OAuthConfig,
    code: &str,
) -> Result<String, reqwest::Error> {
    let token_res = client
        .post("https://backboard.railway.com/oauth/token")
        .basic_auth(&config.client_id, Some(&config.client_secret))
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", config.redirect_uri.as_str()),
        ])
        .send()
        .await?
        .error_for_status()?;

    let token: TokenResponse = token_res.json().await?;
    Ok(token.access_token)
}

pub async fn fetch_user(
    client: &Client,
    access_token: &str,
) -> Result<RailwayUser, reqwest::Error> {
    client
        .get("https://backboard.railway.com/oauth/me")
        .bearer_auth(access_token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}
