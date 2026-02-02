use axum::extract::{ConnectInfo, Query, State};
use axum::http::HeaderMap;
use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;
use chrono::NaiveDate;
use serde::Deserialize;
use std::net::SocketAddr;

use crate::app::AppState;
use crate::constants::{ICS_CONTENT_TYPE, JSON_CONTENT_TYPE};
use crate::web::AppError;
use crate::web::calendar_service::{CalendarQueryParams, fetch_calendar_data, render_calendar_ics};
use crate::web::dto::calendar::CalendarJsonResponse;

#[derive(Debug, Deserialize)]
pub(crate) struct CalendarQuery {
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
pub(crate) async fn calendar(
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
    path = "/calendar/me.ics",
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
pub(crate) async fn calendar_me(
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
pub(crate) async fn calendar_json(
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
    path = "/calendar/me.json",
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
pub(crate) async fn calendar_me_json(
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
