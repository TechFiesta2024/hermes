use axum::{
    extract::{MatchedPath, State},
    http::{HeaderMap, Request, StatusCode},
    routing::{get, post},
    Json, Router,
};
use std::env;

// use tokio_cron_scheduler::JobScheduler;
use lettre::{
    transport::smtp::{authentication::Credentials, PoolConfig},
    AsyncSmtpTransport, Tokio1Executor,
};

use tower_http::trace::TraceLayer;

use hermes::PingResponse;
use tracing::{info, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use hermes::{
    email::{send_email, Email},
    // scheduler::job,
    shutdown,
};

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

    if mode == "development" {
        mailer = AsyncSmtpTransport::<Tokio1Executor>::unencrypted_localhost();
    } else {
        let username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set");
        let password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");
        let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER not set");

        let creds = Credentials::new(username, password);

        mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_server)
            .unwrap()
            .credentials(creds)
            .pool_config(PoolConfig::new().max_size(10))
            .build();
    }

    info!("mailer created");

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/send", post(send))
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
        .with_state(mailer);

    // let scheduler = JobScheduler::new().await.unwrap();
    //
    // scheduler.add(job()).await.unwrap();
    //
    // scheduler.start().await.unwrap();
    // info!("scheduler started");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1111").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await
        .unwrap();
}

async fn ping() -> Json<PingResponse> {
    Json(PingResponse {
        ok: true,
        msg: "Hello World".to_string(),
    })
}

async fn send(
    headers: HeaderMap,
    State(mailer): State<AsyncSmtpTransport<Tokio1Executor>>,
    Json(p): Json<Email>,
) -> StatusCode {
    let apikey = headers.get("api-key").unwrap().to_str().unwrap();
    println!("body -> {p}");
    println!("api-key -> {apikey}");
    send_email(p, mailer).await;
    StatusCode::OK
}
