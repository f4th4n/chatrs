#[macro_use]
extern crate rocket;

use chat_web::messages_handler;
use chat_web::rooms_handler;
use dotenvy::dotenv;
use rocket::response::content;

#[get("/")]
fn index() -> &'static str {
  "ok!"
}

#[catch(415)]
fn unsupported_media() -> content::RawJson<&'static str> {
  content::RawJson("{ \"error\": \"Unsupported Media Type\" }")
}

#[catch(422)]
fn unprocessable_entity() -> content::RawJson<&'static str> {
  content::RawJson("{ \"error\": \"Unprocessable Entity\" }")
}

#[launch]
fn rocket() -> _ {
  dotenv().expect(".env file not found");

  rocket::build()
    .register("/", catchers![unsupported_media, unprocessable_entity])
    .mount(
      "/",
      routes![
        index,                         // /
        messages_handler::create_chat, // POST /chat
        rooms_handler::list_rooms,     // GET /rooms
        rooms_handler::create_room,    // POST /rooms
      ],
    )
}
