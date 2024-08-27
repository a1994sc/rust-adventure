use std::fmt;
use std::clone;
// use gethostname::gethostname;


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl fmt::Display for User {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}\t{}\t{}\t{}", self.active, self.sign_in_count, self.email, self.username)
  }
}

impl clone::Clone for User {
    fn clone(&self) -> Self {
      User { active: self.active, username: self.username.clone(), email: self.email.clone(), sign_in_count: self.sign_in_count }
    }

    fn clone_from(&mut self, _source: &Self) {}
}

fn main() {
  // let name = hostname::get();

  // println!("{}", name.to_string_lossy());

  let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  let user3 = user1.clone();

  let user2 = User {
    email: String::from("another@example.com"),
    ..user1
  };

  println!("{user2}");
  println!("{user3}");
}
