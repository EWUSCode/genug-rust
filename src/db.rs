use bb8::Pool;
use bb8_oracle::Error;
use color_eyre::{eyre::Context, Result};
use tracing::{debug, error, info};

pub(crate) type ConnectionPool = Pool<bb8_oracle::OracleConnectionManager>;

#[derive(Debug, Clone, Copy)]
struct OraErrorSink;

impl bb8::ErrorSink<Error> for OraErrorSink {
    fn sink(&self, error: Error) {
        error!("Database error: {}", error);
    }

    fn boxed_clone(&self) -> Box<dyn bb8::ErrorSink<Error>> {
        debug!("Cloning ErrorSink");
        Box::new(*self)
    }
}

pub async fn setup_connection_pool() -> Result<ConnectionPool> {
    use std::time::Duration;

    info!("Using Oracle database");
    // set up connection pool
    let manager =
        bb8_oracle::OracleConnectionManager::new("smith", "S94dDMHs", "ora:1521/FREEPDB1");
    Pool::builder()
        .connection_timeout(Duration::from_secs(3))
        .error_sink(Box::new(OraErrorSink))
        .build(manager)
        .await
        .context("Connection to Oracle failed")
}
