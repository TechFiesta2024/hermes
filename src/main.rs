use axum::{
    extract::MatchedPath,
    http::{HeaderMap, Request, StatusCode},
    routing::{get, post},
    Json, Router,
};

use tokio_cron_scheduler::JobScheduler;
use tower_http::trace::TraceLayer;

use hermes::PingResponse;
use sqlx::postgres::PgPoolOptions;
use tracing::{info, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use hermes::{
    email::{send_email, Email},
    scheduler::job,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hermes=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    //     let pool = PgPoolOptions::new()
    //         .max_connections(7)
    //         .connect(
    //             "postgresql://postgres:password@loc
    // alhost:5432/techfiesta24",
    //         )
    //         .await
    //         .unwrap();

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
        );

    let scheduler = JobScheduler::new().await.unwrap();

    scheduler.add(job()).await.unwrap();

    scheduler.start().await.unwrap();
    info!("scheduler started");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1111").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn ping() -> Json<PingResponse> {
    Json(PingResponse {
        ok: true,
        msg: "Hello World".to_string(),
    })
}

async fn send(headers: HeaderMap, Json(p): Json<Email>) -> StatusCode {
    let apikey = headers.get("api-key").unwrap().to_str().unwrap();
    println!("body -> {p}");
    println!("api-key -> {apikey}");
    send_email(p).await;
    StatusCode::OK
}
