use anyhow::{Context, Result};
use reqwest::Client;
use tracing::{debug, warn};

use super::API_BASE_URL;
use crate::models::PlanItem;

const API_PLAN_PATH: &str = "/api/PlanyZajec/GETPlanSzczegolowy";
const PLAN_INACTIVE_PARAM: &str = "CzyNieaktywnePlany=0";
const PLAN_LOADER_PARAM: &str = "loader=none";

/// Fetches the detailed schedule plan for the given student and date range.
pub async fn get_plan(
  client: &Client,
  access_token: &str,
  student_id: i64,
  date_from: &str,
  date_to: &str,
) -> Result<Vec<PlanItem>> {
  let url = format!(
    "{API_BASE_URL}{API_PLAN_PATH}?{PLAN_INACTIVE_PARAM}&DataDo={date_to}&DataOd={date_from}&StudentID={student_id}&{PLAN_LOADER_PARAM}"
  );

  debug!(student_id, date_from, date_to, "GET {API_PLAN_PATH}");
  let resp = client
    .get(url)
    .bearer_auth(access_token)
    .send()
    .await
    .context("plan request failed")?;

  let status = resp.status();
  if !status.is_success() {
    warn!(?status, "plan fetch failed");
    let text = resp.text().await.unwrap_or_default();
    anyhow::bail!("plan failed: {status} body={text}");
  }

  debug!(?status, "plan fetch ok");
  resp
    .json::<Vec<PlanItem>>()
    .await
    .context("invalid plan json")
}
