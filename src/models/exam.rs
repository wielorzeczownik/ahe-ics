use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TermQuery {
  pub rok_akad: i32,
  pub semestr_id: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExamProtocolItem {
  #[serde(rename = "Przedmiot", default)]
  pub przedmiot: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExamScheduleItem {
  #[serde(rename = "IDPublikowanaDana")]
  pub id_publikowana_dana: i64,
  #[serde(rename = "EgzPrzedmiot", default)]
  pub egz_przedmiot: String,
  #[serde(rename = "Uwagi", default)]
  pub uwagi: Option<String>,
  #[serde(rename = "EgzData")]
  pub egz_data: NaiveDateTime,
  #[serde(rename = "GodzOd", default)]
  pub godz_od: Option<String>,
  #[serde(rename = "GodzDo", default)]
  pub godz_do: Option<String>,
  #[serde(rename = "Sala", default)]
  pub sala: Option<String>,
  #[serde(rename = "Wykladowca", default)]
  pub wykladowca: Option<String>,
  #[serde(rename = "OpisSzczegolowy", default)]
  pub opis_szczegolowy: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ExamEvent {
  pub id_publikowana_dana: i64,
  pub subject: String,
  pub notes: Option<String>,
  pub location: Option<String>,
  pub lecturer: Option<String>,
  pub details: Option<String>,
  pub starts: NaiveDateTime,
  pub ends: NaiveDateTime,
}
