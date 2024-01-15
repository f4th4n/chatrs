#[derive(PartialEq, Debug)]
pub struct User {
  pub username: String,
}

impl User {
  pub fn new(username: String) -> Self {
    User { username }
  }
}
