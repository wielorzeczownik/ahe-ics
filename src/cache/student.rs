use std::time::Duration;

use anyhow::Result;
use moka::future::Cache;
use tracing::{debug, warn};

use crate::api::ApiClient;
use crate::constants::STUDENT_CONTEXT_CACHE_TTL_SECONDS;
use crate::models::StudentIndex;

#[derive(Clone, Debug)]
pub struct StudentContext {
  pub student_id: i64,
  pub index_id: Option<i64>,
  pub section_name: Option<String>,
}

/// Per-user student metadata cache, keyed by username
pub struct StudentContextCache {
  inner: Cache<String, StudentContext>,
}

impl Default for StudentContextCache {
  fn default() -> Self {
    Self {
      inner: Cache::builder()
        .time_to_live(Duration::from_secs(STUDENT_CONTEXT_CACHE_TTL_SECONDS))
        .build(),
    }
  }
}

impl StudentContextCache {
  /// Returns cached student metadata for the given user, fetching from API when needed
  ///
  /// # Errors
  ///
  /// Returns an error if fetching the student context from the API fails.
  pub async fn get_or_fetch(
    &self,
    username: &str,
    exams_enabled: bool,
    api: &ApiClient,
    access_token: &str,
  ) -> Result<StudentContext> {
    if let Some(ctx) = self.inner.get(username).await {
      debug!("student context cache hit");
      return Ok(ctx);
    }

    debug!("student context cache miss, fetching from API");
    let student_data = api.get_student_data(access_token).await?;
    let student_id = student_data.student_id;

    let mut index_id = if exams_enabled {
      student_data.index_id
    } else {
      None
    };
    let mut section_name = None;

    if exams_enabled && index_id.is_none() {
      match api.get_student_indexes(access_token).await {
        Ok(indexes) => {
          if let Some(index) = pick_index(&indexes) {
            index_id = Some(index.index_id);
            section_name.clone_from(&index.section_name);
            debug!(
              student_id,
              index_id = index.index_id,
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

    let ctx = StudentContext {
      student_id,
      index_id,
      section_name,
    };
    self.inner.insert(username.to_string(), ctx.clone()).await;

    Ok(ctx)
  }
}

fn pick_index(indexes: &[StudentIndex]) -> Option<&StudentIndex> {
  indexes.iter().max_by_key(|item| {
    (
      item.status_symbol.as_deref() == Some("S"),
      item.year.unwrap_or_default(),
      item.semester.unwrap_or_default(),
      item.index_id,
    )
  })
}
