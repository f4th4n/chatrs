use crate::header::Headers;
use crate::message_repo;
use crate::user_repo;
use chat::message::MessageForm;
use log::error;
use rocket::form::Form;
use rocket::post;
use std::error::Error;

type ResultBox<T> = Result<T, Box<dyn Error>>;

fn exec_create_chat(room_name: String, username: String, chat: String) -> ResultBox<()> {
  // let mut room = user_repo::new_room(room_name.clone())?;
  let user = user_repo::new_user(username.clone(), username.clone())?;
  let room_exist = user_repo::get_room(room_name.clone())?;
  match room_exist {
    Some(mut room) => {
      user_repo::assign_room(&room, &user)?;
      let _ = message_repo::send(&user, &mut room, chat)?;

      Ok(())
    }
    None => Err("room doesn't exist".into()),
  }
}

#[post("/chat", data = "<message_form>")]
pub fn create_chat(headers: Headers, message_form: Form<MessageForm>) -> String {
  let room_name = message_form.room_name.clone();
  let chat = message_form.chat.clone();
  let username = headers.token.clone();

  match exec_create_chat(room_name, username, chat) {
    Ok(_) => "ok".to_string(),
    Err(err) => {
      // TODO set status to 500 here
      // TODO if error is nonexistence-room then respond with 400
      error!("[req] error: {}", err);
      "err".to_string()
    }
  }
}
