use anyhow::Result;

use super::ServerSettings;
use super::parse;
use super::types::{CalendarLanguage, CalendarToken};

/// Shared configuration
#[derive(Clone, Debug)]
pub struct SharedConfig {
  pub bind_addr: String,
  pub calendar_past_days: i64,
  pub calendar_future_days: i64,
  pub calendar_token: Option<CalendarToken>,
  pub calendar_lang: CalendarLanguage,
  pub exams_enabled: bool,
  pub json_enabled: bool,
  pub real_ip_header: Option<String>,
}

impl SharedConfig {
  /// Loads the shared-mode configuration from environment variables.
  ///
  /// # Errors
  ///
  /// Returns an error if a required variable is missing or fails to parse.
  pub fn from_env() -> Result<Self> {
    Ok(Self {
      bind_addr: parse::bind_addr(),
      calendar_past_days: parse::calendar_past_days()?,
      calendar_future_days: parse::calendar_future_days()?,
      calendar_token: parse::calendar_token()?,
      calendar_lang: parse::calendar_lang()?,
      exams_enabled: parse::exams_enabled()?,
      json_enabled: parse::json_enabled()?,
      real_ip_header: parse::real_ip_header()?,
    })
  }
}

impl ServerSettings for SharedConfig {
  fn calendar_past_days(&self) -> i64 {
    self.calendar_past_days
  }
  fn calendar_future_days(&self) -> i64 {
    self.calendar_future_days
  }
  fn calendar_token(&self) -> Option<&CalendarToken> {
    self.calendar_token.as_ref()
  }
  fn calendar_lang(&self) -> CalendarLanguage {
    self.calendar_lang
  }
  fn exams_enabled(&self) -> bool {
    self.exams_enabled
  }
  fn json_enabled(&self) -> bool {
    self.json_enabled
  }
  fn real_ip_header(&self) -> Option<&str> {
    self.real_ip_header.as_deref()
  }
}
