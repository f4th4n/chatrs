use crate::message::Message;

#[derive(PartialEq, Debug)]
pub struct Room {
  name: String,
  messages: Vec<Message>,
}

impl Room {
  pub fn new(name: String) -> Self {
    Room { name, messages: vec![] }
  }
}
