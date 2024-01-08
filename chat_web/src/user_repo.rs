/*
GET user:abc
*/

fn _get_connection() -> redis::RedisResult<()> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;
  let _: () = redis::cmd("SET").arg("my_key2").arg(42).query(&mut con)?;

  Ok(())
}

//fn find_rooms() -> Vec<Room> {}
pub fn find_rooms() -> String {
  "yoman".to_string()
}
