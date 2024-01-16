use chat_web::message_repo;
use chat_web::user_repo;
use dotenvy::dotenv;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  dotenv().expect(".env file not found");

  let mut room_1 = user_repo::new_room("room_1".to_string())?;
  let room_2 = user_repo::new_room("room_2".to_string())?;

  let user_1 = user_repo::new_user("wildan".to_string(), "Wildan Fathan".to_string())?;
  let user_2 = user_repo::new_user("kalinga".to_string(), "Kalingga Satria".to_string())?;

  user_repo::assign_room(&room_1, &user_1)?;
  user_repo::assign_room(&room_1, &user_2)?;
  user_repo::assign_room(&room_2, &user_1)?;

  let users_room_1 = user_repo::users_by_room(&room_1)?;
  println!(".... {:?}", users_room_1);

  let _ = message_repo::send(&user_1, &mut room_1, "hello".to_string())?;
  let _ = message_repo::send(&user_1, &mut room_1, "world".to_string())?;

  Ok(())
}
