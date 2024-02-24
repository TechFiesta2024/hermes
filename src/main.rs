use std::sync::{Arc, Mutex};


use axum::{
  extract::State,
  http::StatusCode,
  routing::{get, post},
  Json, Router,
};

use hermes::{GlobalState, PingResponse, SendRequest};

#[tokio::main]
async fn main() {
  let state = Arc::new(Mutex::new(GlobalState::new()));
  let app = Router::new()
    .route("/ping", get(ping))
    .route("/send", post(send))
    .with_state(state);

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
  State(state): State<Arc<Mutex<GlobalState>>>,
  Json(p): Json<SendRequest>,
) -> StatusCode {
  match state.lock() {
    Ok(mut state) => match state.send_json(&p) {
      Err(e) => {
        println!("Error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
      }
      _ => StatusCode::OK,
    },
    Err(e) => {
      println!("{}", e);
      StatusCode::INTERNAL_SERVER_ERROR
    }
  }
}
