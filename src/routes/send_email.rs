use axum::{extract::State, http::StatusCode, Json};
use lettre::{message::SinglePart, AsyncTransport, Message};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct SendEmailBody {
    name: String,
    email: String,
    subject: String,
    email_body: String,
}

pub async fn send_email(
    State(app): State<AppState>,
    Json(body): Json<SendEmailBody>,
) -> StatusCode {
    let to = format!("{} <{}>", body.name, body.email);

    tracing::info!("Sending email to: {}", to);

    let email = Message::builder()
        .from(
            format!("TechFiesta Team <{}>", app.config.smtp.username)
                .parse()
                .unwrap(),
        )
        .to(to.parse().unwrap())
        .subject(body.subject)
        .singlepart(SinglePart::html(body.email_body))
        .unwrap();

    tokio::spawn(async move {
        match app.mailer.send(email).await {
            Ok(_) => tracing::info!("Email sent successfully"),
            Err(e) => tracing::error!("Error: {}", e),
        }
    });

    StatusCode::OK
}
