mod ics;
mod student;
mod token;

pub use ics::{IcsCache, IcsCacheKey};
pub use student::{StudentContext, StudentContextCache};
pub use token::{TokenCache, TokenCacheEntry};

use sha2::{Digest, Sha256};

/// Opaque cache key derived from a full credential pair
pub(crate) type CredentialKey = [u8; 32];

/// Binds a cache entry to the exact credentials that produced it
pub(crate) fn credential_key(username: &str, password: &str) -> CredentialKey {
  let mut hasher = Sha256::new();
  // The separator keeps ("ab", "c") from colliding with ("a", "bc").
  hasher.update(username.as_bytes());
  hasher.update([0u8]);
  hasher.update(password.as_bytes());

  hasher.finalize().into()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn same_credentials_produce_the_same_key() {
    assert_eq!(
      credential_key("alice", "s3cret"),
      credential_key("alice", "s3cret")
    );
  }

  #[test]
  fn a_wrong_password_produces_a_different_key() {
    assert_ne!(
      credential_key("alice", "s3cret"),
      credential_key("alice", "wrong")
    );
    assert_ne!(
      credential_key("alice", "s3cret"),
      credential_key("alice", "")
    );
  }

  #[test]
  fn different_users_produce_different_keys() {
    assert_ne!(
      credential_key("alice", "s3cret"),
      credential_key("bob", "s3cret")
    );
  }

  #[test]
  fn the_separator_prevents_field_boundary_collisions() {
    assert_ne!(credential_key("ab", "c"), credential_key("a", "bc"));
    assert_ne!(credential_key("a", ""), credential_key("", "a"));
  }

  #[test]
  fn keys_do_not_embed_the_credentials() {
    let key = credential_key("alice", "s3cret");

    assert_eq!(key.len(), 32);
    assert!(!key.windows(5).any(|window| window == b"alice"));
  }
}
