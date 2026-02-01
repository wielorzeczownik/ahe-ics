use anyhow::{ Context, Result };
use reqwest::Client;
use tracing::{ debug, warn };

use crate::constants::{ API_BASE_URL, API_STUDENT_PATH };
use crate::models::StudentData;

/// Fetches data for the currently authenticated student.
pub async fn get_student_data(client: &Client, access_token: &str) -> Result<StudentData> {
  let url = format!("{API_BASE_URL}{API_STUDENT_PATH}");

  debug!("GET {API_STUDENT_PATH}");
  let resp = client
    .get(url)
    .bearer_auth(access_token)
    .send().await
    .context("student data request failed")?;

  let status = resp.status();
  if !status.is_success() {
    warn!(?status, "student data fetch failed");
    let text = resp.text().await.unwrap_or_default();
    anyhow::bail!("student data failed: {status} body={text}");
  }

  debug!(?status, "student data fetch ok");
  Ok(resp.json::<StudentData>().await.context("invalid student data json")?)
}
