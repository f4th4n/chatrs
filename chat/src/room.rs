use crate::message::Message;

/// rules: message name for 2 persons will be 'private:person_1:person_2'
///   where person_1 and person_2 is username, and it is always sorted ascending
#[derive(PartialEq, Debug)]
pub struct Room {
  name: String,
  messages: Vec<Message>,
}

impl Room {
  pub fn new(name: String) -> Self {
    Room { name, messages: vec![] }
  }

  pub fn add_message(&mut self, from: String, message_body: String) -> &Self {
    let msg = Message::new(from, message_body);
    let _ = &self.messages.push(msg);
    self
  }
}
