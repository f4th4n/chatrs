use chat::room::Room;
use chat::user::User;
use dotenvy;
use redis::Client;
use redis::Commands;
use redis::Connection;
use std::error::Error;

/*
redis keys:
GET user:abc
*/

type ResultBox<T> = Result<T, Box<dyn Error>>;

fn get_connection() -> ResultBox<Connection> {
  let redis_url = dotenvy::var("REDIS_URL").expect("REDIS_URL is must");
  let client = Client::open(redis_url)?;
  let conn = client.get_connection()?;
  Ok(conn)
}

pub fn set_rooms(user: &User, rooms: Vec<Room>) -> ResultBox<()> {
  let username = &user.username;
  let rooms_name: Vec<_> = rooms.iter().map(|room| room.name.clone()).collect();
  let mut conn = get_connection()?;
  conn.del(username.clone())?;
  conn.lpush(username.clone(), rooms_name)?;
  Ok(())
}

pub fn find_rooms(user: &User) -> ResultBox<Vec<String>> {
  let mut conn = get_connection()?;
  let result: Vec<_> = conn.lrange(user.username.clone(), 0, -1).expect("redis cmd err");
  Ok(result)
}
