use anyhow::{Context, Result};
use reqwest::Client;
use tracing::{debug, warn};

use crate::constants::{API_BASE_URL, API_STUDENT_INDEXES_PATH};
use crate::models::StudentIndex;

/// Fetches all indeks entries for the currently authenticated student.
pub async fn get_student_indexes(client: &Client, access_token: &str) -> Result<Vec<StudentIndex>> {
  let url = format!("{API_BASE_URL}{API_STUDENT_INDEXES_PATH}");

  debug!("GET {API_STUDENT_INDEXES_PATH}");
  let resp = client
    .get(url)
    .bearer_auth(access_token)
    .send()
    .await
    .context("student indexes request failed")?;

  let status = resp.status();
  if !status.is_success() {
    warn!(?status, "student indexes fetch failed");
    let text = resp.text().await.unwrap_or_default();
    anyhow::bail!("student indexes failed: {status} body={text}");
  }

  debug!(?status, "student indexes fetch ok");
  Ok(
    resp
      .json::<Vec<StudentIndex>>()
      .await
      .context("invalid student indexes json")?,
  )
}
