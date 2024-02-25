use axum::{
    http::{header::HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};

use tokio_cron_scheduler::{Job, JobScheduler};

use hermes::{PingResponse, SendRequest};
use lettre::{
    message::header::ContentType, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

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
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/send", post(send));

    let scheduler = JobScheduler::new().await.unwrap();

    scheduler
        .add(
            Job::new_async("1/5 * * * * *", |uuid, mut l| {
                Box::pin(async move {
                    println!("I run async every 7 seconds");

                    let next_tick = l.next_tick_for_job(uuid).await;
                    match next_tick {
                        Ok(Some(ts)) => println!("Next time for 7s job is {:?}", ts),
                        _ => println!("Could not get next tick for 7s job"),
                    }
                })
            })
            .unwrap(),
        )
        .await
        .unwrap();

    scheduler.start().await.unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:1111")
        .await
        .unwrap();
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
    send_email(p).await;
    StatusCode::OK
}
