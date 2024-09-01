#[macro_use]
extern crate rocket;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

use rocket::serde::{json::Json, Serialize};
use std::str;
use num::pow;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct User {
    name: String,
    age: u8,
    alive: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    message: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Decode {
    id: u32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Encode {
    a: u32,
    b: u32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Git {
    version: String,
    dirty: bool,
    commit: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Version {
    hostname: String,
    version: String,
    message: String,
    runtime: String,
    git: Git,
}

#[get("/todo", format = "json")]
fn todo() -> Json<User> {
    let user: User = User {
        name: "Jon Snow".to_string(),
        age: 21,
        alive: true,
    };
    Json(user)
}

#[get("/", format = "json")]
fn index() -> Json<Message> {
    let msg: Message = Message {
        message: "Hello World!".to_string(),
    };
    Json(msg)
}

#[get("/version", format = "json")]
fn version() -> Json<Version> {
    let buf: std::ffi::OsString = hostname::get().unwrap();

    let host: &str = match str::from_utf8(buf.as_encoded_bytes()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let git: Git;

    if let (Some(v), Some(dirty), Some(hash)) = (
      built_info::GIT_VERSION,
      built_info::GIT_DIRTY,
      built_info::GIT_COMMIT_HASH,
    ) {
      git = Git {
        commit: hash.to_string(),
        version: v.to_string(),
        dirty: dirty,
      };
    } else {
      git = Git{
        commit: "fake-hash".to_string(),
        version: "0.0.0".to_string(),
        dirty: true,
      };
    }

    let version: Version = Version {
        runtime: built_info::RUSTC_VERSION.to_string(),
        message: built_info::RUSTC_VERSION.to_string(),
        version: built_info::PKG_VERSION.to_string(),
        hostname: host.to_string(),
        git: git,
    };

    Json(version)
}

#[get("/pair/<num_a>/<num_b>")]
fn pair(num_a: u32, num_b: u32) -> Json<Decode> {
  let sum: u32 = (num_a + num_b) * (num_a + num_b + 1);
  Json(Decode{
    id: (sum / 2 + num_b),
  })
}

#[get("/unpair/<pair>")]
fn unpair(pair: u32) -> Json<Encode> {
  let pairf: f32 = pair as f32;
  let w: u32 = (((8.0 * pairf + 1.0).sqrt()-1.0)/2.0).floor() as u32;
  let t: u32 = (pow(w, 2)+w)/2;

  Json(Encode { a: (w-(pair-t)), b: (pair-t) })
}

#[get("/healthz")]
fn healthz() -> &'static str {
    "OK"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, todo, version, unpair, pair, healthz])
}
