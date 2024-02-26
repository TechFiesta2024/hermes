use lettre::{
    message::header::ContentType, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
        Message::builder()
            .from(self.from.email.parse().unwrap())
            .reply_to(self.from.email.parse().unwrap())
            .to(self.to.email.parse().unwrap())
            .subject(self.subject.clone())
            .header(ContentType::TEXT_HTML)
            .body(self.body.clone())
            .unwrap()
    }
}

pub async fn send_email(email: Email) {
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::unencrypted_localhost();

    match mailer.send(email.to_message()).await {
        Ok(_) => println!("Email sent successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
