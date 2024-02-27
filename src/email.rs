use lettre::{
    message::header::ContentType, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tracing::{error, info};

#[derive(Serialize, Deserialize)]
pub struct Identity {
    pub name: String,
    pub email: String,
}

impl Display for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.name, self.email)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Email {
    pub from: Identity,
    pub to: Identity,
    pub subject: String,
    pub body: String,
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}\nsubject: {}\nbody: {}",
            self.from, self.to, self.subject, self.body
        )
    }
}

impl Email {
    pub fn to_message(&self) -> Message {
        let from_email = format!("{} <{}>", self.from.name, self.from.email);
        let to_email = format!("{} <{}>", self.to.name, self.to.email);
        Message::builder()
            .from(from_email.parse().unwrap())
            .reply_to(from_email.parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject(self.subject.clone())
            .header(ContentType::TEXT_HTML)
            .body(self.body.clone())
            .unwrap()
    }
}

pub async fn send_email(email: Email, mailer: AsyncSmtpTransport<Tokio1Executor>) {
    match mailer.send(email.to_message()).await {
        Ok(_) => info!("Email sent successfully"),
        Err(e) => error!("Error: {}", e),
    }
}
