use crate::header::Headers;
use crate::user_repo;
use chat::room::RoomForm;
use rocket::form::Form;
use rocket::{get, post};

#[get("/rooms")]
pub fn list_rooms() -> String {
  //let _room = user_repo::find_rooms();
  "room".to_string()
}

#[post("/rooms", data = "<room_form>")]
pub fn create_rooms(headers: Headers, room_form: Form<RoomForm>) -> RoomForm {
  let room = user_repo::new_room(room_form.name.clone()).expect("failed to create room");
  let user = user_repo::new_user(headers.token.clone(), headers.token.clone()).expect("failed to create a user");

  user_repo::assign_room(&room, &user).expect("failed assign room");

  RoomForm {
    name: room_form.name.clone(),
  }
}
