use color_eyre::{eyre::Context, Result};
use myservice1::AppState;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let _ = dotenvy::dotenv();
    myservice1::prepare_logging();
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .context("Cannot parse PORT")?;
    debug!("Hello, world!");
    let pool = myservice1::db::setup_connection_pool().await?;
    pool.get().await?;
    let app_state = AppState::new(pool);
    myservice1::api::serve(port, app_state).await?;
    Ok(())
}
