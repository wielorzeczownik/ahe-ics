pub(crate) mod calendar;

use axum::Router;
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;
use axum::routing::get;

use crate::app::AppState;
use crate::constants::JSON_CONTENT_TYPE;
use crate::web::AppError;
use crate::web::openapi;

pub(crate) use self::calendar::{calendar, calendar_json, calendar_me, calendar_me_json};

/// Builds the HTTP router with calendar endpoints.
pub fn router(state: AppState) -> Router {
  Router::new()
    .route("/calendar.ics", get(calendar))
    .route("/calendar/me.ics", get(calendar_me))
    .route("/calendar.json", get(calendar_json))
    .route("/calendar/me.json", get(calendar_me_json))
    .route("/openapi.json", get(openapi_json))
    .fallback(not_found)
    .with_state(state)
}

async fn not_found() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "Not Found")
}

#[utoipa::path(
  get,
  path = "/openapi.json",
  tag = "calendar",
  responses((
    status = 200,
    description = "OpenAPI specification",
    body = String,
    content_type = "application/json",
  ))
)]
pub(crate) async fn openapi_json() -> Result<impl IntoResponse, AppError> {
  let body = openapi::spec_json()?;
  Ok(([(CONTENT_TYPE, JSON_CONTENT_TYPE)], body))
}
