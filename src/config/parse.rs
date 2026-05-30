use anyhow::{Context, Result, bail};

use crate::constants::{
  DEFAULT_BIND_ADDR, DEFAULT_CAL_FUTURE_DAYS, DEFAULT_CAL_LANG, DEFAULT_CAL_PAST_DAYS,
  DEFAULT_EXAMS_ENABLED, DEFAULT_JSON_ENABLED,
};

use super::types::{CalendarLanguage, CalendarToken};

pub(super) fn bind_addr() -> String {
  std::env::var("BIND_ADDR").unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_string())
}

pub(super) fn calendar_past_days() -> Result<i64> {
  parse_days("AHE_CAL_PAST_DAYS", DEFAULT_CAL_PAST_DAYS)
}

pub(super) fn calendar_future_days() -> Result<i64> {
  parse_days("AHE_CAL_FUTURE_DAYS", DEFAULT_CAL_FUTURE_DAYS)
}

pub(super) fn calendar_token() -> Result<Option<CalendarToken>> {
  let Some(raw) = std::env::var("AHE_CAL_TOKEN").ok() else {
    return Ok(None);
  };

  let token = CalendarToken::from_env_value(raw.trim())
    .context("AHE_CAL_TOKEN is invalid; provide plain token or Argon2id hash")?;

  Ok(Some(token))
}

pub(super) fn calendar_lang() -> Result<CalendarLanguage> {
  let value = std::env::var("AHE_CAL_LANG").unwrap_or_else(|_| DEFAULT_CAL_LANG.to_string());
  CalendarLanguage::from_env_value(&value)
}

pub(super) fn exams_enabled() -> Result<bool> {
  parse_bool("AHE_CAL_EXAMS_ENABLED", DEFAULT_EXAMS_ENABLED)
}

pub(super) fn json_enabled() -> Result<bool> {
  parse_bool("AHE_CAL_JSON_ENABLED", DEFAULT_JSON_ENABLED)
}

pub(super) fn real_ip_header() -> Result<Option<String>> {
  let raw = std::env::var("REAL_IP_HEADER")
    .ok()
    .or_else(|| std::env::var("AHE_REAL_IP_HEADER").ok());

  let Some(raw) = raw else {
    return Ok(None);
  };

  let value = raw.trim();
  if value.is_empty() {
    bail!("REAL_IP_HEADER cannot be empty");
  }

  Ok(Some(value.to_ascii_lowercase()))
}

fn parse_days(key: &str, default_value: i64) -> Result<i64> {
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

fn parse_bool(key: &str, default_value: bool) -> Result<bool> {
  let Some(raw) = std::env::var(key).ok() else {
    return Ok(default_value);
  };

  match raw.trim().to_ascii_lowercase().as_str() {
    "1" | "true" | "yes" | "on" => Ok(true),
    "0" | "false" | "no" | "off" => Ok(false),
    _ => bail!("{key} must be a boolean value (true/false, 1/0, yes/no, on/off)"),
  }
}
