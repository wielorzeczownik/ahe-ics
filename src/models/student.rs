use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct StudentData {
  #[serde(rename = "IDStudent")]
  pub id_student: i64,
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
