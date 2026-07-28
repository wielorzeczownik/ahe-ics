use anyhow::{Context, Result, bail};

use super::types::{CalendarLanguage, CalendarToken};

const DEFAULT_BIND_ADDR: &str = "0.0.0.0:8080";
const DEFAULT_CAL_PAST_DAYS: i64 = 60;
const DEFAULT_CAL_FUTURE_DAYS: i64 = 60;
const DEFAULT_CAL_LANG: &str = "pl";
const DEFAULT_EXAMS_ENABLED: bool = true;
const DEFAULT_JSON_ENABLED: bool = true;

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

  normalize_real_ip_header(raw.as_deref())
}

fn parse_days(key: &str, default_value: i64) -> Result<i64> {
  parse_days_value(key, std::env::var(key).ok().as_deref(), default_value)
}

fn parse_bool(key: &str, default_value: bool) -> Result<bool> {
  parse_bool_value(key, std::env::var(key).ok().as_deref(), default_value)
}

/// Normalizes the configured real-ip header name
fn normalize_real_ip_header(raw: Option<&str>) -> Result<Option<String>> {
  let Some(raw) = raw else {
    return Ok(None);
  };

  let value = raw.trim();
  if value.is_empty() {
    bail!("REAL_IP_HEADER cannot be empty");
  }

  Ok(Some(value.to_ascii_lowercase()))
}

fn parse_days_value(key: &str, raw: Option<&str>, default_value: i64) -> Result<i64> {
  let Some(raw) = raw else {
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

fn parse_bool_value(key: &str, raw: Option<&str>, default_value: bool) -> Result<bool> {
  let Some(raw) = raw else {
    return Ok(default_value);
  };

  match raw.trim().to_ascii_lowercase().as_str() {
    "1" | "true" | "yes" | "on" => Ok(true),
    "0" | "false" | "no" | "off" => Ok(false),
    _ => bail!("{key} must be a boolean value (true/false, 1/0, yes/no, on/off)"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const KEY: &str = "AHE_TEST_KEY";

  #[test]
  fn days_fall_back_to_default_when_unset() {
    assert_eq!(parse_days_value(KEY, None, 60).expect("default"), 60);
  }

  #[test]
  fn days_accept_non_negative_integers() {
    assert_eq!(parse_days_value(KEY, Some("0"), 60).expect("zero"), 0);
    assert_eq!(parse_days_value(KEY, Some("365"), 60).expect("value"), 365);
  }

  #[test]
  fn days_reject_negative_and_malformed_values() {
    assert!(parse_days_value(KEY, Some("-1"), 60).is_err());
    assert!(parse_days_value(KEY, Some("abc"), 60).is_err());
    assert!(parse_days_value(KEY, Some(""), 60).is_err());
    assert!(parse_days_value(KEY, Some("1.5"), 60).is_err());
    // Unlike the boolean parser
    assert!(parse_days_value(KEY, Some(" 30 "), 60).is_err());
  }

  #[test]
  fn days_error_message_names_the_key() {
    let error = parse_days_value(KEY, Some("-1"), 60).expect_err("negative is rejected");
    assert!(error.to_string().contains(KEY));
  }

  #[test]
  fn bool_falls_back_to_default_when_unset() {
    assert!(parse_bool_value(KEY, None, true).expect("default"));
    assert!(!parse_bool_value(KEY, None, false).expect("default"));
  }

  #[test]
  fn bool_accepts_documented_truthy_values() {
    for raw in ["1", "true", "TRUE", " yes ", "on"] {
      assert!(
        parse_bool_value(KEY, Some(raw), false).expect("parsed"),
        "expected {raw:?} to parse as true"
      );
    }
  }

  #[test]
  fn bool_accepts_documented_falsy_values() {
    for raw in ["0", "false", "FALSE", " no ", "off"] {
      assert!(
        !parse_bool_value(KEY, Some(raw), true).expect("parsed"),
        "expected {raw:?} to parse as false"
      );
    }
  }

  #[test]
  fn bool_rejects_anything_else() {
    for raw in ["", "maybe", "2", "tak"] {
      assert!(
        parse_bool_value(KEY, Some(raw), true).is_err(),
        "expected {raw:?} to be rejected"
      );
    }
  }

  #[test]
  fn real_ip_header_is_lowercased() {
    assert_eq!(
      normalize_real_ip_header(Some("  X-Forwarded-For  ")).expect("valid header"),
      Some("x-forwarded-for".to_string())
    );
  }

  #[test]
  fn real_ip_header_is_optional_but_never_blank() {
    assert_eq!(normalize_real_ip_header(None).expect("unset"), None);
    assert!(normalize_real_ip_header(Some("   ")).is_err());
    assert!(normalize_real_ip_header(Some("")).is_err());
  }
}
