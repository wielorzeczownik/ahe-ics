mod calendar;
mod dto;
mod real_ip;
mod routes;
mod shared_routes;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;

pub use routes::router;
pub use shared_routes::shared_router;

/// Body returned for any failure that did not originate from a request the
/// caller can fix
const INTERNAL_ERROR_BODY: &str = "internal server error";

#[derive(Debug)]
pub struct AppError {
  status: StatusCode,
  message: String,
  detail: Option<String>,
}

impl AppError {
  pub fn bad_request(message: impl Into<String>) -> Self {
    Self {
      status: StatusCode::BAD_REQUEST,
      message: message.into(),
      detail: None,
    }
  }

  pub fn unauthorized(message: impl Into<String>) -> Self {
    Self {
      status: StatusCode::UNAUTHORIZED,
      message: message.into(),
      detail: None,
    }
  }
}

impl From<anyhow::Error> for AppError {
  fn from(err: anyhow::Error) -> Self {
    // Errors from the WPS layer
    Self {
      status: StatusCode::INTERNAL_SERVER_ERROR,
      message: INTERNAL_ERROR_BODY.to_string(),
      detail: Some(format!("{err:#}")),
    }
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    if let Some(detail) = self.detail {
      error!(status = %self.status, detail, "request failed");
    }

    (self.status, self.message).into_response()
  }
}

#[cfg(test)]
mod tests {
  use axum::body::to_bytes;

  use super::*;

  async fn render(error: AppError) -> (StatusCode, String) {
    let response = error.into_response();
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX)
      .await
      .expect("body collects");

    (
      status,
      String::from_utf8(body.to_vec()).expect("utf-8 body"),
    )
  }

  #[tokio::test]
  async fn upstream_detail_never_reaches_the_client() {
    let upstream = anyhow::anyhow!("login failed: 403 body={{\"user\":\"alice\",\"locked\":true}}");
    let (status, body) = render(AppError::from(upstream)).await;

    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body, INTERNAL_ERROR_BODY);
    assert!(!body.contains("alice"));
    assert!(!body.contains("403"));
    assert!(!body.contains("locked"));
  }

  #[tokio::test]
  async fn error_chain_context_is_not_leaked_either() {
    let upstream =
      anyhow::anyhow!("body={{\"pesel\":\"x\"}}").context("student data request failed");
    let (_, body) = render(AppError::from(upstream)).await;

    assert_eq!(body, INTERNAL_ERROR_BODY);
    assert!(!body.contains("pesel"));
  }

  #[tokio::test]
  async fn client_facing_messages_are_preserved() {
    // These are written by this service, so they are safe to return as-is
    let (status, body) = render(AppError::bad_request("to must be >= from")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, "to must be >= from");

    let (status, body) = render(AppError::unauthorized("invalid calendar token")).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body, "invalid calendar token");
  }
}
