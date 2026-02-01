mod auth;
mod schedule;
mod student;

use anyhow::Result;
use reqwest::Client;

use crate::constants::USER_AGENT;
use crate::models::{ PlanItem, StudentData, TokenResponse };

#[derive(Clone)]
pub struct ApiClient {
  http: Client,
}

impl ApiClient {
  /// Creates a new API client with a configured user-agent.
  pub fn new() -> Result<Self> {
    let http = Client::builder().user_agent(USER_AGENT).build()?;
    Ok(Self { http })
  }

  /// Logs into the WPS API and returns an access token payload.
  pub async fn login(&self, username: &str, password: &str) -> Result<TokenResponse> {
    auth::login(&self.http, username, password).await
  }

  /// Fetches the detailed schedule plan for a student in a date range.
  pub async fn get_plan(
    &self,
    access_token: &str,
    student_id: i64,
    date_from: &str,
    date_to: &str
  ) -> Result<Vec<PlanItem>> {
    schedule::get_plan(&self.http, access_token, student_id, date_from, date_to).await
  }

  /// Fetches the current student's data (includes `IDStudent`).
  pub async fn get_student_data(&self, access_token: &str) -> Result<StudentData> {
    student::get_student_data(&self.http, access_token).await
  }
}
