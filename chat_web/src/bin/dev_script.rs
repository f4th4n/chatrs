use chat::room::Room;
use chat::user::User;
use chat_web::user_repo;
use dotenvy::dotenv;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  dotenv().expect(".env file not found");

  let user1 = User {
    username: "wildan".to_string(),
  };
  let rooms_list = vec![Room::new("room1".to_string()), Room::new("room2".to_string())];
  user_repo::set_rooms(&user1, rooms_list)?;

  let rooms = user_repo::find_rooms(&user1)?;
  println!("rooms {:?}", rooms);

  Ok(())
}
