use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Dydaktyk {
  #[serde(rename = "ImieNazwisko")]
  pub imie_nazwisko: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlanItem {
  #[serde(rename = "DataOD")]
  pub data_od: NaiveDateTime,
  #[serde(rename = "DataDO")]
  pub data_do: NaiveDateTime,
  #[serde(rename = "PNazwa")]
  pub p_nazwa: String,
  #[serde(rename = "TypZajec")]
  pub typ_zajec: String,
  #[serde(rename = "TypZajecSkrot")]
  pub typ_zajec_skrot: String,
  #[serde(rename = "SalaNumer")]
  #[serde(default)]
  pub sala_numer: Option<String>,
  #[serde(rename = "SalaAdres")]
  #[serde(default)]
  pub sala_adres: Option<String>,
  #[serde(rename = "Webinar")]
  #[serde(default)]
  pub webinar: bool,
  #[serde(rename = "Dydaktyk")]
  #[serde(default)]
  pub dydaktyk: Vec<Dydaktyk>,
  #[serde(rename = "IDPlanZajecPoz")]
  pub id_plan_zajec_poz: i64,
}
