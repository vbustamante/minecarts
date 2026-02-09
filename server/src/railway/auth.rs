use chrono::{Duration, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::state::OAuthConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RailwayUser {
    pub sub: String,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub email: String,
    pub email_verified: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RailwayAuthData {
    pub access_token: String,
    pub refresh_token: String,
    expires_in: u64,
    #[serde(default = "default_expires_on")]
    pub expires_on: chrono::DateTime<Utc>,
    // unused fields
    // id_token: String,
    // scope: String,
}

fn default_expires_on() -> chrono::DateTime<Utc> {
    Utc::now()
}

impl RailwayAuthData {
    pub fn with_expires_on(mut self) -> Self {
        self.expires_on = Utc::now() + Duration::seconds(self.expires_in as i64);
        self
    }
}

pub fn build_auth_url(config: &OAuthConfig, csrf_state: &str, ask_for_projects: bool) -> String {
    let mut params = Vec::from([
        ("response_type", "code"),
        ("client_id", config.client_id.as_str()),
        ("redirect_uri", config.redirect_uri.as_str()),
        ("scope", "openid profile email offline_access workspace:admin project:member"),
        ("state", csrf_state)
    ]);

    if ask_for_projects {
        params.push(("prompt", "consent"));
    }

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
) -> Result<RailwayAuthData, reqwest::Error> {
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

    let token: RailwayAuthData = token_res.json::<RailwayAuthData>().await?.with_expires_on();
    Ok(token)
}

pub async fn refresh_token(
    client: &Client,
    config: &OAuthConfig,
    refresh_token: &str,
) -> Result<RailwayAuthData, reqwest::Error> {
    let token_res = client
        .post("https://backboard.railway.com/oauth/token")
        .basic_auth(&config.client_id, Some(&config.client_secret))
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ])
        .send()
        .await?
        .error_for_status()?;

    let token: RailwayAuthData = token_res.json::<RailwayAuthData>().await?.with_expires_on();
    Ok(token)
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
