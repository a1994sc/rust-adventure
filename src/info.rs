pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub mod info {
    use rocket::serde::{json::Json, Serialize};
    use std::str;

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Git {
        version: String,
        dirty: bool,
        commit: String,
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Version {
        hostname: String,
        version: String,
        message: String,
        runtime: String,
        git: Git,
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Message {
        message: String,
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct User {
        name: String,
        age: u8,
        alive: bool,
    }

    #[get("/version", format = "json")]
    pub fn version() -> Json<Version> {
        let buf: std::ffi::OsString = hostname::get().unwrap();

        let host: &str = match str::from_utf8(buf.as_encoded_bytes()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        let git: Git;

        if let (Some(v), Some(dirty), Some(hash)) = (
            super::built_info::GIT_VERSION,
            super::built_info::GIT_DIRTY,
            super::built_info::GIT_COMMIT_HASH,
        ) {
            git = Git {
                commit: hash.to_string(),
                version: v.to_string(),
                dirty,
            };
        } else {
            git = Git {
                commit: "fake-hash".to_string(),
                version: "0.0.0".to_string(),
                dirty: true,
            };
        }

        let version: Version = Version {
            runtime: super::built_info::RUSTC_VERSION.to_string(),
            message: super::built_info::RUSTC_VERSION.to_string(),
            version: super::built_info::PKG_VERSION.to_string(),
            hostname: host.to_string(),
            git: git,
        };

        Json(version)
    }

    #[get("/", format = "json")]
    pub fn index() -> Json<Message> {
        let msg: Message = Message {
            message: "Hello World!".to_string(),
        };
        Json(msg)
    }

    #[get("/healthz")]
    pub fn healthz() -> &'static str {
        "OK"
    }

    #[get("/todo", format = "json")]
    pub fn todo() -> Json<User> {
        let user: User = User {
            name: "Jon Snow".to_string(),
            age: 21,
            alive: true,
        };
        Json(user)
    }
}
