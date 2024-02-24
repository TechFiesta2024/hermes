use std::fmt::Display;

use serde::Deserialize;


#[derive(Deserialize)]
pub struct Identity {
  pub name: String,
  pub email: String,
}

impl Display for Identity {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "({}, {})", self.name, self.email)
  }
}

#[derive(Deserialize)]
pub struct SendRequest {
  pub from: Identity,
  pub to: Identity
}

impl Display for SendRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{} -> {}", self.from, self.to)
  }
}