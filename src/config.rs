use anyhow::{Context, Result, bail};

use crate::constants::{
  DEFAULT_BIND_ADDR, DEFAULT_CAL_FUTURE_DAYS, DEFAULT_CAL_LANG, DEFAULT_CAL_PAST_DAYS,
  DEFAULT_EXAMS_ENABLED, DEFAULT_JSON_ENABLED, DEFAULT_OPENAPI_ENABLED,
};

#[derive(Clone, Copy, Debug)]
pub enum CalendarLanguage {
  Pl,
  En,
}

impl CalendarLanguage {
  fn from_env_value(value: &str) -> Result<Self> {
    match value.trim().to_ascii_lowercase().as_str() {
      "pl" => Ok(Self::Pl),
      "en" => Ok(Self::En),
      _ => bail!("AHE_CAL_LANG must be one of: pl, en"),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Config {
  pub username: String,
  pub password: String,
  pub bind_addr: String,
  pub calendar_past_days: i64,
  pub calendar_future_days: i64,
  pub calendar_token: Option<String>,
  pub calendar_lang: CalendarLanguage,
  pub exams_enabled: bool,
  pub json_enabled: bool,
  pub openapi_enabled: bool,
}

impl Config {
  /// Loads configuration from environment variables.
  ///
  /// Returns an error if required variables are missing or invalid.
  pub fn from_env() -> Result<Self> {
    let username = std::env::var("AHE_USERNAME").context("AHE_USERNAME is required")?;
    let password = std::env::var("AHE_PASSWORD").context("AHE_PASSWORD is required")?;
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_string());
    let calendar_past_days = parse_days_env("AHE_CAL_PAST_DAYS", DEFAULT_CAL_PAST_DAYS)?;
    let calendar_future_days = parse_days_env("AHE_CAL_FUTURE_DAYS", DEFAULT_CAL_FUTURE_DAYS)?;
    let calendar_token = std::env::var("AHE_CAL_TOKEN")
      .ok()
      .map(|value| value.trim().to_string())
      .filter(|value| !value.is_empty());
    let calendar_lang = parse_lang_env("AHE_CAL_LANG", DEFAULT_CAL_LANG)?;
    let exams_enabled = parse_bool_env("AHE_CAL_EXAMS_ENABLED", DEFAULT_EXAMS_ENABLED)?;
    let json_enabled = parse_bool_env("AHE_CAL_JSON_ENABLED", DEFAULT_JSON_ENABLED)?;
    let openapi_enabled = parse_bool_env("AHE_OPENAPI_ENABLED", DEFAULT_OPENAPI_ENABLED)?;

    Ok(Self {
      username,
      password,
      bind_addr,
      calendar_past_days,
      calendar_future_days,
      calendar_token,
      calendar_lang,
      exams_enabled,
      json_enabled,
      openapi_enabled,
    })
  }
}

fn parse_days_env(key: &str, default_value: i64) -> Result<i64> {
  let Some(raw) = std::env::var(key).ok() else {
    return Ok(default_value);
  };

  let value: i64 = raw
    .parse()
    .with_context(|| format!("{key} must be a non-negative integer"))?;

  if value < 0 {
    bail!("{key} must be a non-negative integer");
  }

  Ok(value)
}

fn parse_lang_env(key: &str, default_value: &str) -> Result<CalendarLanguage> {
  let value = std::env::var(key).unwrap_or_else(|_| default_value.to_string());
  CalendarLanguage::from_env_value(&value)
}

fn parse_bool_env(key: &str, default_value: bool) -> Result<bool> {
  let Some(raw) = std::env::var(key).ok() else {
    return Ok(default_value);
  };

  match raw.trim().to_ascii_lowercase().as_str() {
    "1" | "true" | "yes" | "on" => Ok(true),
    "0" | "false" | "no" | "off" => Ok(false),
    _ => bail!("{key} must be a boolean value (true/false, 1/0, yes/no, on/off)"),
  }
}
