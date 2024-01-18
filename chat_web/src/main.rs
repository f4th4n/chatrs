#[macro_use]
extern crate rocket;

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

#[launch]
fn rocket() -> _ {
  dotenv().expect(".env file not found");

  rocket::build().register("/", catchers![unsupported_media]).mount(
    "/",
    routes![
      index,                      // /
      rooms_handler::list_rooms,  // GET /rooms
      rooms_handler::create_room, // POST /rooms
    ],
  )
}
