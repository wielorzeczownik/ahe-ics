use anyhow::Result;
use icalendar::{Calendar, Component, Event, EventLike};

use crate::config::CalendarLanguage;
use crate::constants::CALENDAR_TZ;
use crate::i18n::{IcsTexts, ics_texts};
use crate::models::{ExamEvent, PlanItem};

/// Renders a list of plan items into a single ICS calendar string.
///
/// # Errors
///
/// Returns an error if the calendar cannot be serialized into ICS form.
pub fn render_calendar(
  student_id: i64,
  items: &[PlanItem],
  exams: &[ExamEvent],
  lang: CalendarLanguage,
) -> Result<String> {
  let texts = ics_texts(lang);

  let mut calendar = Calendar::new();
  calendar.name(texts.calendar_name);
  calendar.timezone(CALENDAR_TZ);

  for item in items {
    let uid = format!(
      "ahe-{student_id}-{}@wpsapi.ahe.lodz.pl",
      item.schedule_item_id
    );
    let summary = build_summary(item);
    let location = build_location(item, texts);
    let description = build_description(item, texts);

    let event = Event::new()
      .uid(&uid)
      .summary(&summary)
      .location(&location)
      .description(&description)
      .starts(item.starts_at)
      .ends(item.ends_at)
      .done();

    calendar.push(event);
  }

  for exam in exams {
    let uid = format!(
      "ahe-{student_id}-exam-{}-{}@wpsapi.ahe.lodz.pl",
      exam.published_data_id,
      exam.starts.and_utc().timestamp()
    );
    let summary = build_exam_summary(exam, texts);
    let location = build_exam_location(exam, texts);
    let description = build_exam_description(exam, texts);

    let event = Event::new()
      .uid(&uid)
      .summary(&summary)
      .location(&location)
      .description(&description)
      .starts(exam.starts)
      .ends(exam.ends)
      .done();

    calendar.push(event);
  }

  Ok(calendar.to_string())
}

fn build_summary(item: &PlanItem) -> String {
  let typ = match item.class_type_short.trim() {
    "" => item.class_type.clone(),
    short => format!("{} {short}", item.class_type),
  };
  format!("{} [{typ}]", item.subject_name)
}

fn build_location(item: &PlanItem, texts: &IcsTexts) -> String {
  if item.webinar {
    return texts.location_webinar.to_string();
  }

  let mut parts = Vec::new();
  if let Some(value) = item
    .room_number
    .as_ref()
    .filter(|value| !value.trim().is_empty())
  {
    parts.push(value.trim());
  }
  if let Some(value) = item
    .room_address
    .as_ref()
    .filter(|value| !value.trim().is_empty())
  {
    parts.push(value.trim());
  }

  if parts.is_empty() {
    texts.location_default.to_string()
  } else {
    parts.join(" — ")
  }
}

fn build_description(item: &PlanItem, texts: &IcsTexts) -> String {
  let instructors = if item.instructors.is_empty() {
    texts.missing_data.to_string()
  } else {
    item
      .instructors
      .iter()
      .map(|instructor| instructor.full_name.as_str())
      .collect::<Vec<_>>()
      .join(", ")
  };

  format!(
    "{}: {instructors}\n{}: {}",
    texts.label_instructors, texts.label_type, item.class_type
  )
}

fn build_exam_summary(item: &ExamEvent, texts: &IcsTexts) -> String {
  let subject = if item.subject.trim().is_empty() {
    texts.missing_data.to_string()
  } else {
    item.subject.trim().to_string()
  };
  format!("{}: {subject}", texts.label_exam)
}

fn build_exam_location(item: &ExamEvent, texts: &IcsTexts) -> String {
  item
    .location
    .as_ref()
    .map(|value| value.trim())
    .filter(|value| !value.is_empty())
    .unwrap_or(texts.location_default)
    .to_string()
}

fn build_exam_description(item: &ExamEvent, texts: &IcsTexts) -> String {
  let notes = item.notes.as_deref().unwrap_or(texts.missing_data);
  let lecturer = item.lecturer.as_deref().unwrap_or(texts.missing_data);
  let details = item.details.as_deref().unwrap_or(texts.missing_data);

  format!(
    "{}: {notes}\n{}: {lecturer}\n{}: {details}",
    texts.label_exam_type, texts.label_instructors, texts.label_details
  )
}
