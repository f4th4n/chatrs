#[derive(PartialEq, Debug)]
pub struct User {
  pub username: String,
  pub name: String,
}

impl User {
  pub fn new(username: String, name: String) -> Self {
    User { username, name }
  }
}
