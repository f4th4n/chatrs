#[macro_use]
extern crate rocket;

mod rooms_handler;
mod user_repo;

#[get("/")]
fn index() -> &'static str {
  "ok!"
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![index, rooms_handler::rooms_index])
}
