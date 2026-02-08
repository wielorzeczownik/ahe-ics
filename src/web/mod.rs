mod calendar;
pub(crate) mod dto;
pub(crate) mod openapi;
pub(crate) mod real_ip;
mod routes;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub use routes::router;

#[derive(Debug)]
pub struct AppError {
  status: StatusCode,
  message: String,
}

impl AppError {
  pub fn bad_request(message: impl Into<String>) -> Self {
    Self {
      status: StatusCode::BAD_REQUEST,
      message: message.into(),
    }
  }

  pub fn unauthorized(message: impl Into<String>) -> Self {
    Self {
      status: StatusCode::UNAUTHORIZED,
      message: message.into(),
    }
  }
}

impl From<anyhow::Error> for AppError {
  fn from(err: anyhow::Error) -> Self {
    Self {
      status: StatusCode::INTERNAL_SERVER_ERROR,
      message: err.to_string(),
    }
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    (self.status, self.message).into_response()
  }
}
