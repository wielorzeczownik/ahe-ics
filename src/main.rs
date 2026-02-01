use anyhow::Result;

mod api;
mod app;
mod cache;
mod config;
mod constants;
mod i18n;
mod ics;
mod models;
mod web;

use crate::app::AppState;
use crate::config::Config;
use crate::web::router;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
  dotenvy::dotenv().ok();
  let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_|
    EnvFilter::new("ahe_ics=info,axum=info")
  );
  tracing_subscriber::fmt().with_env_filter(filter).init();

  let config = Config::from_env()?;
  let bind_addr = config.bind_addr.clone();
  let state = AppState::new(config)?;

  let app = router(state);
  let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
  println!("listening on http://{bind_addr}");
  axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

  Ok(())
}
