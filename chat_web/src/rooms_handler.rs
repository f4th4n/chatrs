use crate::user_repo;

#[get("/rooms")]
pub fn rooms_index() -> String {
  let room = user_repo::find_rooms();
  room
}
