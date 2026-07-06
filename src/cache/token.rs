use std::time::Duration;

use anyhow::Result;
use chrono::{DateTime, Utc};
use moka::future::Cache;
use tracing::debug;

use crate::api::ApiClient;
use crate::constants::TOKEN_REFRESH_GRACE_SECONDS;

#[derive(Clone, Debug)]
pub struct TokenCacheEntry {
  pub token: String,
  pub expires_at: DateTime<Utc>,
}

/// Per-user WPS access token cache, keyed by username
pub struct TokenCache {
  inner: Cache<String, TokenCacheEntry>,
}

impl Default for TokenCache {
  fn default() -> Self {
    Self {
      inner: Cache::builder()
        .time_to_live(Duration::from_secs(86400))
        .build(),
    }
  }
}

impl TokenCache {
  /// Returns a valid WPS access token for the given credentials, logging in only when needed
  ///
  /// # Errors
  ///
  /// Returns an error if logging into the WPS API fails.
  pub async fn get_or_login(
    &self,
    username: &str,
    password: &str,
    api: &ApiClient,
  ) -> Result<String> {
    let key = username.to_string();

    if let Some(entry) = self.inner.get(&key).await {
      if entry.expires_at > Utc::now() {
        debug!("token cache hit");
        return Ok(entry.token.clone());
      }
      self.inner.invalidate(&key).await;
    }

    debug!("token cache miss, logging in");
    let token_resp = api.login(username, password).await?;
    let refresh_grace = token_resp
      .expires_in
      .saturating_sub(TOKEN_REFRESH_GRACE_SECONDS);
    let expires_at =
      Utc::now() + chrono::Duration::seconds(i64::try_from(refresh_grace).unwrap_or(i64::MAX));

    let entry = TokenCacheEntry {
      token: token_resp.access_token.clone(),
      expires_at,
    };
    self.inner.insert(key, entry).await;

    Ok(token_resp.access_token)
  }
}
