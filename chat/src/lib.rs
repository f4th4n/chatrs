mod message;
mod room;

use room::Room;

pub fn create_room(name: String) -> Room {
  Room::new(name.to_string())
}
