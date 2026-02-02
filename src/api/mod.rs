mod auth;
mod exams;
mod indexes;
mod schedule;
mod student;

use anyhow::Result;
use chrono::NaiveDate;
use reqwest::Client;

use crate::constants::USER_AGENT;
use crate::models::{ExamEvent, PlanItem, StudentData, StudentIndex, TokenResponse};

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
    date_to: &str,
  ) -> Result<Vec<PlanItem>> {
    schedule::get_plan(&self.http, access_token, student_id, date_from, date_to).await
  }

  /// Fetches the current student's data (includes `IDStudent`).
  pub async fn get_student_data(&self, access_token: &str) -> Result<StudentData> {
    student::get_student_data(&self.http, access_token).await
  }

  /// Fetches indeks entries for the current student.
  pub async fn get_student_indexes(&self, access_token: &str) -> Result<Vec<StudentIndex>> {
    indexes::get_student_indexes(&self.http, access_token).await
  }

  /// Fetches exam events for a student's index in the selected date range.
  pub async fn get_exams(
    &self,
    access_token: &str,
    index_id: i64,
    from: NaiveDate,
    to: NaiveDate,
  ) -> Result<Vec<ExamEvent>> {
    exams::get_exams(&self.http, access_token, index_id, from, to).await
  }
}
