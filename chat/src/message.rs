use chrono::offset::Utc;
use chrono::serde::ts_seconds;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Message {
  pub from: String,
  pub body: String,
  #[serde(with = "ts_seconds")]
  pub created_at: DateTime<chrono::Utc>,
}

impl Message {
  pub fn new(from: String, body: String) -> Self {
    Message {
      from,
      body,
      created_at: Utc::now(),
    }
  }
}
