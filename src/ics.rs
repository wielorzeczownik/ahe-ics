use anyhow::Result;
use chrono::Duration;
use icalendar::{Alarm, Calendar, Component, Event, EventLike, EventStatus, Property, Trigger};

use crate::config::CalendarLanguage;
use crate::i18n::{IcsTexts, ics_texts};
use crate::models::{ExamEvent, PlanItem};

/// IANA timezone all calendar events are expressed in.
const CALENDAR_TZ: &str = "Europe/Warsaw";

/// Fallback event colours (RFC 7986 `COLOR`). Classes reuse the WPS `FormaKolor`;
/// exams carry no colour in the feed, so these fixed values are used instead.
const EXAM_COLOR: &str = "#E06666";
const EXAM_RETAKE_COLOR: &str = "#F6B26B";

/// Reminder lead times (minutes before start) emitted as VALARM components.
const CLASS_REMINDER_MINUTES: i64 = 15;
const EXAM_REMINDER_MINUTES: i64 = 60;
const EXAM_REMINDER_EARLY_MINUTES: i64 = 24 * 60;

/// WPS site pages linked from calendar events via the `URL` property.
const WPS_PLAN_URL: &str = "https://wps.ahe.lodz.pl/plan-kalendarzowy";
const WPS_EXAM_URL: &str = "https://wps.ahe.lodz.pl/egzaminy";

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

#[cfg(test)]
mod tests {
  use chrono::NaiveDate;

  use super::*;
  use crate::i18n::{en::EN, pl::PL};
  use crate::models::Instructor;

  fn datetime(hour: u32, minute: u32) -> chrono::NaiveDateTime {
    NaiveDate::from_ymd_opt(2026, 1, 15)
      .expect("valid date")
      .and_hms_opt(hour, minute, 0)
      .expect("valid time")
  }

  fn plan_item() -> PlanItem {
    PlanItem {
      starts_at: datetime(10, 0),
      ends_at: datetime(11, 30),
      subject_name: "Algebra".to_string(),
      class_type: "Wyklad".to_string(),
      class_type_short: "W".to_string(),
      room_number: Some("A12".to_string()),
      room_address: Some("Sterlinga 26".to_string()),
      webinar: false,
      instructors: vec![Instructor {
        full_name: "Jan Kowalski".to_string(),
      }],
      schedule_item_id: 555,
      form_color: Some("#123456".to_string()),
    }
  }

  fn exam_event() -> ExamEvent {
    ExamEvent {
      published_data_id: 777,
      subject: "Algebra".to_string(),
      notes: None,
      location: Some("A12".to_string()),
      lecturer: Some("Jan Kowalski".to_string()),
      details: None,
      starts: datetime(9, 0),
      ends: datetime(10, 30),
      is_retake: false,
    }
  }

  fn render(items: &[PlanItem], exams: &[ExamEvent]) -> String {
    render_calendar(42, items, exams, CalendarLanguage::Pl).expect("render succeeds")
  }

  fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
  }

  #[test]
  fn empty_calendar_still_renders_a_valid_envelope() {
    let ics = render(&[], &[]);

    assert!(ics.starts_with("BEGIN:VCALENDAR"));
    assert!(ics.trim_end().ends_with("END:VCALENDAR"));
    assert_eq!(count(&ics, "BEGIN:VEVENT"), 0);
  }

  #[test]
  fn plan_item_uid_is_stable_and_scoped_to_student() {
    let ics = render(&[plan_item()], &[]);

    assert!(ics.contains("UID:ahe-42-555@wpsapi.ahe.lodz.pl"));
  }

  #[test]
  fn exam_uid_includes_start_timestamp() {
    let exam = exam_event();
    let timestamp = exam.starts.and_utc().timestamp();
    let ics = render(&[], &[exam]);

    assert!(ics.contains(&format!(
      "UID:ahe-42-exam-777-{timestamp}@wpsapi.ahe.lodz.pl"
    )));
  }

  #[test]
  fn classes_get_one_reminder_and_exams_get_two() {
    // Durations are serialized in seconds, e.g. 15 minutes becomes `-PT900S`
    let plan_only = render(&[plan_item()], &[]);
    assert_eq!(count(&plan_only, "BEGIN:VALARM"), 1);
    assert!(plan_only.contains("TRIGGER;RELATED=START:-PT900S"));

    let exam_only = render(&[], &[exam_event()]);
    assert_eq!(count(&exam_only, "BEGIN:VALARM"), 2);
    assert!(exam_only.contains("TRIGGER;RELATED=START:-PT86400S"));
    assert!(exam_only.contains("TRIGGER;RELATED=START:-PT3600S"));
  }

  #[test]
  fn both_event_kinds_land_in_one_calendar() {
    let ics = render(&[plan_item()], &[exam_event()]);

    assert_eq!(count(&ics, "BEGIN:VEVENT"), 2);
    assert_eq!(count(&ics, "END:VEVENT"), 2);
  }

  #[test]
  fn class_summary_combines_type_and_short_code() {
    let item = plan_item();
    assert_eq!(build_summary(&item), "Algebra [Wyklad W]");
  }

  #[test]
  fn class_summary_omits_blank_short_code() {
    let mut item = plan_item();
    item.class_type_short = "   ".to_string();

    assert_eq!(build_summary(&item), "Algebra [Wyklad]");
  }

  #[test]
  fn webinar_location_wins_over_room_details() {
    let mut item = plan_item();
    item.webinar = true;

    assert_eq!(build_location(&item, &PL), "Webinar");
    assert_eq!(build_location(&item, &EN), "Webinar");
  }

  #[test]
  fn location_joins_room_number_and_address() {
    assert_eq!(build_location(&plan_item(), &PL), "A12 — Sterlinga 26");
  }

  #[test]
  fn location_skips_blank_parts() {
    let mut item = plan_item();
    item.room_number = Some("  ".to_string());

    assert_eq!(build_location(&item, &PL), "Sterlinga 26");

    item.room_address = None;
    assert_eq!(build_location(&item, &PL), "Sala");
    assert_eq!(build_location(&item, &EN), "Room");
  }

  #[test]
  fn description_lists_all_instructors() {
    let mut item = plan_item();
    item.instructors.push(Instructor {
      full_name: "Anna Nowak".to_string(),
    });

    let description = build_description(&item, &PL);
    assert!(description.contains("Prowadzacy: Jan Kowalski, Anna Nowak"));
    assert!(description.contains("Typ: Wyklad"));
  }

  #[test]
  fn description_falls_back_when_instructors_are_missing() {
    let mut item = plan_item();
    item.instructors.clear();

    assert!(build_description(&item, &PL).contains("Prowadzacy: (brak danych)"));
    assert!(build_description(&item, &EN).contains("Instructors: (no data)"));
  }

  #[test]
  fn exam_summary_marks_retakes() {
    let mut exam = exam_event();
    assert_eq!(build_exam_summary(&exam, &PL), "Egzamin: Algebra");

    exam.is_retake = true;
    assert_eq!(
      build_exam_summary(&exam, &PL),
      "Egzamin poprawkowy: Algebra"
    );
    assert_eq!(build_exam_summary(&exam, &EN), "Resit exam: Algebra");
  }

  #[test]
  fn exam_summary_handles_blank_subject() {
    let mut exam = exam_event();
    exam.subject = "   ".to_string();

    assert_eq!(build_exam_summary(&exam, &PL), "Egzamin: (brak danych)");
  }

  #[test]
  fn exam_description_fills_gaps_with_placeholder() {
    let exam = exam_event();
    let description = build_exam_description(&exam, &PL);

    assert!(description.contains("Rodzaj: (brak danych)"));
    assert!(description.contains("Prowadzacy: Jan Kowalski"));
    assert!(description.contains("Szczegoly: (brak danych)"));
  }

  #[test]
  fn exam_location_falls_back_to_default_room() {
    let mut exam = exam_event();
    assert_eq!(build_exam_location(&exam, &PL), "A12");

    exam.location = Some("   ".to_string());
    assert_eq!(build_exam_location(&exam, &PL), "Sala");

    exam.location = None;
    assert_eq!(build_exam_location(&exam, &EN), "Room");
  }

  #[test]
  fn retake_exams_use_a_distinct_colour_and_category() {
    let mut exam = exam_event();
    exam.is_retake = true;
    let ics = render(&[], &[exam]);

    assert!(ics.contains(EXAM_RETAKE_COLOR));
    assert!(!ics.contains(EXAM_COLOR));
  }

  #[test]
  fn class_colour_comes_from_the_feed() {
    let ics = render(&[plan_item()], &[]);
    assert!(ics.contains("COLOR:#123456"));
    assert!(ics.contains("CATEGORIES:Wyklad"));
  }

  #[test]
  fn blank_class_colour_is_omitted() {
    let mut item = plan_item();
    item.form_color = Some("   ".to_string());

    assert!(!render(&[item.clone()], &[]).contains("COLOR:"));

    item.form_color = None;
    assert!(!render(&[item], &[]).contains("COLOR:"));
  }

  #[test]
  fn calendar_name_follows_the_configured_language() {
    let polish = render_calendar(42, &[], &[], CalendarLanguage::Pl).expect("render succeeds");
    let english = render_calendar(42, &[], &[], CalendarLanguage::En).expect("render succeeds");

    assert!(polish.contains("Plan AHE"));
    assert!(english.contains("AHE Schedule"));
  }
}
