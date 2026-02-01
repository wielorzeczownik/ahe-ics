use anyhow::Result;
use icalendar::{ Calendar, Component, Event, EventLike };

use crate::config::CalendarLanguage;
use crate::constants::CALENDAR_TZ;
use crate::i18n::{ IcsTexts, ics_texts };
use crate::models::PlanItem;

/// Renders a list of plan items into a single ICS calendar string.
pub fn render_calendar(
  student_id: i64,
  items: &[PlanItem],
  lang: CalendarLanguage
) -> Result<String> {
  let t = ics_texts(lang);

  let mut calendar = Calendar::new();
  calendar.name(t.calendar_name);
  calendar.timezone(CALENDAR_TZ);

  for item in items {
    let uid = format!("ahe-{student_id}-{}@wpsapi.ahe.lodz.pl", item.id_plan_zajec_poz);
    let summary = build_summary(item);
    let location = build_location(item, &t);
    let description = build_description(item, &t);

    let event = Event::new()
      .uid(&uid)
      .summary(&summary)
      .location(&location)
      .description(&description)
      .starts(item.data_od)
      .ends(item.data_do)
      .done();

    calendar.push(event);
  }

  Ok(calendar.to_string())
}

fn build_summary(item: &PlanItem) -> String {
  let mut typ = item.typ_zajec.clone();
  if !item.typ_zajec_skrot.trim().is_empty() {
    typ = format!("{} {}", item.typ_zajec, item.typ_zajec_skrot);
  }

  format!("{} [{typ}]", item.p_nazwa)
}

fn build_location(item: &PlanItem, t: &IcsTexts) -> String {
  if item.webinar {
    return t.location_webinar.to_string();
  }

  let mut parts = Vec::new();
  if let Some(value) = item.sala_numer.as_ref().filter(|v| !v.trim().is_empty()) {
    parts.push(value.trim());
  }
  if let Some(value) = item.sala_adres.as_ref().filter(|v| !v.trim().is_empty()) {
    parts.push(value.trim());
  }

  if parts.is_empty() {
    t.location_default.to_string()
  } else {
    parts.join(" — ")
  }
}

fn build_description(item: &PlanItem, t: &IcsTexts) -> String {
  let instructors = if item.dydaktyk.is_empty() {
    t.missing_data.to_string()
  } else {
    item.dydaktyk
      .iter()
      .map(|d| d.imie_nazwisko.as_str())
      .collect::<Vec<_>>()
      .join(", ")
  };

  format!("{}: {instructors}\n{}: {}", t.label_instructors, t.label_type, item.typ_zajec)
}
