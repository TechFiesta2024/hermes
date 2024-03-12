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
    workshop_event_name: String,
    subject: String,
    email_body: String,
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
    if matches!(
        body.workshop_event_name.as_str(),
        "product_design" | "ctf" | "cad" | "hardware"
    ) {
        tracing::info!("Sending email to: {}", body.workshop_event_name);

        let rows: Vec<UserInfo> = sqlx::query_as(&format!("SELECT name, email FROM workshop left join college_users on user_email = email where category = '{}'",body.workshop_event_name))
            .fetch_all(&app.pool)
            .await
            .unwrap();

        if rows.is_empty() {
            return StatusCode::NOT_FOUND;
        }

        tokio::spawn(async move {
            send_email(
                app.mailer,
                rows,
                body.subject,
                body.email_body,
                app.config.smtp.username,
            )
            .await;
        });

        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}

pub async fn send_email(
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    address: Vec<UserInfo>,
    subject: String,
    mail_body: String,
    username: String,
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
        .from(format!("TechFiesta Team <{}>", username).parse().unwrap())
        .subject(subject)
        .singlepart(SinglePart::html(mail_body))
        .unwrap();

    match mailer.send(email).await {
        Ok(_) => tracing::info!("Email sent successfully"),
        Err(e) => tracing::error!("Error: {}", e),
    }
}
