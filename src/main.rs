use axum::{extract::MatchedPath, http::Request, Router};
use lettre::{
    transport::smtp::{authentication::Credentials, PoolConfig},
    AsyncSmtpTransport, Tokio1Executor,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use hermes::{config::get_config, routes, shutdown, AppState};

#[tokio::main]
async fn main() {
    let config = get_config();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap())
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    let mailer: AsyncSmtpTransport<Tokio1Executor>;

    if config.mode == "production" {
        let creds = Credentials::new(config.smtp.username.clone(), config.smtp.password.clone());

        mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp.server)
            .unwrap()
            .credentials(creds)
            .pool_config(PoolConfig::new().max_size(10))
            .build();

        tracing::info!("{:?}", mailer);

        tracing::info!("mailer configured for production mode")
    } else {
        mailer = AsyncSmtpTransport::<Tokio1Executor>::unencrypted_localhost();

        tracing::info!("mailer configured for development mode")
    }

    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy_with(config.database.connection_string());

    let app_state = AppState {
        pool: conn,
        mailer,
        config,
    };

    let app = Router::new()
        .merge(routes::routes(app_state))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                tracing::info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1111").await.unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await
        .unwrap();
}
