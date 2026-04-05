use std::net::SocketAddr;

use axum::Router;
use axum::extract::{ConnectInfo, Query, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, StatusCode};
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

  router.with_state(state)
}

async fn not_found() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "Not Found")
}

async fn calendar_ics(
  State(state): State<AppState>,
  Query(query): Query<CalendarQuery>,
  headers: HeaderMap,
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<impl IntoResponse, AppError> {
  let ics = render_calendar_ics(state, query.into(), headers, addr).await?;
  Ok(([(CONTENT_TYPE, ICS_CONTENT_TYPE)], ics))
}

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
