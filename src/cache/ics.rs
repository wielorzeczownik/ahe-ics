use std::time::Duration;

use chrono::NaiveDate;
use moka::future::Cache;

use crate::constants::ICS_CACHE_TTL_SECONDS;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IcsCacheKey {
  pub student_id: i64,
  pub from: NaiveDate,
  pub to: NaiveDate,
}

#[derive(Clone, Debug)]
pub struct IcsCache {
  inner: Cache<IcsCacheKey, String>,
}

impl Default for IcsCache {
  fn default() -> Self {
    Self::new()
  }
}

impl IcsCache {
  #[must_use]
  pub fn new() -> Self {
    Self {
      inner: Cache::builder()
        .time_to_live(Duration::from_secs(ICS_CACHE_TTL_SECONDS))
        .build(),
    }
  }

  pub async fn get(&self, key: &IcsCacheKey) -> Option<String> {
    self.inner.get(key).await
  }

  pub async fn insert(&self, key: IcsCacheKey, value: String) {
    self.inner.insert(key, value).await;
  }
}
