use chat_web::user_repo;
use dotenvy::dotenv;

fn main() {
  dotenv().expect(".env file not found");

  match user_repo::find_rooms() {
    Ok(rooms) => println!("{}", rooms.join(", ")),
    Err(err) => println!("no rooms: {}", err),
  }
}
