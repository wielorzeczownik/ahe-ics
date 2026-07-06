use std::net::SocketAddr;

use axum::http::HeaderMap;
use chrono::{Duration, NaiveDate};
use tracing::{debug, info, warn};

use crate::app::AppState;
use crate::cache::IcsCacheKey;
use crate::config::ServerSettings;
use crate::ics::render_calendar;
use crate::models::{ExamEvent, PlanItem};
use crate::web::AppError;
use crate::web::real_ip::resolve_client_ip;

#[derive(Debug)]
pub(crate) struct CalendarQueryParams {
  pub(crate) from: Option<NaiveDate>,
  pub(crate) to: Option<NaiveDate>,
  pub(crate) token: Option<String>,
}

#[derive(Debug)]
pub(crate) struct CalendarRenderData {
  pub(crate) student_id: i64,
  pub(crate) from: NaiveDate,
  pub(crate) to: NaiveDate,
  pub(crate) plan: Vec<PlanItem>,
  pub(crate) exams: Vec<ExamEvent>,
}

#[derive(Debug)]
struct CalendarRequestContext {
  token: String,
  student_id: i64,
  index_id: Option<i64>,
  from: NaiveDate,
  to: NaiveDate,
}

pub(crate) async fn render_calendar_ics<C: ServerSettings>(
  state: AppState<C>,
  username: &str,
  password: &str,
  query: CalendarQueryParams,
  headers: HeaderMap,
  addr: SocketAddr,
) -> Result<String, AppError> {
  let context =
    prepare_calendar_request_context(&state, username, password, query, headers, addr).await?;

  let key = IcsCacheKey {
    student_id: context.student_id,
    from: context.from,
    to: context.to,
  };

  if let Some(cached) = state.ics_cache.get(&key).await {
    debug!("ics cache hit");
    return Ok(cached);
  }

  debug!("ics cache miss");
  let data = fetch_calendar_render_data(&state, &context).await?;
  let ics = render_calendar(
    data.student_id,
    &data.plan,
    &data.exams,
    state.config.calendar_lang(),
  )?;

  state.ics_cache.insert(key, ics.clone()).await;

  Ok(ics)
}

pub(crate) async fn fetch_calendar_data<C: ServerSettings>(
  state: AppState<C>,
  username: &str,
  password: &str,
  query: CalendarQueryParams,
  headers: HeaderMap,
  addr: SocketAddr,
) -> Result<CalendarRenderData, AppError> {
  let context =
    prepare_calendar_request_context(&state, username, password, query, headers, addr).await?;
  fetch_calendar_render_data(&state, &context).await
}

async fn prepare_calendar_request_context<C: ServerSettings>(
  state: &AppState<C>,
  username: &str,
  password: &str,
  query: CalendarQueryParams,
  headers: HeaderMap,
  addr: SocketAddr,
) -> Result<CalendarRequestContext, AppError> {
  let peer_ip = addr.ip();
  let resolved_ip = resolve_client_ip(peer_ip, &headers, state.config.real_ip_header());
  if matches!(
    resolved_ip.source,
    crate::web::real_ip::ClientIpSource::HeaderInvalid
  ) {
    warn!(peer_ip = %peer_ip, ip_source = %resolved_ip.source, "real-ip header invalid, falling back to peer address");
  }
  info!(ip = %resolved_ip.ip, "calendar request");

  if let Some(expected) = state.config.calendar_token() {
    let provided = extract_token(&query, &headers);
    let is_valid = provided
      .as_deref()
      .is_some_and(|value| expected.verify(value));

    if !is_valid {
      warn!(ip = %resolved_ip.ip, "unauthorized: invalid calendar token");
      return Err(AppError::unauthorized("invalid calendar token"));
    }
  }

  let today = chrono::Local::now().date_naive();
  let from = query
    .from
    .unwrap_or_else(|| today - Duration::days(state.config.calendar_past_days()));
  let to = query
    .to
    .unwrap_or_else(|| today + Duration::days(state.config.calendar_future_days()));

  if to < from {
    return Err(AppError::bad_request("to must be >= from"));
  }

  let token = match state
    .token_cache
    .get_or_login(username, password, &state.api)
    .await
  {
    Ok(value) => value,
    Err(error) => {
      warn!(ip = %resolved_ip.ip, "WPS login failed");
      return Err(AppError::from(error));
    }
  };
  let student_context = state
    .student_context_cache
    .get_or_fetch(username, state.config.exams_enabled(), &state.api, &token)
    .await?;

  Ok(CalendarRequestContext {
    token,
    student_id: student_context.student_id,
    index_id: student_context.index_id,
    from,
    to,
  })
}

async fn fetch_calendar_render_data<C: ServerSettings>(
  state: &AppState<C>,
  context: &CalendarRequestContext,
) -> Result<CalendarRenderData, AppError> {
  let date_from = context.from.format("%Y-%m-%d").to_string();
  let date_to = context.to.format("%Y-%m-%d").to_string();
  let plan = state
    .api
    .get_plan(&context.token, context.student_id, &date_from, &date_to)
    .await?;
  let exams = if state.config.exams_enabled() {
    if let Some(index_id) = context.index_id {
      match state
        .api
        .get_exams(&context.token, index_id, context.from, context.to)
        .await
      {
        Ok(items) => items,
        Err(error) => {
          warn!(
            context.student_id,
            error = %error,
            "failed to fetch exams, continuing with schedule only"
          );
          Vec::new()
        }
      }
    } else {
      warn!(
        context.student_id,
        "IndeksID not found in student data, skipping exams"
      );
      Vec::new()
    }
  } else {
    info!("exam fetching disabled by AHE_CAL_EXAMS_ENABLED");
    Vec::new()
  };

  Ok(CalendarRenderData {
    student_id: context.student_id,
    from: context.from,
    to: context.to,
    plan,
    exams,
  })
}

fn extract_token(query: &CalendarQueryParams, headers: &HeaderMap) -> Option<String> {
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
