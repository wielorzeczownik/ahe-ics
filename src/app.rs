use std::sync::Arc;

use anyhow::Result;

use crate::api::ApiClient;
use crate::cache::{IcsCache, StudentContextCache, TokenCache};
use crate::config::ServerSettings;

#[derive(Clone)]
pub struct AppState<C: ServerSettings> {
  pub config: C,
  pub api: ApiClient,
  pub token_cache: Arc<TokenCache>,
  pub student_context_cache: Arc<StudentContextCache>,
  pub ics_cache: IcsCache,
}

impl<C: ServerSettings> AppState<C> {
  /// Builds the shared application state from the given configuration.
  ///
  /// # Errors
  ///
  /// Returns an error if the API client cannot be constructed.
  pub fn new(config: C) -> Result<Self> {
    let api = ApiClient::new()?;
    Ok(Self {
      config,
      api,
      token_cache: Arc::new(TokenCache::default()),
      student_context_cache: Arc::new(StudentContextCache::default()),
      ics_cache: IcsCache::new(),
    })
  }
}
