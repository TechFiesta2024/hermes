use kafka::{
  client::RequiredAcks,
  producer::{Producer, Record},
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::time::Duration;

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

// pub struct GlobalState {
//   pub producer: Producer,
// }
//
// impl GlobalState {
//   pub fn new() -> Self {
//     let producer = Producer::from_hosts(vec!["localhost:9092".to_owned()])
//       .with_ack_timeout(Duration::from_millis(500))
//       .with_required_acks(RequiredAcks::One)
//       .create()
//       .unwrap();
//
//     Self { producer }
//   }
//
//   pub fn send_json<T: Serialize>(self: &mut Self, data: &T) -> Result<(), Box<dyn Error>>{
//     let s = serde_json::to_string(&data)?;
//     let r = Record::from_value("", s.as_bytes());
//     self.producer.send(&r)?;
//     Ok(())
//   }
// }
