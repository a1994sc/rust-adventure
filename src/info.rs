pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub mod info {
    use rocket::serde::{json::Json, Serialize};
    use std::str;

    #[derive(Serialize, Debug)]
    #[serde(crate = "rocket::serde")]
    pub struct Version {
        hostname: String,
        version: String,
        message: String,
        runtime: String,
    }

    #[derive(Serialize, Debug, Clone)]
    #[serde(crate = "rocket::serde")]
    pub struct Package {
        package: String,
        version: String,
    }

    #[derive(Serialize, Debug)]
    #[serde(crate = "rocket::serde")]
    pub struct Packages {
        direct: Vec<Package>,
        indirect: Vec<Package>,
    }

    impl PartialEq for Version {
        fn eq(&self, other: &Self) -> bool {
            self.hostname == other.hostname
                && self.runtime == other.runtime
                && self.version == other.version
                && self.message == other.message
        }
    }

    #[derive(Serialize, Debug)]
    #[serde(crate = "rocket::serde")]
    pub struct Message {
        message: String,
    }

    impl PartialEq for Message {
        fn eq(&self, other: &Self) -> bool {
            self.message == other.message
        }
    }

    #[derive(Serialize, Debug)]
    #[serde(crate = "rocket::serde")]
    pub struct User {
        name: String,
        age: u8,
        alive: bool,
    }

    impl PartialEq for User {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name && self.age == other.age && self.alive == other.alive
        }
    }

    #[get("/version", format = "json")]
    pub fn version() -> Json<Version> {
        let buf: std::ffi::OsString = hostname::get().unwrap();

        let host: &str = match str::from_utf8(buf.as_encoded_bytes()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        let version: Version = Version {
            runtime: super::built_info::RUSTC_VERSION.to_string(),
            message: "This is a cool REST API".to_string(),
            version: super::built_info::PKG_VERSION.to_string(),
            hostname: host.to_string(),
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

    #[get("/package", format = "json")]
    pub fn packages() -> Json<Packages> {
        let mut direct: Vec<Package> = Vec::new();
        let mut indirect: Vec<Package> = Vec::new();

        for (_i, el) in super::built_info::DIRECT_DEPENDENCIES.iter().enumerate() {
            direct.push(Package {package: String::from(el.0), version: String::from(el.1)})
        }

        for (_i, el) in super::built_info::INDIRECT_DEPENDENCIES.iter().enumerate() {
            indirect.push(Package {package: String::from(el.0), version: String::from(el.1)})
        }

        Json(Packages { direct, indirect })
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

    #[cfg(test)]
    mod test {
        #[test]
        fn method_index() {
            let msg0: super::Message = super::Message {
                message: "Hello World!".to_string(),
            };
            let msg1: super::Message = super::Message {
                message: "Hi World!".to_string(),
            };

            assert_eq!(super::index().into_inner(), msg0);
            assert_ne!(super::index().into_inner(), msg1);
        }

        #[test]
        fn method_healthz() {
            assert_eq!(super::healthz(), "OK");
            assert_ne!(super::healthz(), "BAD");
        }

        #[test]
        fn method_todo() {
            let user0: super::User = super::User {
                name: "Jon Snow".to_string(),
                age: 21,
                alive: true,
            };
            let user1: super::User = super::User {
                name: "Jon Snow".to_string(),
                age: 21,
                alive: false,
            };
            assert_eq!(super::todo().into_inner(), user0);
            assert_ne!(super::todo().into_inner(), user1);
        }

        #[test]
        fn struct_message() {
            let same0: super::Message = super::Message {
                message: "This is a test".to_string(),
            };
            let same1: super::Message = super::Message {
                message: "This is a test".to_string(),
            };
            let diff0: super::Message = super::Message {
                message: "This is different".to_string(),
            };
            assert_eq!(same0, same1);
            assert_ne!(same0, diff0);
        }

        #[test]
        fn struct_user() {
            let user0: super::User = super::User {
                name: "John Smith".to_string(),
                age: 18,
                alive: true,
            };
            let user1: super::User = super::User {
                name: "John Smith".to_string(),
                age: 18,
                alive: true,
            };
            let user2: super::User = super::User {
                name: "John Smith".to_string(),
                age: 99,
                alive: false,
            };
            let user3: super::User = super::User {
                name: "John Smith".to_string(),
                age: 99,
                alive: true,
            };

            assert_eq!(user0, user1);
            assert_ne!(user0, user2);
            assert_ne!(user2, user3);
        }
    }
}
