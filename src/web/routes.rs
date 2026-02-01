use axum::Router;
use axum::extract::{ ConnectInfo, Query, State };
use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;
use axum::routing::get;
use chrono::{ Duration, NaiveDate };
use serde::Deserialize;
use std::net::SocketAddr;

use crate::app::AppState;
use crate::cache::IcsCacheKey;
use crate::constants::{ ICS_CONTENT_TYPE, JSON_CONTENT_TYPE };
use crate::ics::render_calendar;
use crate::web::AppError;
use crate::web::openapi;

#[derive(Debug, Deserialize)]
pub(crate) struct CalendarQuery {
  from: Option<NaiveDate>,
  to: Option<NaiveDate>,
  token: Option<String>,
}

/// Builds the HTTP router with calendar endpoints.
pub fn router(state: AppState) -> Router {
  Router::new()
    .route("/calendar.ics", get(calendar))
    .route("/calendar/me.ics", get(calendar_me))
    .route("/openapi.json", get(openapi_json))
    .fallback(not_found)
    .with_state(state)
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
  ConnectInfo(addr): ConnectInfo<SocketAddr>
) -> Result<impl IntoResponse, AppError> {
  let ics = calendar_core(state, query, headers, addr).await?;
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
  ConnectInfo(addr): ConnectInfo<SocketAddr>
) -> Result<impl IntoResponse, AppError> {
  let ics = calendar_core(state, query, headers, addr).await?;
  Ok(([(CONTENT_TYPE, ICS_CONTENT_TYPE)], ics))
}

async fn calendar_core(
  state: AppState,
  query: CalendarQuery,
  headers: HeaderMap,
  addr: SocketAddr
) -> Result<String, AppError> {
  tracing::info!(client_ip = %addr.ip(), "calendar request");
  if let Some(expected) = state.config.calendar_token.as_deref() {
    let provided = extract_token(&query, &headers);
    if provided.as_deref() != Some(expected) {
      return Err(AppError::unauthorized("invalid calendar token"));
    }
  }

  let today = chrono::Local::now().date_naive();
  let from = query.from.unwrap_or_else(|| today - Duration::days(state.config.calendar_past_days));
  let to = query.to.unwrap_or_else(|| today + Duration::days(state.config.calendar_future_days));

  if to < from {
    return Err(AppError::bad_request("to must be >= from"));
  }

  let token = state.token_cache.get_or_login(&state.config, &state.api).await?;

  let student_data = state.api.get_student_data(&token).await?;
  let student_id = student_data.id_student;

  let key = IcsCacheKey {
    student_id,
    from,
    to,
  };

  if let Some(cached) = state.ics_cache.get(&key).await {
    tracing::debug!("ics cache hit");
    return Ok(cached);
  }

  tracing::debug!("ics cache miss");
  let date_from = from.format("%Y-%m-%d").to_string();
  let date_to = to.format("%Y-%m-%d").to_string();
  let plan = state.api.get_plan(&token, student_id, &date_from, &date_to).await?;

  let ics = render_calendar(student_id, &plan, state.config.calendar_lang)?;

  state.ics_cache.insert(key, ics.clone()).await;

  Ok(ics)
}

fn extract_token(query: &CalendarQuery, headers: &HeaderMap) -> Option<String> {
  let header_token = headers
    .get("x-calendar-token")
    .and_then(|value| value.to_str().ok())
    .map(str::to_string);

  let bearer_token = headers
    .get("authorization")
    .and_then(|value| value.to_str().ok())
    .and_then(|value| value.strip_prefix("Bearer "))
    .map(str::to_string);

  query.token.clone().or(header_token).or(bearer_token)
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
