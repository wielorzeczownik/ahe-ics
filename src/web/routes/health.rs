use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::warn;

use crate::app::AppState;

#[utoipa::path(
  get,
  path = "/healthz",
  tag = "health",
  responses((
    status = 204,
    description = "Service is healthy",
  ), (
    status = 503,
    description = "Upstream API unavailable",
    body = String,
    content_type = "text/plain",
  ))
)]
pub(crate) async fn healthz(State(state): State<AppState>) -> impl IntoResponse {
  let token = match state
    .token_cache
    .get_or_login(&state.config, &state.api)
    .await
  {
    Ok(token) => token,
    Err(error) => {
      warn!(error = %error, "health check login failed");
      return (StatusCode::SERVICE_UNAVAILABLE, "upstream login failed");
    }
  };

  if let Err(error) = state.api.get_student_data(&token).await {
    warn!(error = %error, "health check student data failed");
    return (StatusCode::SERVICE_UNAVAILABLE, "upstream api unavailable");
  }

  (StatusCode::NO_CONTENT, "")
}
