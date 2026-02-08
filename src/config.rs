use anyhow::{Context, Result, anyhow, bail};
use argon2::password_hash::PasswordHash;
use argon2::{Argon2, PasswordVerifier};

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
pub enum CalendarToken {
  Plain(String),
  Argon2id(String),
}

impl CalendarToken {
  fn from_env_value(value: &str) -> Result<Self> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
      bail!("AHE_CAL_TOKEN cannot be empty");
    }

    if let Some(raw) = trimmed.strip_prefix("plain:") {
      let token = raw.trim();
      if token.is_empty() {
        bail!("AHE_CAL_TOKEN plain token cannot be empty");
      }
      return Ok(Self::Plain(token.to_string()));
    }

    if let Some(raw) = trimmed.strip_prefix("argon2:") {
      return parse_argon2_calendar_token(raw.trim());
    }

    if looks_like_argon2id_hash(trimmed) {
      return parse_argon2_calendar_token(trimmed);
    }

    Ok(Self::Plain(trimmed.to_string()))
  }

  pub fn verify(&self, provided: &str) -> bool {
    match self {
      Self::Plain(expected) => provided == expected,
      Self::Argon2id(hash) => {
        let Ok(parsed) = PasswordHash::new(hash) else {
          return false;
        };
        Argon2::default()
          .verify_password(provided.as_bytes(), &parsed)
          .is_ok()
      }
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
  pub calendar_token: Option<CalendarToken>,
  pub calendar_lang: CalendarLanguage,
  pub exams_enabled: bool,
  pub json_enabled: bool,
  pub openapi_enabled: bool,
  pub real_ip_header: Option<String>,
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
    let calendar_token = parse_calendar_token_env("AHE_CAL_TOKEN")?;
    let calendar_lang = parse_lang_env("AHE_CAL_LANG", DEFAULT_CAL_LANG)?;
    let exams_enabled = parse_bool_env("AHE_CAL_EXAMS_ENABLED", DEFAULT_EXAMS_ENABLED)?;
    let json_enabled = parse_bool_env("AHE_CAL_JSON_ENABLED", DEFAULT_JSON_ENABLED)?;
    let openapi_enabled = parse_bool_env("AHE_OPENAPI_ENABLED", DEFAULT_OPENAPI_ENABLED)?;
    let real_ip_header = parse_real_ip_header_env()?;

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
      real_ip_header,
    })
  }
}

fn parse_calendar_token_env(key: &str) -> Result<Option<CalendarToken>> {
  let Some(raw) = std::env::var(key).ok() else {
    return Ok(None);
  };

  let token = CalendarToken::from_env_value(raw.trim())
    .with_context(|| format!("{key} is invalid; provide plain token or Argon2id hash"))?;

  Ok(Some(token))
}

fn parse_argon2_calendar_token(hash: &str) -> Result<CalendarToken> {
  if hash.is_empty() {
    bail!("AHE_CAL_TOKEN Argon2id hash cannot be empty");
  }

  let parsed = PasswordHash::new(hash)
    .map_err(|_| anyhow!("AHE_CAL_TOKEN Argon2id hash is invalid PHC string"))?;

  if parsed.algorithm.as_str() != "argon2id" {
    bail!("AHE_CAL_TOKEN must use Argon2id (expected prefix '$argon2id$')");
  }

  Ok(CalendarToken::Argon2id(hash.to_string()))
}

fn looks_like_argon2id_hash(value: &str) -> bool {
  value.starts_with("$argon2id$")
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

fn parse_real_ip_header_env() -> Result<Option<String>> {
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
