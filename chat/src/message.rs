use chrono::DateTime;

#[derive(PartialEq, Debug)]
pub struct Message {
  from: String,
  body: String,
  created_at: DateTime<chrono::Utc>,
}

impl Message {
  pub fn new(from: String, body: String) -> Self {
    Message {
      from,
      body,
      created_at: chrono::offset::Utc::now(),
    }
  }
}
