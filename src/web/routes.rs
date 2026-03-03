use std::net::SocketAddr;

use axum::Router;
use axum::extract::{ConnectInfo, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;
use axum::routing::get;
use chrono::NaiveDate;
use serde::Deserialize;
use tracing::warn;

use crate::app::AppState;
use crate::constants::{ICS_CONTENT_TYPE, JSON_CONTENT_TYPE};
use crate::web::AppError;
use crate::web::calendar::{CalendarQueryParams, fetch_calendar_data, render_calendar_ics};
use crate::web::dto::CalendarJsonResponse;
use crate::web::openapi;

#[derive(Debug, Deserialize)]
struct CalendarQuery {
  from: Option<NaiveDate>,
  to: Option<NaiveDate>,
  token: Option<String>,
}

impl From<CalendarQuery> for CalendarQueryParams {
  fn from(value: CalendarQuery) -> Self {
    Self {
      from: value.from,
      to: value.to,
      token: value.token,
    }
  }
}

/// Builds the HTTP router with calendar endpoints.
pub fn router(state: AppState) -> Router {
  let mut router = Router::new()
    .route("/calendar.ics", get(calendar_ics))
    .route("/calendar/me.ics", get(calendar_ics))
    .route("/healthz", get(healthz))
    .fallback(not_found);

  if state.config.json_enabled {
    router = router
      .route("/calendar.json", get(calendar_json))
      .route("/calendar/me.json", get(calendar_json));
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
  path = "/calendar.ics",
  tag = "calendar",
  params(
    ("from" = Option<String>, Query, description = "Start date in YYYY-MM-DD"),
    ("to" = Option<String>, Query, description = "End date in YYYY-MM-DD"),
    ("token" = Option<String>, Query, description = "Calendar token when token protection is enabled")
  ),
  responses(
    (status = 200, description = "ICS calendar feed", body = String, content_type = "text/calendar"),
    (status = 400, description = "Invalid query parameters", body = String, content_type = "text/plain"),
    (status = 401, description = "Invalid token", body = String, content_type = "text/plain"),
    (status = 500, description = "Internal server error", body = String, content_type = "text/plain")
  ),
  security(
    ("calendarTokenQuery" = []),
    ("calendarTokenHeader" = []),
    ("calendarTokenBearer" = [])
  )
)]
async fn calendar_ics(
  State(state): State<AppState>,
  Query(query): Query<CalendarQuery>,
  headers: HeaderMap,
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<impl IntoResponse, AppError> {
  let ics = render_calendar_ics(state, query.into(), headers, addr).await?;
  Ok(([(CONTENT_TYPE, ICS_CONTENT_TYPE)], ics))
}

#[utoipa::path(
  get,
  path = "/calendar.json",
  tag = "calendar",
  params(
    ("from" = Option<String>, Query, description = "Start date in YYYY-MM-DD"),
    ("to" = Option<String>, Query, description = "End date in YYYY-MM-DD"),
    ("token" = Option<String>, Query, description = "Calendar token when token protection is enabled")
  ),
  responses(
    (status = 200, description = "Calendar source data used for ICS rendering", body = CalendarJsonResponse, content_type = "application/json"),
    (status = 400, description = "Invalid query parameters", body = String, content_type = "text/plain"),
    (status = 401, description = "Invalid token", body = String, content_type = "text/plain"),
    (status = 500, description = "Internal server error", body = String, content_type = "text/plain")
  ),
  security(
    ("calendarTokenQuery" = []),
    ("calendarTokenHeader" = []),
    ("calendarTokenBearer" = [])
  )
)]
async fn calendar_json(
  State(state): State<AppState>,
  Query(query): Query<CalendarQuery>,
  headers: HeaderMap,
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<impl IntoResponse, AppError> {
  let data = fetch_calendar_data(state, query.into(), headers, addr).await?;
  let body = serde_json::to_vec(&CalendarJsonResponse::from_parts(
    data.student_id,
    data.from,
    data.to,
    data.plan,
    data.exams,
  ))
  .map_err(anyhow::Error::from)?;
  Ok(([(CONTENT_TYPE, JSON_CONTENT_TYPE)], body))
}

#[utoipa::path(
  get,
  path = "/healthz",
  tag = "health",
  responses(
    (status = 204, description = "Service is healthy"),
    (status = 503, description = "Upstream API unavailable", body = String, content_type = "text/plain")
  )
)]
async fn healthz(State(state): State<AppState>) -> impl IntoResponse {
  let token = match state
    .token_cache
    .get_or_login(&state.config, &state.api)
    .await
  {
    Ok(token) => token,
    Err(error) => {
      warn!(error = %error, "health check login failed");
      return (StatusCode::SERVICE_UNAVAILABLE, "upstream login failed");
    }
  };

  if let Err(error) = state.api.get_student_data(&token).await {
    warn!(error = %error, "health check student data failed");
    return (StatusCode::SERVICE_UNAVAILABLE, "upstream api unavailable");
  }

  (StatusCode::NO_CONTENT, "")
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
async fn openapi_json(
  State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
  let body = openapi::spec_json(state.config.json_enabled)?;
  Ok(([(CONTENT_TYPE, JSON_CONTENT_TYPE)], body))
}
