mod dedicated;
mod parse;
mod shared;
mod types;

pub use dedicated::Config;
pub use shared::SharedConfig;
pub use types::{CalendarLanguage, CalendarToken};

/// Shared server-level settings used by both dedicated and shared binaries.
pub trait ServerSettings: Clone + Send + Sync + 'static {
  fn calendar_past_days(&self) -> i64;
  fn calendar_future_days(&self) -> i64;
  fn calendar_token(&self) -> Option<&CalendarToken>;
  fn calendar_lang(&self) -> CalendarLanguage;
  fn exams_enabled(&self) -> bool;
  fn json_enabled(&self) -> bool;
  fn real_ip_header(&self) -> Option<&str>;
}
