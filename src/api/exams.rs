use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use anyhow::{Context, Result};
use chrono::{Duration, NaiveDate, NaiveTime};
use reqwest::Client;
use tracing::{debug, warn};

use super::API_BASE_URL;
use crate::models::{
  CurrentAcademicYearResponse, ExamEvent, ExamProtocolIntermediateItem, ExamProtocolItem,
  ExamRecipient, ExamScheduleItem, TermQuery,
};

const API_EXAM_PROTOCOL_PATH: &str =
  "/api/ProtokolyEgzaminacyjne/GetProtokolEgzaminacyjnySzczegolowy";
const API_EXAM_PROTOCOL_INTERMEDIATE_PATH: &str =
  "/api/ProtokolyEgzaminacyjne/GetProtokolEgzaminacyjnyPosredni";
const API_EXAM_FILTER_PATH: &str = "/api/Egzaminy/GETEgazminFiltr";
const API_CURRENT_ACADEMIC_YEAR_PATH: &str = "/api/Slowniki/GETPobierzAktualnyRokAkademicki";
/// Settlement label that marks a subject as an exam (vs a plain pass).
const EXAM_SETTLEMENT_NAME: &str = "egzamin";

pub async fn get_exams(
  client: &Client,
  access_token: &str,
  index_id: i64,
  section_name: Option<&str>,
  from: NaiveDate,
  to: NaiveDate,
) -> Result<Vec<ExamEvent>> {
  let academic_year = get_current_academic_year(client, access_token).await?;
  let terms = build_terms_for_year(academic_year);

  let mut subjects_by_term: BTreeMap<TermQuery, BTreeSet<String>> = BTreeMap::new();
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
          if !subjects.contains(&normalized_subject) {
            continue;
          }
          if !recipient_section_matches(&item.recipients, section_name) {
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

  events.sort_by(|left, right| {
    left
      .starts
      .cmp(&right.starts)
      .then_with(|| left.subject.cmp(&right.subject))
  });
  Ok(events)
}

/// Reads current academic year used by WPS dictionary endpoints.
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

/// Fetches detailed exam protocol entries for a student index and term.
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

  resp
    .json::<Vec<ExamProtocolItem>>()
    .await
    .context("invalid exam protocol json")
}

/// Resolves subjects that should be treated as exams for a given term.
async fn resolve_exam_subjects_for_term(
  client: &Client,
  access_token: &str,
  term: TermQuery,
  items: Vec<ExamProtocolItem>,
) -> BTreeSet<String> {
  // We only keep subjects that are explicitly settled as an exam.
  // If "Szczegolowy" has null settlement, we resolve it via the "Posredni" endpoint.
  let mut subjects = BTreeSet::new();
  let mut settlement_cache: HashMap<(i64, i64), bool> = HashMap::new();

  for item in items {
    let Some(normalized_subject) = normalize_subject(&item.subject) else {
      continue;
    };

    if is_exam_settlement(item.settlement_method_name.as_deref()) {
      subjects.insert(normalized_subject);
      continue;
    }

    let exam_card_id = item.exam_card_id;
    let exam_card_position_id = item.exam_card_position_id;
    if exam_card_id <= 0 || exam_card_position_id <= 0 {
      continue;
    }

    let cache_key = (exam_card_id, exam_card_position_id);
    // Avoid duplicate network calls when the same exam card pair appears multiple times.
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
      subjects.insert(normalized_subject);
    }
  }

  subjects
}

/// Fetches intermediate protocol details used when settlement is missing in the detailed protocol.
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

  resp
    .json::<Vec<ExamProtocolIntermediateItem>>()
    .await
    .context("invalid exam protocol intermediate json")
}

/// Fetches public exam schedule entries for the selected academic term.
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

  resp
    .json::<Vec<ExamScheduleItem>>()
    .await
    .context("invalid exam schedule json")
}

/// Normalizes free text values for case-insensitive matching.
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

/// Returns true when settlement label maps to the configured "exam" keyword.
fn is_exam_settlement(value: Option<&str>) -> bool {
  normalize_subject(value.unwrap_or_default()).is_some_and(|name| name == EXAM_SETTLEMENT_NAME)
}

/// Maps a raw exam schedule item into an ICS event within the requested date window.
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
    .map_or(starts + Duration::minutes(90), |time| {
      exam_date.and_time(time)
    });

  if ends <= starts {
    ends = starts + Duration::minutes(90);
  }

  let is_retake = is_retake_notes(item.notes.as_deref());

  Some(ExamEvent {
    published_data_id: item.published_data_id,
    subject: item.exam_subject.trim().to_string(),
    notes: clean_text(item.notes),
    location: clean_text(item.room),
    lecturer: clean_lecturer(item.lecturer),
    details: clean_text(item.details),
    starts,
    ends,
    is_retake,
  })
}

