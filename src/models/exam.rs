use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CurrentAcademicYearResponse {
  #[serde(rename = "RokAkad")]
  pub academic_year: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TermQuery {
  pub academic_year: i32,
  pub semester_id: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExamProtocolItem {
  #[serde(rename = "Przedmiot", default)]
  pub subject: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExamScheduleItem {
  #[serde(rename = "IDPublikowanaDana")]
  pub published_data_id: i64,
  #[serde(rename = "EgzPrzedmiot", default)]
  pub exam_subject: String,
  #[serde(rename = "Uwagi", default)]
  pub notes: Option<String>,
  #[serde(rename = "EgzData")]
  pub exam_date: NaiveDateTime,
  #[serde(rename = "GodzOd", default)]
  pub start_time: Option<String>,
  #[serde(rename = "GodzDo", default)]
  pub end_time: Option<String>,
  #[serde(rename = "Sala", default)]
  pub room: Option<String>,
  #[serde(rename = "Wykladowca", default)]
  pub lecturer: Option<String>,
  #[serde(rename = "OpisSzczegolowy", default)]
  pub details: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ExamEvent {
  pub published_data_id: i64,
  pub subject: String,
  pub notes: Option<String>,
  pub location: Option<String>,
  pub lecturer: Option<String>,
  pub details: Option<String>,
  pub starts: NaiveDateTime,
  pub ends: NaiveDateTime,
}
