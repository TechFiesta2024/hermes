use std::sync::{Arc, Mutex};

use axum::{
  extract::State,
  http::{
    header::{HeaderMap, HeaderValue},
    StatusCode,
  },
  routing::{get, post},
  Json, Router,
};

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
  // let state = Arc::new(Mutex::new(GlobalState::new()));
  let app = Router::new()
    .route("/ping", get(ping))
    .route("/send", post(send));
  // .with_state(state);

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

async fn send(
  // State(state): State<Arc<Mutex<GlobalState>>>,
  headers: HeaderMap,
  Json(p): Json<SendRequest>,
) -> StatusCode {
  // match state.lock() {
  //   Ok(mut state) => match state.send_json(&p) {
  //     Err(e) => {
  //       println!("Error: {}", e);
  //       StatusCode::INTERNAL_SERVER_ERROR
  //     }
  //     _ => StatusCode::OK,
  //   },
  //   Err(e) => {
  //     println!("{}", e);
  //     StatusCode::INTERNAL_SERVER_ERROR
  //   }
  // }
  let apikey = headers.get("api-key").unwrap().to_str().unwrap();
  println!("body -> {p}");
  println!("api-key -> {apikey}");
  send_email(p).await;
  StatusCode::OK
}