/// Detects resit exams (`egzamin poprawkowy`) from the free-text notes field.
fn is_retake_notes(notes: Option<&str>) -> bool {
  notes.is_some_and(|value| value.to_lowercase().contains("poprawkow"))
}

/// Soft section guard used to disambiguate exams that share a subject name across sections.
fn recipient_section_matches(recipients: &[ExamRecipient], section_name: Option<&str>) -> bool {
  let Some(section) = section_name.and_then(normalize_subject) else {
    return true;
  };

  let mut saw_section = false;
  for recipient in recipients {
    if let Some(value) = recipient.section.as_deref().and_then(normalize_subject) {
      saw_section = true;
      if value == section {
        return true;
      }
    }
  }

  !saw_section
}

/// Parses `HH:MM` strings returned by WPS API.
fn parse_time(value: &str) -> Option<NaiveTime> {
  let mut parts = value.trim().split(':');
  let hour = parts.next()?.trim().parse::<u32>().ok()?;
  let minute = parts.next()?.trim().parse::<u32>().ok()?;
  if parts.next().is_some() {
    return None;
  }

  NaiveTime::from_hms_opt(hour, minute, 0)
}

/// Trims optional text fields and drops empty values.
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

/// Trims lecturer names and removes leading dashes used by API formatting.
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

/// Builds the default pair of terms (winter/summer) for the academic year.
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

#[cfg(test)]
mod tests {
  use super::*;

  fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("valid date")
  }

  fn recipient(section: Option<&str>) -> ExamRecipient {
    ExamRecipient {
      section: section.map(str::to_string),
    }
  }

  /// Minimal schedule item
  fn schedule_item(exam_day: NaiveDate) -> ExamScheduleItem {
    ExamScheduleItem {
      published_data_id: 100,
      exam_subject: "Analiza matematyczna".to_string(),
      notes: None,
      exam_date: exam_day.and_hms_opt(0, 0, 0).expect("valid midnight"),
      start_time: Some("10:00".to_string()),
      end_time: Some("11:30".to_string()),
      room: None,
      lecturer: None,
      details: None,
      recipients: Vec::new(),
    }
  }

  #[test]
  fn normalize_subject_collapses_whitespace_and_case() {
    assert_eq!(
      normalize_subject("  Analiza   MATEMATYCZNA \n II "),
      Some("analiza matematyczna ii".to_string())
    );
  }

  #[test]
  fn normalize_subject_rejects_blank_values() {
    assert_eq!(normalize_subject(""), None);
    assert_eq!(normalize_subject("   \t\n "), None);
  }

  #[test]
  fn is_exam_settlement_matches_only_exact_exam_label() {
    assert!(is_exam_settlement(Some("egzamin")));
    assert!(is_exam_settlement(Some("  EGZAMIN  ")));
    assert!(!is_exam_settlement(Some("zaliczenie")));
    assert!(!is_exam_settlement(Some("egzamin poprawkowy")));
    assert!(!is_exam_settlement(None));
  }

  #[test]
  fn is_retake_notes_detects_polish_declensions() {
    assert!(is_retake_notes(Some("Egzamin poprawkowy")));
    assert!(is_retake_notes(Some("termin POPRAWKOWY")));
    assert!(is_retake_notes(Some("egzamin poprawkowa sesja")));
    assert!(!is_retake_notes(Some("egzamin podstawowy")));
    assert!(!is_retake_notes(None));
  }

  #[test]
  fn parse_time_accepts_hour_minute_pairs() {
    assert_eq!(parse_time("09:45"), NaiveTime::from_hms_opt(9, 45, 0));
    assert_eq!(parse_time(" 9:45 "), NaiveTime::from_hms_opt(9, 45, 0));
    assert_eq!(parse_time("23:59"), NaiveTime::from_hms_opt(23, 59, 0));
  }

  #[test]
  fn parse_time_rejects_malformed_values() {
    assert_eq!(parse_time(""), None);
    assert_eq!(parse_time("10"), None);
    assert_eq!(parse_time("10:aa"), None);
    assert_eq!(parse_time("24:00"), None);
    assert_eq!(parse_time("10:60"), None);
    assert_eq!(parse_time("10:00:00"), None);
  }

  #[test]
  fn recipient_section_matches_without_configured_section() {
    let recipients = vec![recipient(Some("IN1"))];
    assert!(recipient_section_matches(&recipients, None));
    // A blank configured section normalizes away and disables the guard
    assert!(recipient_section_matches(&recipients, Some("   ")));
  }

  #[test]
  fn recipient_section_matches_on_case_insensitive_section() {
    let recipients = vec![recipient(Some("ZAOCZNE")), recipient(Some("in1"))];
    assert!(recipient_section_matches(&recipients, Some("IN1")));
  }

  #[test]
  fn recipient_section_rejects_foreign_sections() {
    let recipients = vec![recipient(Some("IN2")), recipient(Some("IN3"))];
    assert!(!recipient_section_matches(&recipients, Some("IN1")));
  }

  #[test]
  fn recipient_section_passes_when_feed_carries_no_section() {
    // Soft guard
    let recipients = vec![recipient(None), recipient(Some("  "))];
    assert!(recipient_section_matches(&recipients, Some("IN1")));
    assert!(recipient_section_matches(&[], Some("IN1")));
  }

  #[test]
  fn map_exam_event_keeps_events_inside_window_inclusively() {
    let from = date(2026, 1, 10);
    let to = date(2026, 1, 20);

    for day in [date(2026, 1, 10), date(2026, 1, 15), date(2026, 1, 20)] {
      assert!(
        map_exam_event(schedule_item(day), from, to).is_some(),
        "expected {day} to fall inside the window"
      );
    }
  }

  #[test]
  fn map_exam_event_drops_events_outside_window() {
    let from = date(2026, 1, 10);
    let to = date(2026, 1, 20);

    assert!(map_exam_event(schedule_item(date(2026, 1, 9)), from, to).is_none());
    assert!(map_exam_event(schedule_item(date(2026, 1, 21)), from, to).is_none());
  }

  #[test]
  fn map_exam_event_uses_declared_times() {
    let day = date(2026, 1, 15);
    let event = map_exam_event(schedule_item(day), day, day).expect("event in window");

    assert_eq!(event.starts, day.and_hms_opt(10, 0, 0).expect("start"));
    assert_eq!(event.ends, day.and_hms_opt(11, 30, 0).expect("end"));
  }

  #[test]
  fn map_exam_event_defaults_missing_start_to_nine() {
    let day = date(2026, 1, 15);
    let mut item = schedule_item(day);
    item.start_time = None;
    item.end_time = None;

    let event = map_exam_event(item, day, day).expect("event in window");

    assert_eq!(event.starts, day.and_hms_opt(9, 0, 0).expect("start"));
    // No end time in the feed means a 90 minute slot.
    assert_eq!(event.ends, day.and_hms_opt(10, 30, 0).expect("end"));
  }

  #[test]
  fn map_exam_event_repairs_non_positive_duration() {
    let day = date(2026, 1, 15);
    let mut item = schedule_item(day);
    item.start_time = Some("14:00".to_string());
    // End before start (or equal to it) must fall back to the default length
    item.end_time = Some("13:00".to_string());

    let event = map_exam_event(item, day, day).expect("event in window");

    assert_eq!(event.starts, day.and_hms_opt(14, 0, 0).expect("start"));
    assert_eq!(event.ends, day.and_hms_opt(15, 30, 0).expect("end"));
  }

  #[test]
  fn map_exam_event_falls_back_when_start_time_is_unparsable() {
    let day = date(2026, 1, 15);
    let mut item = schedule_item(day);
    item.start_time = Some("nope".to_string());
    item.end_time = Some("11:30".to_string());

    let event = map_exam_event(item, day, day).expect("event in window");

    assert_eq!(event.starts, day.and_hms_opt(9, 0, 0).expect("start"));
    assert_eq!(event.ends, day.and_hms_opt(11, 30, 0).expect("end"));
  }

  #[test]
  fn map_exam_event_cleans_optional_text_fields() {
    let day = date(2026, 1, 15);
    let mut item = schedule_item(day);
    item.exam_subject = "  Analiza matematyczna  ".to_string();
    item.room = Some("   ".to_string());
    item.lecturer = Some("- dr Jan Kowalski".to_string());
    item.details = Some("  sala A  ".to_string());
    item.notes = Some("Egzamin poprawkowy".to_string());

    let event = map_exam_event(item, day, day).expect("event in window");

    assert_eq!(event.subject, "Analiza matematyczna");
    assert_eq!(event.location, None);
    assert_eq!(event.lecturer.as_deref(), Some("dr Jan Kowalski"));
    assert_eq!(event.details.as_deref(), Some("sala A"));
    assert!(event.is_retake);
  }

  #[test]
  fn clean_lecturer_strips_leading_dashes() {
    assert_eq!(
      clean_lecturer(Some("--  dr Jan Kowalski ".to_string())).as_deref(),
      Some("dr Jan Kowalski")
    );
    assert_eq!(clean_lecturer(Some("  -  ".to_string())), None);
    assert_eq!(clean_lecturer(None), None);
  }

  #[test]
  fn clean_text_drops_blank_values() {
    assert_eq!(clean_text(Some("  x  ".to_string())).as_deref(), Some("x"));
    assert_eq!(clean_text(Some("   ".to_string())), None);
    assert_eq!(clean_text(None), None);
  }

  #[test]
  fn build_terms_for_year_covers_both_semesters() {
    let terms = build_terms_for_year(2025);

    assert_eq!(terms.len(), 2);
    assert!(terms.iter().all(|term| term.academic_year == 2025));
    assert_eq!(terms[0].semester_id, 1);
    assert_eq!(terms[1].semester_id, 2);
  }
}
