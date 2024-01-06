use chrono::DateTime;

#[derive(PartialEq, Debug)]
pub struct Message {
  message: String,
  created_at: DateTime<chrono::Utc>,
}
