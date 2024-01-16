use chat::message::Message;
use chat::room::Room;
use chat::user::User;
use redis::Client;
use redis::Commands;
use redis::Connection;
use std::error::Error;

type ResultBox<T> = Result<T, Box<dyn Error>>;

static ROOM_PREFIX: &str = "chatrs:message:";

fn get_connection() -> ResultBox<Connection> {
  let redis_url = dotenvy::var("REDIS_URL").expect("REDIS_URL is must");
  let client = Client::open(redis_url)?;
  let conn = client.get_connection()?;
  Ok(conn)
}

/// assign room name to user
pub fn send(user: &User, room: &mut Room, msg_body: String) -> ResultBox<()> {
  let msg = Message::new(user.username.clone(), msg_body.clone());
  let mut conn = get_connection()?;
  let room_name = &room.name.clone();

  room.add_message(msg);

  let key = format!("{ROOM_PREFIX}{room_name}");
  let value = serde_json::to_string(&room.messages)?;
  println!("new message in {}", key.clone());
  //println!("message {}", value.clone());

  conn.lpush(key, value)?;

  Ok(())
}
