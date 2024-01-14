use dotenvy;
use redis::Client;
use redis::Commands;
use redis::Connection;
use std::error::Error;

/*
redis keys:
GET user:abc
*/

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn get_connection() -> Result<Connection> {
  let redis_url = dotenvy::var("REDIS_URL").expect("REDIS_URL is must");
  let client = Client::open(redis_url)?;
  let conn = client.get_connection()?;
  Ok(conn)
}

pub fn find_rooms() -> Result<Vec<String>> {
  let mut conn = get_connection()?;
  //let result: Option<String> = conn.get("chatrs".to_string())?;
  let list_name = "bikes:repairs".to_string();
  let result: Vec<String> = conn.lrange(list_name, 0, -1).expect("failed to execute LRANGE for 'items'");
  Ok(result)
}
