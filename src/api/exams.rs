use std::collections::{BTreeMap, HashMap, HashSet};

use anyhow::{Context, Result};
use chrono::{Duration, NaiveDate, NaiveTime};
use reqwest::Client;
use tracing::{debug, warn};

use crate::constants::{
  API_BASE_URL, API_CURRENT_ACADEMIC_YEAR_PATH, API_EXAM_FILTER_PATH,
  API_EXAM_PROTOCOL_INTERMEDIATE_PATH, API_EXAM_PROTOCOL_PATH, EXAM_SETTLEMENT_NAME,
};
use crate::models::{
  CurrentAcademicYearResponse, ExamEvent, ExamProtocolIntermediateItem, ExamProtocolItem,
  ExamScheduleItem, TermQuery,
};

#[derive(Debug, Clone, Default)]
struct TermSubjects {
  names: std::collections::BTreeSet<String>,
}

impl TermSubjects {
  fn is_empty(&self) -> bool {
    self.names.is_empty()
  }
}

pub async fn get_exams(
  client: &Client,
  access_token: &str,
  index_id: i64,
  from: NaiveDate,
  to: NaiveDate,
) -> Result<Vec<ExamEvent>> {
  let academic_year = get_current_academic_year(client, access_token).await?;
  let terms = build_terms_for_year(academic_year);

  let mut subjects_by_term: BTreeMap<TermQuery, TermSubjects> = BTreeMap::new();
  for term in terms {
    match get_exam_protocol(client, access_token, index_id, term).await {
      Ok(items) => {
        let subjects = resolve_exam_subjects_for_term(client, access_token, term, items).await;
        if !subjects.is_empty() {
          subjects_by_term.insert(term, subjects);
        }
      }
      Err(error) => {
        warn!(
          academic_year = term.academic_year,
          semester_id = term.semester_id,
          error = %error,
          "exam protocol fetch failed"
        );
      }
    }
  }

  if subjects_by_term.is_empty() {
    debug!(
      index_id,
      "no exam protocol subjects found for requested range"
    );
    return Ok(Vec::new());
  }

  let mut events = Vec::new();
  let mut seen = HashSet::new();

  for (term, subjects) in subjects_by_term {
    match get_exam_schedule(client, access_token, term).await {
      Ok(items) => {
        for item in items {
          let Some(normalized_subject) = normalize_subject(&item.exam_subject) else {
            continue;
          };
          if !subjects.names.contains(&normalized_subject) {
            continue;
          }

          let Some(event) = map_exam_event(item, from, to) else {
            continue;
          };

          let key = format!(
            "{}|{}|{}",
            event.published_data_id, event.starts, normalized_subject
          );
          if seen.insert(key) {
            events.push(event);
          }
        }
      }
      Err(error) => {
        warn!(
          academic_year = term.academic_year,
          semester_id = term.semester_id,
          error = %error,
          "exam schedule fetch failed"
        );
      }
    }
  }

  events.sort_by(|a, b| {
    a.starts
      .cmp(&b.starts)
      .then_with(|| a.subject.cmp(&b.subject))
  });
  Ok(events)
}

async fn get_current_academic_year(client: &Client, access_token: &str) -> Result<i32> {
  let url = format!("{API_BASE_URL}{API_CURRENT_ACADEMIC_YEAR_PATH}");

  debug!("GET {API_CURRENT_ACADEMIC_YEAR_PATH}");
  let resp = client
    .get(url)
    .bearer_auth(access_token)
    .send()
    .await
    .context("current academic year request failed")?;

  let status = resp.status();
  if !status.is_success() {
    let text = resp.text().await.unwrap_or_default();
    anyhow::bail!("current academic year failed: {status} body={text}");
  }

  let payload = resp
    .json::<CurrentAcademicYearResponse>()
    .await
    .context("invalid current academic year json")?;

  Ok(payload.academic_year)
}

async fn get_exam_protocol(
  client: &Client,
  access_token: &str,
  index_id: i64,
  term: TermQuery,
) -> Result<Vec<ExamProtocolItem>> {
  let url = format!(
    "{API_BASE_URL}{API_EXAM_PROTOCOL_PATH}?IndeksID={index_id}&RokAkad={}&SemestrID={}",
    term.academic_year, term.semester_id
  );

  debug!(
    index_id,
    academic_year = term.academic_year,
    semester_id = term.semester_id,
    "GET {API_EXAM_PROTOCOL_PATH}"
  );
  let resp = client
    .get(url)
    .bearer_auth(access_token)
    .send()
    .await
    .context("exam protocol request failed")?;

  let status = resp.status();
  if !status.is_success() {
    let text = resp.text().await.unwrap_or_default();
    anyhow::bail!("exam protocol failed: {status} body={text}");
  }

  Ok(
    resp
      .json::<Vec<ExamProtocolItem>>()
      .await
      .context("invalid exam protocol json")?,
  )
}

