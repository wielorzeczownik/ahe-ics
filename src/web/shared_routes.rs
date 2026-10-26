use std::net::SocketAddr;
use std::num::NonZeroU32;
use std::sync::LazyLock;

use axum::extract::{ConnectInfo, Query, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Form, Router};
use chrono::NaiveDate;
use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use serde::Deserialize;
use tracing::warn;

use crate::app::AppState;
use crate::config::SharedConfig;
use crate::web::AppError;
use crate::web::calendar::{CalendarQueryParams, fetch_calendar_data, render_calendar_ics};
use crate::web::dto::CalendarJsonResponse;

/// Query params for the shared binary
#[derive(Debug, Deserialize)]
struct CalendarQuery {
  /// Encrypted credentials minted by `POST /link`
  #[serde(rename = "c")]
  link: String,
  from: Option<NaiveDate>,
  to: Option<NaiveDate>,
  token: Option<String>,
}

impl From<CalendarQuery> for CalendarQueryParams {
  fn from(query: CalendarQuery) -> Self {
    Self {
      from: query.from,
      to: query.to,
      token: query.token,
    }
  }
}

#[derive(Deserialize)]
struct LinkForm {
  username: String,
  password: String,
  token: Option<String>,
}

/// WPS logins `POST /link` may attempt per minute, across all clients.
const LINK_LOGINS_PER_MINUTE: NonZeroU32 = NonZeroU32::new(10).expect("non-zero");

/// One counter for everybody: behind Cloudflare every client shares a few
/// addresses and the real-ip header is client-controlled, so a per-IP limit
/// would be easy to dodge. Keeps the server from being a WPS password oracle.
static LINK_BRAKE: LazyLock<DefaultDirectRateLimiter> =
  LazyLock::new(|| RateLimiter::direct(Quota::per_minute(LINK_LOGINS_PER_MINUTE)));

/// Builds the HTTP router for the shared binary
pub fn shared_router(state: AppState<SharedConfig>) -> Router {
  let mut router = Router::new()
    .route("/calendar.ics", get(calendar_ics))
    .route("/calendar/me.ics", get(calendar_ics))
    .route("/link", post(create_link))
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
  let (username, password) = resolve_credentials(&state.config, &query)?;
  let ics = render_calendar_ics(state, &username, &password, query.into(), headers, addr).await?;
  Ok(([(CONTENT_TYPE, "text/calendar; charset=utf-8")], ics))
}

async fn calendar_json(
  State(state): State<AppState<SharedConfig>>,
  Query(query): Query<CalendarQuery>,
  headers: HeaderMap,
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<impl IntoResponse, AppError> {
  let (username, password) = resolve_credentials(&state.config, &query)?;
  let data = fetch_calendar_data(state, &username, &password, query.into(), headers, addr).await?;
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

/// Credentials sealed into the `c` param by `POST /link`
fn resolve_credentials(
  config: &SharedConfig,
  query: &CalendarQuery,
) -> Result<(String, String), AppError> {
  config
    .link_key
    .open(&query.link)
    .ok_or_else(|| AppError::unauthorized("invalid or expired link"))
}

/// Checks the credentials against WPS and returns a calendar path with them
/// sealed into `c`. Taken as a form body so they never land in a URL.
async fn create_link(
  State(state): State<AppState<SharedConfig>>,
  Form(form): Form<LinkForm>,
) -> Result<String, AppError> {
  let token = form.token.filter(|token| !token.is_empty());
  if let Some(expected) = &state.config.calendar_token
    && !token.as_deref().is_some_and(|token| expected.verify(token))
  {
    warn!("unauthorized: invalid calendar token on link creation");
    return Err(AppError::unauthorized("invalid calendar token"));
  }

  if form.username.is_empty() || form.password.is_empty() {
    return Err(AppError::bad_request("username and password are required"));
  }

  if LINK_BRAKE.check().is_err() {
    warn!("link creation rate limit hit");
    return Err(AppError::too_many_requests(
      "too many link requests, try again in a minute",
    ));
  }

  if let Err(error) = state
    .token_cache
    .get_or_login(&form.username, &form.password, &state.api)
    .await
  {
    // Transport errors (network, proxy, bad JSON) are not the user's fault
    if error.downcast_ref::<reqwest::Error>().is_some() {
      return Err(AppError::from(error));
    }
    warn!("WPS login failed on link creation");
    return Err(AppError::unauthorized(
      "WPS login failed, check username and password",
    ));
  }

  let sealed = state.config.link_key.seal(&form.username, &form.password)?;
  let mut params = vec![("c", sealed.as_str())];
  if state.config.calendar_token.is_some()
    && let Some(token) = &token
  {
    params.push(("token", token));
  }
  let query = serde_urlencoded::to_string(params).map_err(anyhow::Error::from)?;

  Ok(format!("/calendar.ics?{query}\n"))
}

/// Shared instance healthz
async fn healthz(_state: State<AppState<SharedConfig>>) -> impl IntoResponse {
  StatusCode::NO_CONTENT
}
