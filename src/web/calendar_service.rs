use axum::http::HeaderMap;
use chrono::{Duration, NaiveDate};
use std::net::SocketAddr;

use crate::app::AppState;
use crate::cache::IcsCacheKey;
use crate::ics::render_calendar;
use crate::models::{ExamEvent, PlanItem, StudentIndex};
use crate::web::AppError;

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

pub(crate) async fn render_calendar_ics(
  state: AppState,
  query: CalendarQueryParams,
  headers: HeaderMap,
  addr: SocketAddr,
) -> Result<String, AppError> {
  let context = prepare_calendar_request_context(&state, query, headers, addr).await?;

  let key = IcsCacheKey {
    student_id: context.student_id,
    from: context.from,
    to: context.to,
  };

  if let Some(cached) = state.ics_cache.get(&key).await {
    tracing::debug!("ics cache hit");
    return Ok(cached);
  }

  tracing::debug!("ics cache miss");
  let data = fetch_calendar_render_data(&state, &context).await?;
  let ics = render_calendar(
    data.student_id,
    &data.plan,
    &data.exams,
    state.config.calendar_lang,
  )?;

  state.ics_cache.insert(key, ics.clone()).await;

  Ok(ics)
}

pub(crate) async fn fetch_calendar_data(
  state: AppState,
  query: CalendarQueryParams,
  headers: HeaderMap,
  addr: SocketAddr,
) -> Result<CalendarRenderData, AppError> {
  let context = prepare_calendar_request_context(&state, query, headers, addr).await?;
  fetch_calendar_render_data(&state, &context).await
}

async fn prepare_calendar_request_context(
  state: &AppState,
  query: CalendarQueryParams,
  headers: HeaderMap,
  addr: SocketAddr,
) -> Result<CalendarRequestContext, AppError> {
  tracing::info!(client_ip = %addr.ip(), "calendar request");
  if let Some(expected) = state.config.calendar_token.as_deref() {
    let provided = extract_token(&query, &headers);
    if provided.as_deref() != Some(expected) {
      return Err(AppError::unauthorized("invalid calendar token"));
    }
  }

  let today = chrono::Local::now().date_naive();
  let from = query
    .from
    .unwrap_or_else(|| today - Duration::days(state.config.calendar_past_days));
  let to = query
    .to
    .unwrap_or_else(|| today + Duration::days(state.config.calendar_future_days));

  if to < from {
    return Err(AppError::bad_request("to must be >= from"));
  }

  let token = state
    .token_cache
    .get_or_login(&state.config, &state.api)
    .await?;

  let student_data = state.api.get_student_data(&token).await?;
  let student_id = student_data.student_id;
  // When exams are disabled, skip index resolution entirely and render only schedule entries.
  let mut index_id = if state.config.exams_enabled {
    student_data.index_id
  } else {
    None
  };

  if state.config.exams_enabled && index_id.is_none() {
    match state.api.get_student_indexes(&token).await {
      Ok(indexes) => {
        index_id = pick_index_id(&indexes);
        if let Some(found) = index_id {
          tracing::debug!(
            student_id,
            index_id = found,
            "IndeksID resolved from indeks list"
          );
        } else {
          tracing::warn!(student_id, "student indeks list is empty, skipping exams");
        }
      }
      Err(error) => {
        tracing::warn!(student_id, error = %error, "failed to fetch indeks list, skipping exams");
      }
    }
  }

  Ok(CalendarRequestContext {
    token,
    student_id,
    index_id,
    from,
    to,
  })
}

async fn fetch_calendar_render_data(
  state: &AppState,
  context: &CalendarRequestContext,
) -> Result<CalendarRenderData, AppError> {
  let date_from = context.from.format("%Y-%m-%d").to_string();
  let date_to = context.to.format("%Y-%m-%d").to_string();
  let plan = state
    .api
    .get_plan(&context.token, context.student_id, &date_from, &date_to)
    .await?;
  let exams = if state.config.exams_enabled {
    if let Some(index_id) = context.index_id {
      match state
        .api
        .get_exams(&context.token, index_id, context.from, context.to)
        .await
      {
        Ok(items) => items,
        Err(error) => {
          tracing::warn!(
            context.student_id,
            error = %error,
            "failed to fetch exams, continuing with schedule only"
          );
          Vec::new()
        }
      }
    } else {
      tracing::warn!(
        context.student_id,
        "IndeksID not found in student data, skipping exams"
      );
      Vec::new()
    }
  } else {
    tracing::info!("exam fetching disabled by AHE_CAL_EXAMS_ENABLED");
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

fn pick_index_id(indexes: &[StudentIndex]) -> Option<i64> {
  indexes
    .iter()
    .max_by_key(|item| {
      (
        item.status_symbol.as_deref() == Some("S"),
        item.year.unwrap_or_default(),
        item.semester.unwrap_or_default(),
        item.index_id,
      )
    })
    .map(|item| item.index_id)
}
