use dotenvy;
use redis::Client;
use redis::Commands;
use redis::Connection;

/*
redis keys:
GET user:abc
*/

fn get_connection() -> Connection {
  let redis_url = match dotenvy::var("REDIS_URL") {
    Ok(url) => url,
    _error => panic!("REDIS_URL is required"),
  };

  let client = match Client::open(redis_url) {
    Ok(url) => url,
    _error => panic!("Cannot connect to redis server"),
  };

  let conn = match client.get_connection() {
    Ok(conn) => conn,
    _error => panic!("Cannot get connection to redis server"),
  };

  conn
}

//fn find_rooms() -> Vec<Room> {}
pub fn find_rooms() -> String {
  let mut con = get_connection();
  let key = "chatrs".to_string();
  let result: Option<String> = con.get(key).expect("Failed to get value from Redis");

  match result {
    Some(value) => value,
    None => "not found".to_string(),
  }
}
