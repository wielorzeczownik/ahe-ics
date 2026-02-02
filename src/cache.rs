use std::hash::{Hash, Hasher};
use std::time::Duration;

use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use moka::future::Cache;
use tokio::sync::RwLock;
use tracing::{debug, warn};

use crate::api::ApiClient;
use crate::config::Config;
use crate::constants::{
  ICS_CACHE_TTL_SECONDS, STUDENT_CONTEXT_CACHE_TTL_SECONDS, TOKEN_REFRESH_GRACE_SECONDS,
};
use crate::models::StudentIndex;

#[derive(Clone, Debug)]
pub struct TokenCacheEntry {
  pub token: String,
  pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Default)]
pub struct TokenCache {
  inner: RwLock<Option<TokenCacheEntry>>,
}

#[derive(Clone, Debug)]
pub struct StudentContext {
  pub student_id: i64,
  pub index_id: Option<i64>,
}

#[derive(Clone, Debug)]
struct StudentContextCacheEntry {
  value: StudentContext,
  expires_at: DateTime<Utc>,
}

#[derive(Debug, Default)]
pub struct StudentContextCache {
  inner: RwLock<Option<StudentContextCacheEntry>>,
}

impl StudentContextCache {
  /// Creates an empty student context cache.
  pub fn new() -> Self {
    Self {
      inner: RwLock::new(None),
    }
  }

  /// Returns cached student metadata, loading it from API when needed.
  pub async fn get_or_fetch(
    &self,
    config: &Config,
    api: &ApiClient,
    access_token: &str,
  ) -> Result<StudentContext> {
    if let Some(value) = self.valid_context().await {
      debug!("student context cache hit");
      return Ok(value);
    }

    let mut guard = self.inner.write().await;
    if let Some(entry) = guard.as_ref() {
      if entry.expires_at > Utc::now() {
        debug!("student context cache hit after lock");
        return Ok(entry.value.clone());
      }
    }

    debug!("student context cache miss, fetching from API");
    let student_data = api.get_student_data(access_token).await?;
    let student_id = student_data.student_id;

    let mut index_id = if config.exams_enabled {
      student_data.index_id
    } else {
      None
    };

    if config.exams_enabled && index_id.is_none() {
      match api.get_student_indexes(access_token).await {
        Ok(indexes) => {
          index_id = pick_index_id(&indexes);
          if let Some(found) = index_id {
            debug!(
              student_id,
              index_id = found,
              "IndeksID resolved from indeks list"
            );
          } else {
            warn!(student_id, "student indeks list is empty, skipping exams");
          }
        }
        Err(error) => {
          warn!(student_id, error = %error, "failed to fetch indeks list, skipping exams");
        }
      }
    }

    let value = StudentContext {
      student_id,
      index_id,
    };
    let expires_at =
      Utc::now() + chrono::Duration::seconds(STUDENT_CONTEXT_CACHE_TTL_SECONDS as i64);

    *guard = Some(StudentContextCacheEntry {
      value: value.clone(),
      expires_at,
    });

    Ok(value)
  }

  async fn valid_context(&self) -> Option<StudentContext> {
    let guard = self.inner.read().await;
    guard
      .as_ref()
      .filter(|entry| entry.expires_at > Utc::now())
      .map(|entry| entry.value.clone())
  }
}

impl TokenCache {
  /// Creates an empty token cache.
  pub fn new() -> Self {
    Self {
      inner: RwLock::new(None),
    }
  }

  /// Returns a valid access token, logging in only if required.
  pub async fn get_or_login(&self, config: &Config, api: &ApiClient) -> Result<String> {
    if let Some(token) = self.valid_token().await {
      debug!("token cache hit");
      return Ok(token);
    }

    let mut guard = self.inner.write().await;
    if let Some(entry) = guard.as_ref() {
      if entry.expires_at > Utc::now() {
        debug!("token cache hit after lock");
        return Ok(entry.token.clone());
      }
    }

    debug!("token cache miss, logging in");
    let token = api.login(&config.username, &config.password).await?;
    let refresh_grace = token.expires_in.saturating_sub(TOKEN_REFRESH_GRACE_SECONDS);
    let expires_at = Utc::now() + chrono::Duration::seconds(refresh_grace as i64);

    let entry = TokenCacheEntry {
      token: token.access_token.clone(),
      expires_at,
    };
    *guard = Some(entry);

    Ok(token.access_token)
  }

  async fn valid_token(&self) -> Option<String> {
    let guard = self.inner.read().await;
    guard
      .as_ref()
      .filter(|entry| entry.expires_at > Utc::now())
      .map(|entry| entry.token.clone())
  }
}

#[derive(Clone, Debug, Eq)]
pub struct IcsCacheKey {
  pub student_id: i64,
  pub from: NaiveDate,
  pub to: NaiveDate,
}

impl PartialEq for IcsCacheKey {
  fn eq(&self, other: &Self) -> bool {
    self.student_id == other.student_id && self.from == other.from && self.to == other.to
  }
}

impl Hash for IcsCacheKey {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.student_id.hash(state);
    self.from.hash(state);
    self.to.hash(state);
  }
}

#[derive(Clone, Debug)]
pub struct IcsCache {
  inner: Cache<IcsCacheKey, String>,
}

impl IcsCache {
  /// Creates an ICS cache with a fixed TTL.
  pub fn new() -> Self {
    let inner = Cache::builder()
      .time_to_live(Duration::from_secs(ICS_CACHE_TTL_SECONDS))
      .build();
    Self { inner }
  }

  /// Returns cached ICS data for the given key, if present.
  pub async fn get(&self, key: &IcsCacheKey) -> Option<String> {
    self.inner.get(key).await
  }

  /// Stores ICS data in cache for the given key.
  pub async fn insert(&self, key: IcsCacheKey, value: String) {
    self.inner.insert(key, value).await;
  }
}

fn pick_index_id(indexes: &[StudentIndex]) -> Option<i64> {
  indexes
    .iter()
    .max_by_key(|item| {
      (
        item.status_symbol.as_deref() == Some("S"),
        item.year.unwrap_or_default(),
        item.semester.unwrap_or_default(),
        item.index_id,
      )
    })
    .map(|item| item.index_id)
}
