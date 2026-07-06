use anyhow::{Result, anyhow, bail};
use argon2::password_hash::PasswordHash;
use argon2::{Argon2, PasswordVerifier};

#[derive(Clone, Copy, Debug)]
pub enum CalendarLanguage {
  Pl,
  En,
}

impl CalendarLanguage {
  pub(super) fn from_env_value(value: &str) -> Result<Self> {
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
  pub(super) fn from_env_value(value: &str) -> Result<Self> {
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
      return Self::from_argon2_hash(raw.trim());
    }

    if trimmed.starts_with("$argon2id$") {
      return Self::from_argon2_hash(trimmed);
    }

    Ok(Self::Plain(trimmed.to_string()))
  }

  pub(super) fn from_argon2_hash(hash: &str) -> Result<Self> {
    if hash.is_empty() {
      bail!("AHE_CAL_TOKEN Argon2id hash cannot be empty");
    }

    let parsed = PasswordHash::new(hash)
      .map_err(|_| anyhow!("AHE_CAL_TOKEN Argon2id hash is invalid PHC string"))?;

    if parsed.algorithm.as_str() != "argon2id" {
      bail!("AHE_CAL_TOKEN must use Argon2id (expected prefix '$argon2id$')");
    }

    Ok(Self::Argon2id(hash.to_string()))
  }

  #[must_use]
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
