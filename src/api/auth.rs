use anyhow::{ Context, Result };
use reqwest::Client;
use tracing::{ debug, warn };

use crate::constants::{ API_BASE_URL, API_LOGIN_PATH, LOGIN_GRANT_TYPE, LOGIN_ROLE_ID };
use crate::models::TokenResponse;

/// Performs the WPS login and returns the access token response.
pub async fn login(client: &Client, username: &str, password: &str) -> Result<TokenResponse> {
  let url = format!("{API_BASE_URL}{API_LOGIN_PATH}");

  debug!("POST {API_LOGIN_PATH}");
  let resp = client
    .post(url)
    .form(
      &[
        ("username", username),
        ("password", password),
        ("roleID", LOGIN_ROLE_ID),
        ("grant_type", LOGIN_GRANT_TYPE),
      ]
    )
    .send().await
    .context("login request failed")?;

  let status = resp.status();
  if !status.is_success() {
    warn!(?status, "login failed");
    let text = resp.text().await.unwrap_or_default();
    anyhow::bail!("login failed: {status} body={text}");
  }

  debug!(?status, "login ok");
  Ok(resp.json::<TokenResponse>().await.context("invalid token json")?)
}
