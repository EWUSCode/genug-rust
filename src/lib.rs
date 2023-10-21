use axum::http::StatusCode;
use tracing::error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod api;
pub mod db;

pub use api::state::AppState;

pub fn prepare_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "myservice1=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Utility function for mapping any error into
/// a `500 Internal Server Error` response.
pub(crate) fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    error!("Internal server error: {}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
