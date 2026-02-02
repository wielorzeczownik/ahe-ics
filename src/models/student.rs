use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct StudentData {
  #[serde(rename = "IDStudent")]
  pub id_student: i64,
  #[serde(rename = "IndeksID", alias = "IDIndeks", default)]
  pub indeks_id: Option<i64>,
  #[serde(rename = "Imie", default)]
  pub imie: Option<String>,
  #[serde(rename = "DrugieImie", default)]
  pub drugie_imie: Option<String>,
  #[serde(rename = "Nazwisko", default)]
  pub nazwisko: Option<String>,
  #[serde(rename = "Email1", default)]
  pub email1: Option<String>,
  #[serde(rename = "TelefonKom", default)]
  pub telefon_kom: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StudentIndex {
  #[serde(rename = "IDIndeks")]
  pub id_indeks: i64,
  #[serde(rename = "StatusSymbol", default)]
  pub status_symbol: Option<String>,
  #[serde(rename = "Rok", default)]
  pub rok: Option<i32>,
  #[serde(rename = "Semestr", default)]
  pub semestr: Option<i32>,
}
