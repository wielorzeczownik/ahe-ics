pub(crate) mod calendar;
pub(crate) mod health;

use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;
use axum::routing::get;

use crate::app::AppState;
use crate::constants::JSON_CONTENT_TYPE;
use crate::web::AppError;
use crate::web::openapi;

pub(crate) use self::calendar::{calendar, calendar_json, calendar_me, calendar_me_json};
pub(crate) use self::health::healthz;

/// Builds the HTTP router with calendar endpoints.
pub fn router(state: AppState) -> Router {
  let mut router = Router::new()
    .route("/calendar.ics", get(calendar))
    .route("/calendar/me.ics", get(calendar_me))
    .route("/healthz", get(healthz))
    .fallback(not_found);

  if state.config.json_enabled {
    router = router
      .route("/calendar.json", get(calendar_json))
      .route("/calendar/me.json", get(calendar_me_json));
  }

  if state.config.openapi_enabled {
    router = router.route("/openapi.json", get(openapi_json));
  }

  router.with_state(state)
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
pub(crate) async fn openapi_json(
  State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
  let body = openapi::spec_json(state.config.json_enabled)?;
  Ok(([(CONTENT_TYPE, JSON_CONTENT_TYPE)], body))
}
