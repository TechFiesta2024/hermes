use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize)]
pub struct PingResponse {
    pub ok: bool,
    pub msg: String,
}

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
pub struct SendRequest {
    pub from: Identity,
    pub to: Identity,
    pub subject: String,
    pub body: String,
}

impl Display for SendRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}\nsubject: {}\nbody: {}",
            self.from, self.to, self.subject, self.body
        )
    }
}
