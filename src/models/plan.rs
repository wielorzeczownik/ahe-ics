use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Instructor {
  #[serde(rename = "ImieNazwisko")]
  pub full_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlanItem {
  #[serde(rename = "DataOD")]
  pub starts_at: NaiveDateTime,
  #[serde(rename = "DataDO")]
  pub ends_at: NaiveDateTime,
  #[serde(rename = "PNazwa")]
  pub subject_name: String,
  #[serde(rename = "TypZajec")]
  pub class_type: String,
  #[serde(rename = "TypZajecSkrot")]
  pub class_type_short: String,
  #[serde(rename = "SalaNumer", default)]
  pub room_number: Option<String>,
  #[serde(rename = "SalaAdres", default)]
  pub room_address: Option<String>,
  #[serde(rename = "Webinar", default)]
  pub webinar: bool,
  #[serde(rename = "Dydaktyk", default)]
  pub instructors: Vec<Instructor>,
  #[serde(rename = "IDPlanZajecPoz")]
  pub schedule_item_id: i64,
}
