use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use utoipa::ToSchema;

use crate::models::{ExamEvent, PlanItem};

#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct CalendarJsonResponse {
  student_id: i64,
  from: NaiveDate,
  to: NaiveDate,
  plan: Vec<CalendarPlanJsonItem>,
  exams: Vec<CalendarExamJsonItem>,
}

#[derive(Debug, Serialize, ToSchema)]
struct CalendarPlanJsonItem {
  schedule_item_id: i64,
  starts_at: NaiveDateTime,
  ends_at: NaiveDateTime,
  subject_name: String,
  class_type: String,
  class_type_short: String,
  room_number: Option<String>,
  room_address: Option<String>,
  webinar: bool,
  instructors: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
struct CalendarExamJsonItem {
  published_data_id: i64,
  subject: String,
  notes: Option<String>,
  location: Option<String>,
  lecturer: Option<String>,
  details: Option<String>,
  starts: NaiveDateTime,
  ends: NaiveDateTime,
}

impl CalendarJsonResponse {
  pub(crate) fn from_parts(
    student_id: i64,
    from: NaiveDate,
    to: NaiveDate,
    plan: Vec<PlanItem>,
    exams: Vec<ExamEvent>,
  ) -> Self {
    Self {
      student_id,
      from,
      to,
      plan: plan.into_iter().map(Into::into).collect(),
      exams: exams.into_iter().map(Into::into).collect(),
    }
  }
}

impl From<PlanItem> for CalendarPlanJsonItem {
  fn from(value: PlanItem) -> Self {
    Self {
      schedule_item_id: value.schedule_item_id,
      starts_at: value.starts_at,
      ends_at: value.ends_at,
      subject_name: value.subject_name,
      class_type: value.class_type,
      class_type_short: value.class_type_short,
      room_number: value.room_number,
      room_address: value.room_address,
      webinar: value.webinar,
      instructors: value
        .instructors
        .into_iter()
        .map(|instructor| instructor.full_name)
        .collect(),
    }
  }
}

impl From<ExamEvent> for CalendarExamJsonItem {
  fn from(value: ExamEvent) -> Self {
    Self {
      published_data_id: value.published_data_id,
      subject: value.subject,
      notes: value.notes,
      location: value.location,
      lecturer: value.lecturer,
      details: value.details,
      starts: value.starts,
      ends: value.ends,
    }
  }
}
