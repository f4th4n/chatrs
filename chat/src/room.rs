use crate::message::Message;
use rocket::FromForm;
use rocket::Responder;
use serde::{Deserialize, Serialize};

/// rules: message name for 2 persons will be 'private:person_1:person_2'
///   where person_1 and person_2 is username, and it is always sorted ascending
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Room {
  pub name: String,
  pub messages: Vec<Message>,
}

#[derive(Debug, FromForm, Responder)]
#[response(status = 200, content_type = "json")]
pub struct RoomForm {
  pub name: String,
}

impl Room {
  pub fn new(name: String) -> Self {
    Room { name, messages: vec![] }
  }

  pub fn add_message(&mut self, msg: Message) -> &Self {
    let _ = &self.messages.push(msg);
    self
  }
}
