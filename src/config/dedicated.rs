use std::fmt;

use anyhow::{Context, Result};

use super::ServerSettings;
use super::parse;
use super::types::{CalendarLanguage, CalendarToken};

/// Dedicated configuration
#[derive(Clone)]
pub struct Config {
  pub username: String,
  pub password: String,
  pub bind_addr: String,
  pub calendar_past_days: i64,
  pub calendar_future_days: i64,
  pub calendar_token: Option<CalendarToken>,
  pub calendar_lang: CalendarLanguage,
  pub exams_enabled: bool,
  pub json_enabled: bool,
  pub real_ip_header: Option<String>,
}

impl fmt::Debug for Config {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct("Config")
      .field("username", &self.username)
      .field("password", &"<redacted>")
      .field("bind_addr", &self.bind_addr)
      .field("calendar_past_days", &self.calendar_past_days)
      .field("calendar_future_days", &self.calendar_future_days)
      .field("calendar_token", &self.calendar_token)
      .field("calendar_lang", &self.calendar_lang)
      .field("exams_enabled", &self.exams_enabled)
      .field("json_enabled", &self.json_enabled)
      .field("real_ip_header", &self.real_ip_header)
      .finish()
  }
}

impl Config {
  /// Loads the dedicated-mode configuration from environment variables.
  ///
  /// # Errors
  ///
  /// Returns an error if a required variable is missing or fails to parse.
  pub fn from_env() -> Result<Self> {
    let username = std::env::var("AHE_USERNAME").context("AHE_USERNAME is required")?;
    let password = std::env::var("AHE_PASSWORD").context("AHE_PASSWORD is required")?;

    Ok(Self {
      username,
      password,
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

impl ServerSettings for Config {
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

#[cfg(test)]
mod tests {
  use super::*;

  fn sample() -> Config {
    Config {
      username: "jan.kowalski".to_string(),
      password: "super-tajne".to_string(),
      bind_addr: "0.0.0.0:8080".to_string(),
      calendar_past_days: 60,
      calendar_future_days: 60,
      calendar_token: Some(CalendarToken::Plain("kalendarz-token".to_string())),
      calendar_lang: CalendarLanguage::Pl,
      exams_enabled: true,
      json_enabled: true,
      real_ip_header: None,
    }
  }

  #[test]
  fn debug_never_prints_the_password() {
    let rendered = format!("{:?}", sample());

    assert!(!rendered.contains("super-tajne"), "leaked: {rendered}");
    // The nested token must be redacted by its own Debug as well
    assert!(!rendered.contains("kalendarz-token"), "leaked: {rendered}");
  }

  #[test]
  fn debug_keeps_the_non_secret_fields() {
    let rendered = format!("{:?}", sample());

    assert!(rendered.contains("jan.kowalski"));
    assert!(rendered.contains("0.0.0.0:8080"));
    assert!(rendered.contains("exams_enabled"));
  }
}
