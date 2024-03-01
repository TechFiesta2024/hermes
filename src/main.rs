use axum::{extract::MatchedPath, http::Request, Router};
use lettre::{
    transport::smtp::{authentication::Credentials, PoolConfig},
    AsyncSmtpTransport, Tokio1Executor,
};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use hermes::{routes, shutdown, AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "hermes=debug,tower_http=debug,axum::rejection=trace,tokio=trace,runtime=trace"
                    .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mode = env::var("MODE").unwrap_or_else(|_| "development".into());

    info!("mode: {}", mode);

    let mailer: AsyncSmtpTransport<Tokio1Executor>;

    if mode == "production" {
        let username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set");
        let password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");
        let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER not set");

        let creds = Credentials::new(username, password);

        mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_server)
            .unwrap()
            .credentials(creds)
            .pool_config(PoolConfig::new().max_size(10))
            .build();

        info!("mailer configured for production mode")
    } else {
        mailer = AsyncSmtpTransport::<Tokio1Executor>::unencrypted_localhost();

        info!("mailer configured for development mode")
    }

    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:password@localhost:5432/techfiesta24")
        .await
        .unwrap();

    let app_state = AppState { pool: conn, mailer };

    let app = Router::new()
        .merge(routes::routes(app_state))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1111").await.unwrap();

    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await
        .unwrap();
}
