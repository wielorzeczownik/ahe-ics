use std::collections::{BTreeMap, HashSet};

use anyhow::{Context, Result};
use chrono::{Duration, NaiveDate, NaiveTime};
use reqwest::Client;
use serde::Deserialize;
use tracing::{debug, warn};

use crate::constants::{
  API_BASE_URL, API_CURRENT_ACADEMIC_YEAR_PATH, API_EXAM_FILTER_PATH, API_EXAM_PROTOCOL_PATH,
};
use crate::models::{ExamEvent, ExamProtocolItem, ExamScheduleItem, TermQuery};

#[derive(Debug, Clone, Default)]
struct TermSubjects {
  names: std::collections::BTreeSet<String>,
}

#[derive(Debug, Deserialize)]
struct CurrentAcademicYearResponse {
  #[serde(rename = "RokAkad")]
  rok_akad: i32,
}

impl TermSubjects {
  fn from_protocol_items(items: Vec<ExamProtocolItem>) -> Self {
    let mut subjects = Self::default();
    for item in items {
      if let Some(name) = normalize_subject(&item.przedmiot) {
        subjects.names.insert(name);
      }
    }
    subjects
  }

  fn is_empty(&self) -> bool {
    self.names.is_empty()
  }
}

pub async fn get_exams(
  client: &Client,
  access_token: &str,
  indeks_id: i64,
  from: NaiveDate,
  to: NaiveDate,
) -> Result<Vec<ExamEvent>> {
  let rok_akad = get_current_academic_year(client, access_token).await?;
  let terms = build_terms_for_year(rok_akad);

  let mut subjects_by_term: BTreeMap<TermQuery, TermSubjects> = BTreeMap::new();
  for term in terms {
    match get_exam_protocol(client, access_token, indeks_id, term).await {
      Ok(items) => {
        let subjects = TermSubjects::from_protocol_items(items);
        if !subjects.is_empty() {
          subjects_by_term.insert(term, subjects);
        }
      }
      Err(error) => {
        warn!(
          rok_akad = term.rok_akad,
          semestr_id = term.semestr_id,
          error = %error,
          "exam protocol fetch failed"
        );
      }
    }
  }

  if subjects_by_term.is_empty() {
    debug!(
      indeks_id,
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
          let Some(normalized_subject) = normalize_subject(&item.egz_przedmiot) else {
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
            event.id_publikowana_dana, event.starts, normalized_subject
          );
          if seen.insert(key) {
            events.push(event);
          }
        }
      }
      Err(error) => {
        warn!(
          rok_akad = term.rok_akad,
          semestr_id = term.semestr_id,
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

  Ok(payload.rok_akad)
}

async fn get_exam_protocol(
  client: &Client,
  access_token: &str,
  indeks_id: i64,
  term: TermQuery,
) -> Result<Vec<ExamProtocolItem>> {
  let url = format!(
    "{API_BASE_URL}{API_EXAM_PROTOCOL_PATH}?IndeksID={indeks_id}&RokAkad={}&SemestrID={}",
    term.rok_akad, term.semestr_id
  );

  debug!(
    indeks_id,
    rok_akad = term.rok_akad,
    semestr_id = term.semestr_id,
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

async fn get_exam_schedule(
  client: &Client,
  access_token: &str,
  term: TermQuery,
) -> Result<Vec<ExamScheduleItem>> {
  let url = format!(
    "{API_BASE_URL}{API_EXAM_FILTER_PATH}?KierunekID=&PracownikID=&RokAkad={}&SekcjaID=&SemestrID={}&SystemID=&TrybID=",
    term.rok_akad, term.semestr_id
  );

  debug!(
    rok_akad = term.rok_akad,
    semestr_id = term.semestr_id,
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

fn map_exam_event(item: ExamScheduleItem, from: NaiveDate, to: NaiveDate) -> Option<ExamEvent> {
  let exam_date = item.egz_data.date();
  if exam_date < from || exam_date > to {
    return None;
  }

  let start_time = item
    .godz_od
    .as_deref()
    .and_then(parse_time)
    .or_else(|| NaiveTime::from_hms_opt(9, 0, 0))?;
  let starts = exam_date.and_time(start_time);

  let mut ends = item
    .godz_do
    .as_deref()
    .and_then(parse_time)
    .map(|time| exam_date.and_time(time))
    .unwrap_or(starts + Duration::minutes(90));

  if ends <= starts {
    ends = starts + Duration::minutes(90);
  }

  Some(ExamEvent {
    id_publikowana_dana: item.id_publikowana_dana,
    subject: item.egz_przedmiot.trim().to_string(),
    notes: clean_text(item.uwagi),
    location: clean_text(item.sala),
    lecturer: clean_lecturer(item.wykladowca),
    details: clean_text(item.opis_szczegolowy),
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

fn build_terms_for_year(rok_akad: i32) -> Vec<TermQuery> {
  vec![
    TermQuery {
      rok_akad,
      semestr_id: 1,
    },
    TermQuery {
      rok_akad,
      semestr_id: 2,
    },
  ]
}
