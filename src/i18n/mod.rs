mod en;
mod pl;

use crate::config::CalendarLanguage;

pub struct IcsTexts {
  pub calendar_name: &'static str,
  pub location_webinar: &'static str,
  pub location_default: &'static str,
  pub label_instructors: &'static str,
  pub label_type: &'static str,
  pub missing_data: &'static str,
}

pub fn ics_texts(lang: CalendarLanguage) -> &'static IcsTexts {
  match lang {
    CalendarLanguage::Pl => &pl::ICS_TEXTS,
    CalendarLanguage::En => &en::ICS_TEXTS,
  }
}
