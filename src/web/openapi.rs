use anyhow::Result;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
  paths(
    crate::web::routes::calendar::calendar,
    crate::web::routes::calendar::calendar_me,
    crate::web::routes::calendar::calendar_json,
    crate::web::routes::calendar::calendar_me_json,
    crate::web::routes::health::healthz,
    crate::web::routes::openapi_json
  ),
  tags(
    (name = "calendar", description = "Calendar feed endpoints"),
    (name = "health", description = "Service health check")
  ),
  modifiers(&SecurityAddon),
  info(
    title = "AHE-ICS API",
    description = "Generates a subscribable ICS feed from AHE WPS schedule data."
  )
)]
pub struct ApiDoc;

#[derive(OpenApi)]
#[openapi(
  paths(
    crate::web::routes::calendar::calendar,
    crate::web::routes::calendar::calendar_me,
    crate::web::routes::health::healthz,
    crate::web::routes::openapi_json
  ),
  tags(
    (name = "calendar", description = "Calendar feed endpoints"),
    (name = "health", description = "Service health check")
  ),
  modifiers(&SecurityAddon),
  info(
    title = "AHE-ICS API",
    description = "Generates a subscribable ICS feed from AHE WPS schedule data."
  )
)]
pub struct ApiDocNoJson;

struct SecurityAddon;

impl Modify for SecurityAddon {
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    openapi.components = Some(
      utoipa::openapi::ComponentsBuilder::new()
        .security_scheme(
          "calendarTokenQuery",
          SecurityScheme::ApiKey(ApiKey::Query(ApiKeyValue::new("token"))),
        )
        .security_scheme(
          "calendarTokenHeader",
          SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-Calendar-Token"))),
        )
        .security_scheme(
          "calendarTokenBearer",
          SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
        )
        .build(),
    );
  }
}

pub fn spec_json(json_enabled: bool) -> Result<String> {
  let spec = if json_enabled {
    ApiDoc::openapi()
  } else {
    ApiDocNoJson::openapi()
  };
  Ok(serde_json::to_string(&spec)?)
}