async fn resolve_exam_subjects_for_term(
  client: &Client,
  access_token: &str,
  term: TermQuery,
  items: Vec<ExamProtocolItem>,
) -> TermSubjects {
  let mut subjects = TermSubjects::default();
  let mut settlement_cache: HashMap<(i64, i64), bool> = HashMap::new();

  for item in items {
    let Some(normalized_subject) = normalize_subject(&item.subject) else {
      continue;
    };

    if is_exam_settlement(item.settlement_method_name.as_deref()) {
      subjects.names.insert(normalized_subject);
      continue;
    }

    let exam_card_id = item.exam_card_id;
    let exam_card_position_id = item.exam_card_position_id;
    if exam_card_id <= 0 || exam_card_position_id <= 0 {
      continue;
    }

    let cache_key = (exam_card_id, exam_card_position_id);
    let is_exam = if let Some(value) = settlement_cache.get(&cache_key) {
      *value
    } else {
      let value = match get_exam_protocol_intermediate(
        client,
        access_token,
        exam_card_id,
        exam_card_position_id,
      )
      .await
      {
        Ok(entries) => entries
          .iter()
          .any(|entry| is_exam_settlement(entry.settlement_method_name.as_deref())),
        Err(error) => {
          warn!(
            academic_year = term.academic_year,
            semester_id = term.semester_id,
            exam_card_id,
            exam_card_position_id,
            error = %error,
            "exam protocol intermediate fetch failed"
          );
          false
        }
      };
      settlement_cache.insert(cache_key, value);
      value
    };

    if is_exam {
      subjects.names.insert(normalized_subject);
    }
  }

  subjects
}

async fn get_exam_protocol_intermediate(
  client: &Client,
  access_token: &str,
  exam_card_id: i64,
  exam_card_position_id: i64,
) -> Result<Vec<ExamProtocolIntermediateItem>> {
  let url = format!(
    "{API_BASE_URL}{API_EXAM_PROTOCOL_INTERMEDIATE_PATH}?KartaEgzID={exam_card_id}&KartaEgzPozID={exam_card_position_id}"
  );

  debug!(
    exam_card_id,
    exam_card_position_id, "GET {API_EXAM_PROTOCOL_INTERMEDIATE_PATH}"
  );
  let resp = client
    .get(url)
    .bearer_auth(access_token)
    .send()
    .await
    .context("exam protocol intermediate request failed")?;

  let status = resp.status();
  if !status.is_success() {
    let text = resp.text().await.unwrap_or_default();
    anyhow::bail!("exam protocol intermediate failed: {status} body={text}");
  }

  Ok(
    resp
      .json::<Vec<ExamProtocolIntermediateItem>>()
      .await
      .context("invalid exam protocol intermediate json")?,
  )
}

async fn get_exam_schedule(
  client: &Client,
  access_token: &str,
  term: TermQuery,
) -> Result<Vec<ExamScheduleItem>> {
  let url = format!(
    "{API_BASE_URL}{API_EXAM_FILTER_PATH}?KierunekID=&PracownikID=&RokAkad={}&SekcjaID=&SemestrID={}&SystemID=&TrybID=",
    term.academic_year, term.semester_id
  );

  debug!(
    academic_year = term.academic_year,
    semester_id = term.semester_id,
    "GET {API_EXAM_FILTER_PATH}"
  );
  let resp = client
    .get(url)
    .bearer_auth(access_token)
    .send()
    .await
    .context("exam schedule request failed")?;

  let status = resp.status();
  if !status.is_success() {
    let text = resp.text().await.unwrap_or_default();
    anyhow::bail!("exam schedule failed: {status} body={text}");
  }

  Ok(
    resp
      .json::<Vec<ExamScheduleItem>>()
      .await
      .context("invalid exam schedule json")?,
  )
}

fn normalize_subject(value: &str) -> Option<String> {
  let normalized = value
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ")
    .to_lowercase();
  if normalized.is_empty() {
    None
  } else {
    Some(normalized)
  }
}

fn is_exam_settlement(value: Option<&str>) -> bool {
  normalize_subject(value.unwrap_or_default()).is_some_and(|name| name == EXAM_SETTLEMENT_NAME)
}

fn map_exam_event(item: ExamScheduleItem, from: NaiveDate, to: NaiveDate) -> Option<ExamEvent> {
  let exam_date = item.exam_date.date();
  if exam_date < from || exam_date > to {
    return None;
  }

  let start_time = item
    .start_time
    .as_deref()
    .and_then(parse_time)
    .or_else(|| NaiveTime::from_hms_opt(9, 0, 0))?;
  let starts = exam_date.and_time(start_time);

  let mut ends = item
    .end_time
    .as_deref()
    .and_then(parse_time)
    .map(|time| exam_date.and_time(time))
    .unwrap_or(starts + Duration::minutes(90));

  if ends <= starts {
    ends = starts + Duration::minutes(90);
  }

  Some(ExamEvent {
    published_data_id: item.published_data_id,
    subject: item.exam_subject.trim().to_string(),
    notes: clean_text(item.notes),
    location: clean_text(item.room),
    lecturer: clean_lecturer(item.lecturer),
    details: clean_text(item.details),
    starts,
    ends,
  })
}

fn parse_time(value: &str) -> Option<NaiveTime> {
  let mut parts = value.trim().split(':');
  let hour = parts.next()?.trim().parse::<u32>().ok()?;
  let minute = parts.next()?.trim().parse::<u32>().ok()?;
  if parts.next().is_some() {
    return None;
  }

  NaiveTime::from_hms_opt(hour, minute, 0)
}

fn clean_text(value: Option<String>) -> Option<String> {
  value.and_then(|raw| {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
      None
    } else {
      Some(trimmed.to_string())
    }
  })
}

fn clean_lecturer(value: Option<String>) -> Option<String> {
  value.and_then(|raw| {
    let trimmed = raw.trim().trim_start_matches('-').trim();
    if trimmed.is_empty() {
      None
    } else {
      Some(trimmed.to_string())
    }
  })
}

fn build_terms_for_year(academic_year: i32) -> Vec<TermQuery> {
  vec![
    TermQuery {
      academic_year,
      semester_id: 1,
    },
    TermQuery {
      academic_year,
      semester_id: 2,
    },
  ]
}
