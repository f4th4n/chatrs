//use crate::user_repo;
use rocket::get;
use rocket::post;

#[get("/rooms")]
pub fn list_rooms() -> String {
  //let _room = user_repo::find_rooms();
  "room".to_string()
}

#[post("/rooms")]
pub fn create_rooms() -> String {
  //let _room = user_repo::find_rooms();
  "room".to_string()
}
