use std::net::SocketAddr;

use anyhow::Result;
use tracing_subscriber::EnvFilter;

use ahe_ics::app::AppState;
use ahe_ics::config::Config;
use ahe_ics::web::router;

#[tokio::main]
async fn main() -> Result<()> {
  dotenvy::dotenv().ok();
  let filter =
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("ahe_ics=info,axum=info"));
  tracing_subscriber::fmt().with_env_filter(filter).init();

  let config = Config::from_env()?;
  let bind_addr = config.bind_addr.clone();
  let state = AppState::new(config)?;

  let app = router(state);
  let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
  println!("listening on http://{bind_addr}");
  axum::serve(
    listener,
    app.into_make_service_with_connect_info::<SocketAddr>(),
  )
  .await?;

  Ok(())
}
