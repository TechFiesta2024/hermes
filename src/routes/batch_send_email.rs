use axum::{extract::State, http::StatusCode, Json};
use lettre::{
    message::{header, Mailboxes, MessageBuilder, SinglePart},
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};
use serde::Deserialize;
use sqlx::FromRow;

use crate::AppState;

#[derive(Deserialize)]
pub struct BatchSendEmailBody {
    workshop_event_name: WorkShopsEvents,
    subject: String,
    email_body: String,
}

#[derive(Deserialize)]
enum WorkShopsEvents {
    ProductDesign,
    CAD,
    CTF,
    Hardware,
}

#[derive(FromRow, Debug)]
pub struct UserInfo {
    pub name: String,
    pub email: String,
}

pub async fn batch_send_email(
    State(app): State<AppState>,
    Json(body): Json<BatchSendEmailBody>,
) -> StatusCode {
    match body.workshop_event_name {
        WorkShopsEvents::ProductDesign => {
            tracing::info!("Sending email to: ProductDesign");

            let rows: Vec<UserInfo> = sqlx::query_as("SELECT name, email FROM workshop_product")
                .fetch_all(&app.pool)
                .await
                .unwrap();

            send_email(app.mailer, rows, body.subject, body.email_body).await;
        }
        WorkShopsEvents::CAD => {
            tracing::info!("Sending email to: CAD");

            let rows: Vec<UserInfo> = sqlx::query_as("SELECT name, email FROM workshop_cad")
                .fetch_all(&app.pool)
                .await
                .unwrap();

            send_email(app.mailer, rows, body.subject, body.email_body).await;
        }
        WorkShopsEvents::CTF => {
            tracing::info!("Sending email to: CTF");

            let rows: Vec<UserInfo> = sqlx::query_as("SELECT name, email FROM workshop_ctf")
                .fetch_all(&app.pool)
                .await
                .unwrap();

            send_email(app.mailer, rows, body.subject, body.email_body).await;
        }
        WorkShopsEvents::Hardware => {
            tracing::info!("Sending email to: Hardware");

            let rows: Vec<UserInfo> = sqlx::query_as("SELECT name, email FROM workshop_hardware")
                .fetch_all(&app.pool)
                .await
                .unwrap();

            send_email(app.mailer, rows, body.subject, body.email_body).await;
        }
    }

    StatusCode::OK
}

pub async fn send_email(
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    address: Vec<UserInfo>,
    subject: String,
    mail_body: String,
) {
    let to_addresser = address
        .iter()
        .map(|user| format!("{} <{}>", user.name, user.email))
        .collect::<Vec<String>>()
        .join(", ");

    tracing::info!("Sending email to: {}", to_addresser);

    let mailboxes: Mailboxes = to_addresser.parse().unwrap();

    let to_header: header::Bcc = mailboxes.into();

    let email = MessageBuilder::new()
        .mailbox(to_header)
        .from("Tech Team <no-reply@localhost.com>".parse().unwrap())
        .subject(subject)
        .singlepart(SinglePart::html(mail_body))
        .unwrap();

    match mailer.send(email).await {
        Ok(_) => tracing::info!("Email sent successfully"),
        Err(e) => tracing::error!("Error: {}", e),
    }
}
