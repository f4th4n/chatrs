#[macro_use]
extern crate rocket;

use chat_web::rooms_handler;
use dotenvy::dotenv;

#[get("/")]
fn index() -> &'static str {
  "ok!"
}

#[launch]
fn rocket() -> _ {
  dotenv().expect(".env file not found");
  rocket::build().mount(
    "/",
    routes![
      index,                       // /
      rooms_handler::list_rooms,   // GET /rooms
      rooms_handler::create_rooms, // POST /rooms
    ],
  )
}
