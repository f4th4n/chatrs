pub mod db;
pub mod message;
pub mod room;
pub mod user;

/*
use room::Room;

pub fn create_room(name: String) -> Room {
  Room::new(name.to_string())
}

pub fn send_message(room: &mut Room, from: String, message_body: String) -> () {
  room.add_message(from, message_body);
  ()
}
*/
