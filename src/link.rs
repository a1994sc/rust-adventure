pub mod linkage {
    use rocket::serde::{json::Json, Serialize};

    #[derive(Serialize, Debug)]
    #[serde(crate = "rocket::serde")]
    pub struct Encoded {
        id: u32,
    }

    impl PartialEq for Encoded {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    #[derive(Serialize, Debug)]
    #[serde(crate = "rocket::serde")]
    pub struct Decoded {
        a: u32,
        b: u32,
    }

    impl PartialEq for Decoded {
        fn eq(&self, other: &Self) -> bool {
            self.a == other.a && self.b == other.b
        }
    }

    #[rocket::get("/separate/<pair>")]
    pub fn separate(pair: u32) -> Json<Decoded> {
        let pairf: f32 = pair as f32;
        let w: u32 = (((8.0 * pairf + 1.0).sqrt() - 1.0) / 2.0).floor() as u32;
        let t: u32 = (num::pow(w, 2) + w) / 2;

        Json(Decoded {
            a: (w - (pair - t)),
            b: (pair - t),
        })
    }

    #[rocket::get("/pair/<x>/<y>")]
    pub fn pair(x: u32, y: u32) -> Json<Encoded> {
        let sum: u32 = (x + y) * (x + y + 1);
        Json(Encoded { id: (sum / 2 + y) })
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn pair() {
            assert_eq!(super::pair(17, 9).into_inner().id, 360);
            assert_eq!(super::pair(9, 17).into_inner().id, 368);

            assert_eq!(super::pair(17, 9).into_inner(), super::Encoded { id: 360 });
            assert_eq!(super::pair(9, 17).into_inner(), super::Encoded { id: 368 });
        }

        #[test]
        fn separate() {
            assert_eq!(super::separate(360).into_inner().a, 17);
            assert_eq!(super::separate(360).into_inner().b, 9);

            assert_eq!(super::separate(368).into_inner().a, 9);
            assert_eq!(super::separate(368).into_inner().b, 17);

            assert_eq!(
                super::separate(360).into_inner(),
                super::Decoded { a: 17, b: 9 }
            );
            assert_eq!(
                super::separate(368).into_inner(),
                super::Decoded { a: 9, b: 17 }
            );
        }
    }
}
