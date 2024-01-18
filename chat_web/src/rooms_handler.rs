use crate::header::Headers;
use crate::user_repo;
use chat::room::RoomForm;
use log::error;
use rocket::form::Form;
use rocket::{get, post};
use std::error::Error;

type ResultBox<T> = Result<T, Box<dyn Error>>;

#[get("/rooms")]
pub fn list_rooms() -> String {
  //let _room = user_repo::find_rooms();
  "room".to_string()
}

fn exec_create_room(room_name: String, username: String) -> ResultBox<()> {
  let room = user_repo::new_room(room_name.clone())?;
  let user = user_repo::new_user(username.clone(), username.clone())?;

  user_repo::assign_room(&room, &user)?;
  Ok(())
}

#[post("/rooms", data = "<room_form>")]
pub fn create_room(headers: Headers, room_form: Form<RoomForm>) -> String {
  let room_name = room_form.name.clone();
  let username = headers.token.clone();
  match exec_create_room(room_name, username) {
    Ok(_) => "ok".to_string(),
    Err(err) => {
      error!("[req] error: {}", err);
      "err".to_string()
    }
  }
}
