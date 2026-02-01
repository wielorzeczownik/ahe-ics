use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct TokenResponse {
  pub access_token: String,
  pub token_type: String,
  pub expires_in: u64,
}
