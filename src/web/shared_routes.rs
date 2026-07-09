use std::net::SocketAddr;

use axum::Router;
use axum::extract::{ConnectInfo, Query, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use chrono::NaiveDate;
use serde::Deserialize;

use crate::app::AppState;
use crate::config::SharedConfig;
use crate::web::AppError;
use crate::web::calendar::{CalendarQueryParams, fetch_calendar_data, render_calendar_ics};
use crate::web::dto::CalendarJsonResponse;

/// Query params for the shared binary
#[derive(Debug, Deserialize)]
struct CalendarQuery {
  username: String,
  password: String,
  from: Option<NaiveDate>,
  to: Option<NaiveDate>,
  token: Option<String>,
}

/// Builds the HTTP router for the shared binary
pub fn shared_router(state: AppState<SharedConfig>) -> Router {
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
  State(state): State<AppState<SharedConfig>>,
  Query(query): Query<CalendarQuery>,
  headers: HeaderMap,
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<impl IntoResponse, AppError> {
  let username = query.username.clone();
  let password = query.password.clone();
  let params = CalendarQueryParams {
    from: query.from,
    to: query.to,
    token: query.token,
  };
  let ics = render_calendar_ics(state, &username, &password, params, headers, addr).await?;
  Ok(([(CONTENT_TYPE, "text/calendar; charset=utf-8")], ics))
}

async fn calendar_json(
  State(state): State<AppState<SharedConfig>>,
  Query(query): Query<CalendarQuery>,
  headers: HeaderMap,
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<impl IntoResponse, AppError> {
  let username = query.username.clone();
  let password = query.password.clone();
  let params = CalendarQueryParams {
    from: query.from,
    to: query.to,
    token: query.token,
  };
  let data = fetch_calendar_data(state, &username, &password, params, headers, addr).await?;
  let body = serde_json::to_vec(&CalendarJsonResponse::from_parts(
    data.student_id,
    data.from,
    data.to,
    data.plan,
    data.exams,
  ))
  .map_err(anyhow::Error::from)?;
  Ok(([(CONTENT_TYPE, "application/json; charset=utf-8")], body))
}

/// Shared instance healthz
async fn healthz(_state: State<AppState<SharedConfig>>) -> impl IntoResponse {
  StatusCode::NO_CONTENT
}
