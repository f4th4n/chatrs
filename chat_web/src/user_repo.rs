use chat::room::Room;
use chat::user::User;
use dotenvy;
use log::info;
use redis::Client;
use redis::Commands;
use redis::Connection;
use std::error::Error;

/*
redis keys:
GET user:abc
*/

type ResultBox<T> = Result<T, Box<dyn Error>>;

static ROOM_PREFIX: &str = "chatrs:room:";

fn get_connection() -> ResultBox<Connection> {
  let redis_url = dotenvy::var("REDIS_URL").expect("REDIS_URL is must");
  let client = Client::open(redis_url)?;
  let conn = client.get_connection()?;
  Ok(conn)
}

pub fn new_room(room_name: String) -> ResultBox<Room> {
  let room = Room::new(room_name);
  let mut conn = get_connection()?;
  conn.sadd("chatrs:rooms", vec![&room.name.clone()])?;
  info!("[room] new room `{}`", room.name.clone());
  Ok(room)
}

pub fn new_user(username: String, name: String) -> ResultBox<User> {
  let user = User {
    username: username.clone(),
    name: name.clone(),
  };
  let mut conn = get_connection()?;
  conn.hset(user.username.clone(), "username".to_string(), username.clone())?;
  conn.hset(user.name.clone(), "name".to_string(), name)?;
  info!("[user] new user `{}`", username.clone());
  Ok(user)
}

/// push user to room
pub fn assign_room(room: &Room, user: &User) -> ResultBox<()> {
  let mut conn = get_connection()?;
  //let prefix: String = "chatrs:room:".to_owned();
  let room_name: &String = &room.name.clone();
  let key = format!("{ROOM_PREFIX}{room_name}");

  conn.sadd(key, vec![&user.username.clone()])?;

  Ok(())
}

pub fn users_by_room(room: &Room) -> ResultBox<Vec<String>> {
  let mut conn = get_connection()?;
  let room_name: &String = &room.name.clone();
  let key = format!("{ROOM_PREFIX}{room_name}");
  let result: Vec<_> = conn.smembers(key).expect("redis cmd err");
  Ok(result)
}

pub fn find_rooms(user: &User) -> ResultBox<Vec<String>> {
  let mut conn = get_connection()?;
  let result: Vec<_> = conn.smembers(user.username.clone()).expect("redis cmd err");
  Ok(result)
}
