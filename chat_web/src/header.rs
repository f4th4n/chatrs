use rocket::{
  http::Status,
  outcome::Outcome,
  request::{self, FromRequest, Request as OtherRequest},
};

#[derive(Debug)]
pub struct Headers {
  pub token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Headers {
  type Error = ();

  async fn from_request(request: &'r OtherRequest<'_>) -> request::Outcome<Self, Self::Error> {
    let headers = request.headers();
    let authorization = headers.get_one("Authorization").map(|h| h.to_string());
    match authorization {
      Some(token) => Outcome::Success(Headers { token }),
      None => Outcome::Error((Status { code: 401 }, ())),
    }
  }
}
