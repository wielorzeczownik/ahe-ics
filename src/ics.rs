use anyhow::Result;
use chrono::Duration;
use icalendar::{Alarm, Calendar, Component, Event, EventLike, EventStatus, Property, Trigger};

use crate::config::CalendarLanguage;
use crate::constants::{
  CALENDAR_TZ, CLASS_REMINDER_MINUTES, EXAM_REMINDER_EARLY_MINUTES, EXAM_REMINDER_MINUTES,
  WPS_EXAM_URL, WPS_PLAN_URL,
};
use crate::i18n::{IcsTexts, ics_texts};
use crate::models::{ExamEvent, PlanItem};

/// Fallback event colours (RFC 7986 `COLOR`). Classes reuse the WPS `FormaKolor`;
/// exams carry no colour in the feed, so these fixed values are used instead.
const EXAM_COLOR: &str = "#E06666";
const EXAM_RETAKE_COLOR: &str = "#F6B26B";

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

    let mut event = Event::new();
    event
      .uid(&uid)
      .summary(&summary)
      .location(&location)
      .description(&description)
      .starts(item.starts_at)
      .ends(item.ends_at)
      .status(EventStatus::Confirmed)
      .append_property(Property::new("TRANSP", "OPAQUE"))
      .append_property(Property::new("URL", WPS_PLAN_URL))
      .alarm(Alarm::display(
        &summary,
        Trigger::before_start(Duration::minutes(CLASS_REMINDER_MINUTES)),
      ));

    let category = item.class_type.trim();
    if !category.is_empty() {
      event.append_property(Property::new("CATEGORIES", category));
    }
    if let Some(color) = item
      .form_color
      .as_deref()
      .map(str::trim)
      .filter(|value| !value.is_empty())
    {
      event.append_property(Property::new("COLOR", color));
    }

    calendar.push(event.done());
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
    let category = if exam.is_retake {
      texts.label_exam_retake
    } else {
      texts.label_exam
    };
    let color = if exam.is_retake {
      EXAM_RETAKE_COLOR
    } else {
      EXAM_COLOR
    };

    let mut event = Event::new();
    event
      .uid(&uid)
      .summary(&summary)
      .location(&location)
      .description(&description)
      .starts(exam.starts)
      .ends(exam.ends)
      .status(EventStatus::Confirmed)
      .append_property(Property::new("TRANSP", "OPAQUE"))
      .append_property(Property::new("URL", WPS_EXAM_URL))
      .append_property(Property::new("CATEGORIES", category))
      .append_property(Property::new("COLOR", color))
      .alarm(Alarm::display(
        &summary,
        Trigger::before_start(Duration::minutes(EXAM_REMINDER_EARLY_MINUTES)),
      ))
      .alarm(Alarm::display(
        &summary,
        Trigger::before_start(Duration::minutes(EXAM_REMINDER_MINUTES)),
      ));

    calendar.push(event.done());
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
  let label = if item.is_retake {
    texts.label_exam_retake
  } else {
    texts.label_exam
  };
  format!("{label}: {subject}")
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
