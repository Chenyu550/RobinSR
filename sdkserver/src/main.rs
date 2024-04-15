use anyhow::Result;
use axum::extract::Request;
use axum::routing::{get, post};
use axum::{Router, ServiceExt};
use services::{auth, dispatch, errors};
use tokio::net::TcpListener;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;
use tracing::Level;

mod config;
mod logging;
mod services;

use config::init_config;
use logging::init_tracing;

const PORT: u16 = 21000;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    init_config();

    let span = tracing::span!(Level::DEBUG, "main");
    let _ = span.enter();

    let app = Router::new()
        .route(
            dispatch::QUERY_DISPATCH_ENDPOINT,
            get(dispatch::query_dispatch),
        )
        .route(
            dispatch::QUERY_GATEWAY_ENDPOINT,
            get(dispatch::query_gateway),
        )
        .route(auth::RISKY_API_CHECK_ENDPOINT, post(auth::risky_api_check))
        .route(
            auth::LOGIN_WITH_PASSWORD_ENDPOINT,
            post(auth::login_with_password),
        )
        .route(
            auth::LOGIN_WITH_SESSION_TOKEN_ENDPOINT,
            post(auth::login_with_session_token),
        )
        .route(
            auth::GRANTER_LOGIN_VERIFICATION_ENDPOINT,
            post(auth::granter_login_verification),
        )
        .fallback(errors::not_found);

    let app = NormalizePathLayer::trim_trailing_slash().layer(app);

    let addr = format!("0.0.0.0:{PORT}");
    let server = TcpListener::bind(&addr).await?;

    tracing::info!("sdkserver is listening at {addr}");
    axum::serve(server, ServiceExt::<Request>::into_make_service(app)).await?;

    Ok(())
}
