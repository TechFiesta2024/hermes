use axum::{
    extract::MatchedPath,
    http::{HeaderMap, Request, StatusCode},
    routing::{get, post},
    Json, Router,
};

use tokio_cron_scheduler::{Job, JobScheduler};
use tower_http::trace::TraceLayer;

use hermes::{Identity, PingResponse, SendRequest};
use lettre::{
    message::header::ContentType, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use tracing::{info, info_span};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn send_email(send_request: SendRequest) {
    let email = Message::builder()
        .from(send_request.from.email.parse().unwrap())
        .reply_to(send_request.from.email.parse().unwrap())
        .to(send_request.to.email.parse().unwrap())
        .subject(send_request.subject)
        .header(ContentType::TEXT_HTML)
        .body(send_request.body)
        .unwrap();

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::unencrypted_localhost();

    match mailer.send(email).await {
        Ok(_) => println!("Email sent successfully"),
        Err(e) => println!("Error: {}", e),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hermes=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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

    scheduler
        .add(
            Job::new_async("1/10 * * * * *", |_uuid, _l| {
                Box::pin(async move {
                    // let p = SendRequest {
                    //     from: Identity {
                    //         name: "Hermes".to_string(),
                    //         email: "hermes@localhost".to_string(),
                    //     },
                    //     to: Identity {
                    //         name: "Hermes".to_string(),
                    //         email: "hermes@localhost".to_string(),
                    //     },
                    //     subject: "Hello".to_string(),
                    //     body: "Hello".to_string(),
                    // };
                    // send_email(p).await;
                    info!("send mail");
                })
            })
            .unwrap(),
        )
        .await
        .unwrap();

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

async fn send(headers: HeaderMap, Json(p): Json<SendRequest>) -> StatusCode {
    let apikey = headers.get("api-key").unwrap().to_str().unwrap();
    println!("body -> {p}");
    println!("api-key -> {apikey}");
    // send_email(p).await;
    StatusCode::OK
}
