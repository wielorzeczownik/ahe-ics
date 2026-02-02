use std::sync::Arc;

use anyhow::Result;

use crate::api::ApiClient;
use crate::cache::{IcsCache, StudentContextCache, TokenCache};
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
  pub config: Config,
  pub api: ApiClient,
  pub token_cache: Arc<TokenCache>,
  pub student_context_cache: Arc<StudentContextCache>,
  pub ics_cache: IcsCache,
}

impl AppState {
  /// Builds the application state (API client + caches) from config.
  ///
  /// Returns an error if the HTTP client cannot be created.
  pub fn new(config: Config) -> Result<Self> {
    let api = ApiClient::new()?;
    Ok(Self {
      config,
      api,
      token_cache: Arc::new(TokenCache::new()),
      student_context_cache: Arc::new(StudentContextCache::new()),
      ics_cache: IcsCache::new(),
    })
  }
}
