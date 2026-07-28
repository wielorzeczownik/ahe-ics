use std::fmt;

use anyhow::{Result, anyhow, bail};
use argon2::password_hash::PasswordHash;
use argon2::{Argon2, PasswordVerifier};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

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

#[derive(Clone)]
pub enum CalendarToken {
  Plain(String),
  Argon2id(String),
}

/// Keeps the configured mode visible while never printing the token itself.
impl fmt::Debug for CalendarToken {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    let variant = match self {
      Self::Plain(_) => "Plain",
      Self::Argon2id(_) => "Argon2id",
    };
    write!(formatter, "CalendarToken::{variant}(<redacted>)")
  }
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

    // PHC makes the digest optional, so a hash truncated at the salt still parses.
    // Reject it here, otherwise every request would fail verification at runtime
    if parsed.hash.is_none() {
      bail!("AHE_CAL_TOKEN Argon2id hash is missing its digest; copy the full PHC string");
    }

    Ok(Self::Argon2id(hash.to_string()))
  }

  #[must_use]
  pub fn verify(&self, provided: &str) -> bool {
    match self {
      Self::Plain(expected) => {
        let provided = Sha256::digest(provided.as_bytes());
        let expected = Sha256::digest(expected.as_bytes());
        provided[..].ct_eq(&expected[..]).into()
      }
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

#[cfg(test)]
mod tests {
  use argon2::password_hash::{PasswordHasher, SaltString};
  use argon2::{Algorithm, Params, Version};

  use super::*;

  /// Deterministic salt keeps the generated PHC strings stable across runs
  const TEST_SALT: &str = "dGVzdHNhbHR0ZXN0c2FsdA";

  fn hash_with(algorithm: Algorithm, password: &str) -> String {
    let salt = SaltString::from_b64(TEST_SALT).expect("valid salt");
    Argon2::new(algorithm, Version::V0x13, Params::default())
      .hash_password(password.as_bytes(), &salt)
      .expect("hashing succeeds")
      .to_string()
  }

  #[test]
  fn language_accepts_supported_values() {
    assert!(matches!(
      CalendarLanguage::from_env_value("  PL  "),
      Ok(CalendarLanguage::Pl)
    ));
    assert!(matches!(
      CalendarLanguage::from_env_value("en"),
      Ok(CalendarLanguage::En)
    ));
  }

  #[test]
  fn language_rejects_unknown_values() {
    assert!(CalendarLanguage::from_env_value("de").is_err());
    assert!(CalendarLanguage::from_env_value("").is_err());
  }

  #[test]
  fn token_defaults_to_plain() {
    let token = CalendarToken::from_env_value("  s3cret  ").expect("valid token");

    assert!(matches!(token, CalendarToken::Plain(ref value) if value == "s3cret"));
    assert!(token.verify("s3cret"));
    assert!(!token.verify("other"));
    // Surrounding whitespace was stripped, so the padded form must not verify
    assert!(!token.verify("  s3cret  "));
  }

  #[test]
  fn token_honours_explicit_plain_prefix() {
    // Without the prefix this would be sniffed as a hash
    let token = CalendarToken::from_env_value("plain: $argon2id$looking").expect("valid token");

    assert!(matches!(token, CalendarToken::Plain(_)));
    assert!(token.verify("$argon2id$looking"));
  }

  #[test]
  fn token_rejects_empty_forms() {
    assert!(CalendarToken::from_env_value("").is_err());
    assert!(CalendarToken::from_env_value("   ").is_err());
    assert!(CalendarToken::from_env_value("plain:   ").is_err());
    assert!(CalendarToken::from_env_value("argon2:").is_err());
  }

  #[test]
  fn token_detects_bare_argon2id_hash() {
    let hash = hash_with(Algorithm::Argon2id, "s3cret");
    let token = CalendarToken::from_env_value(&hash).expect("valid token");

    assert!(matches!(token, CalendarToken::Argon2id(_)));
    assert!(token.verify("s3cret"));
    assert!(!token.verify("wrong"));
    // The hash itself is not a valid password
    assert!(!token.verify(&hash));
  }

  #[test]
  fn token_accepts_prefixed_argon2id_hash() {
    let hash = hash_with(Algorithm::Argon2id, "s3cret");
    let token = CalendarToken::from_env_value(&format!("argon2: {hash}")).expect("valid token");

    assert!(matches!(token, CalendarToken::Argon2id(_)));
    assert!(token.verify("s3cret"));
  }

  #[test]
  fn token_rejects_weaker_argon2_variants() {
    for algorithm in [Algorithm::Argon2i, Algorithm::Argon2d] {
      let hash = hash_with(algorithm, "s3cret");
      assert!(
        CalendarToken::from_env_value(&format!("argon2:{hash}")).is_err(),
        "expected {algorithm:?} to be rejected"
      );
    }
  }

  #[test]
  fn token_rejects_malformed_argon2_hash() {
    assert!(CalendarToken::from_env_value("argon2:not-a-phc-string").is_err());
    assert!(CalendarToken::from_env_value("argon2:$argon2i$broken").is_err());
  }

  #[test]
  fn token_rejects_hash_without_digest() {
    // A PHC string carrying only the salt parses fine but can never verify,
    // so it has to be caught at startup rather than on every request
    let salt_only = "$argon2id$v=19$m=19456,t=2,p=1$dGVzdHNhbHR0ZXN0c2FsdA";

    assert!(CalendarToken::from_env_value(salt_only).is_err());
    assert!(CalendarToken::from_env_value(&format!("argon2:{salt_only}")).is_err());
    assert!(CalendarToken::from_env_value("argon2:$argon2id$broken").is_err());
  }

  #[test]
  fn debug_never_prints_the_token() {
    let plain = CalendarToken::from_env_value("s3cret").expect("valid token");
    let rendered = format!("{plain:?}");

    assert!(!rendered.contains("s3cret"), "leaked: {rendered}");
    assert!(
      rendered.contains("Plain"),
      "mode should stay visible: {rendered}"
    );

    let hash = hash_with(Algorithm::Argon2id, "s3cret");
    let argon = CalendarToken::from_env_value(&hash).expect("valid token");
    let rendered = format!("{argon:?}");

    assert!(!rendered.contains(&hash), "leaked: {rendered}");
    assert!(
      rendered.contains("Argon2id"),
      "mode should stay visible: {rendered}"
    );
  }

  #[test]
  fn token_accepts_digest_of_unexpected_length() {
    let hash = hash_with(Algorithm::Argon2id, "s3cret");
    let truncated = &hash[..hash.len() - 8];

    let token = CalendarToken::from_env_value(truncated).expect("parses");
    assert!(!token.verify("s3cret"));
  }
}
