use axum::{
    body::Body,
    extract::{MatchedPath, State},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Json, Router,
};
use lettre::{
    transport::smtp::{authentication::Credentials, PoolConfig},
    AsyncSmtpTransport, Tokio1Executor,
};
use std::env;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use hermes::PingResponse;
use hermes::{
    email::{send_email, Email},
    shutdown,
};

async fn verify_key(req: Request<Body>, next: Next) -> Result<Response, Response> {
    let (parts, body) = req.into_parts();
    let api_key = env::var("API_KEY").unwrap();
    let headers = parts.headers.get("x-api-key");

    match headers {
        Some(key) => {
            if key.to_str().unwrap() != api_key {
                return Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::empty())
                    .unwrap());
            }
        }
        None => {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::empty())
                .unwrap());
        }
    }

    let new_req = Request::from_parts(parts, body);
    Ok(next.run(new_req).await)
}

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
    } else {
        mailer = AsyncSmtpTransport::<Tokio1Executor>::unencrypted_localhost();
    }

    info!("mailer created");

    let send_email_router = Router::new()
        .route("/send", post(send))
        .layer(middleware::from_fn(verify_key));

    let app = Router::new()
        .route("/health_check", get(ping))
        .merge(send_email_router)
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
        .layer(CorsLayer::permissive())
        .with_state(mailer);

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
    State(mailer): State<AsyncSmtpTransport<Tokio1Executor>>,
    Json(p): Json<Email>,
) -> StatusCode {
    send_email(p, mailer).await;
    StatusCode::OK
}
