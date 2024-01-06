use chat;

fn main() {
  let mut room = chat::create_room("room:1".to_string());
  chat::send_message(&mut room, "acc1".to_string(), "hi world".to_string());
  chat::send_message(&mut room, "acc2".to_string(), "hi world".to_string());

  println!("Hello, cli");
}
