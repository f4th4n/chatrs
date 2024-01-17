use chat::message::Message;
use chat::room::Room;
use chat::user::User;
use log::{debug, info};
use redis::{Client, Commands, Connection};
use std::error::Error;

type ResultBox<T> = Result<T, Box<dyn Error>>;

static ROOM_PREFIX: &str = "chatrs:message:";

// LRANGE chatrs:message:room_1 0 -1

fn get_connection() -> ResultBox<Connection> {
  let redis_url = dotenvy::var("REDIS_URL").expect("REDIS_URL is must");
  let client = Client::open(redis_url)?;
  let conn = client.get_connection()?;
  Ok(conn)
}

fn msg_key(room_name: String) -> String {
  format!("{ROOM_PREFIX}{room_name}")
}

/// assign room name to user
pub fn send(user: &User, room: &mut Room, msg_body: String) -> ResultBox<()> {
  let msg = Message::new(user.username.clone(), msg_body.clone());
  let mut conn = get_connection()?;

  room.add_message(msg);

  let key = msg_key(room.name.clone());
  let value = serde_json::to_string(&room.messages)?;
  info!("[message] new message in room `{}`", key.clone());
  debug!("[message] detail message`{}`", value);

  conn.lpush(key, value)?;

  Ok(())
}
