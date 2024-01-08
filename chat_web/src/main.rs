#[macro_use]
extern crate rocket;

mod rooms_handler;
mod user_repo;

use dotenvy::dotenv;

#[get("/")]
fn index() -> &'static str {
  "ok!"
}

#[launch]
fn rocket() -> _ {
  dotenv().expect(".env file not found");
  rocket::build().mount("/", routes![index, rooms_handler::rooms_index])
}
