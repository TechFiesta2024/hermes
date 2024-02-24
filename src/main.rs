use axum::{
    routing::{post, get},
    http::StatusCode,
    Json, Router
};

// use serde::{Serialize, Deserialize};
use hermes::SendRequest;
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/send", post(send));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:1111")
        .await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct PingResponse {
    ok: bool,
    msg: String
}

async fn ping() -> Json<PingResponse> {
    Json(PingResponse {
        ok: true,
        msg: "Hello World".to_string()
    })
}

async fn send(Json(p): Json<SendRequest>) -> StatusCode {
    println!("{}", p);
    StatusCode::OK
}