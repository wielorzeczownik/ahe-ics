use std::fmt;
use std::time::Duration;

use anyhow::Result;
use chrono::{DateTime, Utc};
use moka::future::Cache;
use tracing::debug;

use crate::api::ApiClient;
use crate::cache::{CredentialKey, credential_key};

const TOKEN_REFRESH_GRACE_SECONDS: u64 = 30;

#[derive(Clone)]
pub struct TokenCacheEntry {
  pub token: String,
  pub expires_at: DateTime<Utc>,
}

impl fmt::Debug for TokenCacheEntry {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct("TokenCacheEntry")
      .field("token", &"<redacted>")
      .field("expires_at", &self.expires_at)
      .finish()
  }
}

/// Per-user WPS access token cache, keyed by the full credential pair
pub struct TokenCache {
  inner: Cache<CredentialKey, TokenCacheEntry>,
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
    let key = credential_key(username, password);

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn debug_never_prints_the_access_token() {
    let entry = TokenCacheEntry {
      token: "wps-access-token".to_string(),
      expires_at: Utc::now(),
    };
    let rendered = format!("{entry:?}");

    assert!(!rendered.contains("wps-access-token"), "leaked: {rendered}");
    assert!(rendered.contains("expires_at"));
  }
}
