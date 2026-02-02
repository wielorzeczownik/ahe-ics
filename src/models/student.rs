use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct StudentData {
  #[serde(rename = "IDStudent")]
  pub student_id: i64,
  #[serde(rename = "IndeksID", alias = "IDIndeks", default)]
  pub index_id: Option<i64>,
  #[serde(rename = "Imie", default)]
  pub first_name: Option<String>,
  #[serde(rename = "DrugieImie", default)]
  pub middle_name: Option<String>,
  #[serde(rename = "Nazwisko", default)]
  pub last_name: Option<String>,
  #[serde(rename = "Email1", default)]
  pub primary_email: Option<String>,
  #[serde(rename = "TelefonKom", default)]
  pub mobile_phone: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StudentIndex {
  #[serde(rename = "IDIndeks")]
  pub index_id: i64,
  #[serde(rename = "StatusSymbol", default)]
  pub status_symbol: Option<String>,
  #[serde(rename = "Rok", default)]
  pub year: Option<i32>,
  #[serde(rename = "Semestr", default)]
  pub semester: Option<i32>,
}
