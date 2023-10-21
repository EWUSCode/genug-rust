mod auth;
mod health;
mod hello;
pub(crate) mod state;

use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use axum::{routing::get, Router};
use color_eyre::{eyre::Context, Result};
use tower_http::trace::TraceLayer;

use crate::AppState;

use self::hello::{get_hello, post_hello};

pub async fn serve(port: u16, app_state: AppState) -> Result<()> {
    let app = api_routes(app_state)
        .await?
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::new(IpAddr::from_str("::")?, port);
    tracing::info!("listening on {}", addr);
    axum::Server::try_bind(&addr)
        .wrap_err_with(|| format!("Failed to start server on {addr}"))?
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .wrap_err_with(|| format!("Error while serving"))
}

async fn api_routes(app_state: AppState) -> Result<Router> {
    let auth: jwt_authorizer::layer::AsyncAuthorizationLayer<auth::User> =
        auth::auth_layer("http://localhost:8101/realms/rodat").await?;
    Ok(Router::new()
        .nest(
            "/app",
            Router::new()
                .route("/up", get(|| async { "OK" }))
                .route("/health", get(health::health)),
        )
        .nest(
            "/api",
            Router::new().route("/hello", get(get_hello).post(post_hello)),
        )
        .layer(auth)
        .with_state(app_state))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}
