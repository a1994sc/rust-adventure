pub mod linkage {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Encoded {
        pub id: u32,
    }

    impl PartialEq for Encoded {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Decoded {
        pub a: u32,
        pub b: u32,
    }

    impl PartialEq for Decoded {
        fn eq(&self, other: &Self) -> bool {
            self.a == other.a && self.b == other.b
        }
    }

    pub fn separate(enc: Encoded) -> Decoded {
        let pairf: f32 = enc.id as f32;
        let w: u32 = (((8.0 * pairf + 1.0).sqrt() - 1.0) / 2.0).floor() as u32;
        let t: u32 = (num::pow(w, 2) + w) / 2;

        Decoded {
            a: (w - (enc.id - t)),
            b: (enc.id - t),
        }
    }

    pub fn pair(dec: Decoded) -> Encoded {
        Encoded {
            id: (((dec.a + dec.b) * (dec.a + dec.b + 1)) / 2 + dec.b),
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_pair() {
            assert_eq!(pair(Decoded { a: 17, b: 9 }).id, 360);
            assert_eq!(pair(Decoded { a: 9, b: 17 }).id, 368);

            assert_eq!(pair(Decoded { a: 17, b: 9 }), Encoded { id: 360 });
            assert_eq!(pair(Decoded { a: 9, b: 17 }), Encoded { id: 368 });
        }

        #[test]
        fn test_separate() {
            println!("{:?}", Encoded { id: 360 });
            assert_eq!(separate(Encoded { id: 360 }).a, 17);
            assert_eq!(separate(Encoded { id: 360 }).b, 9);

            println!("{:?}", Encoded { id: 368 });
            // assert_eq!(separate(Encoded { id: 368 }).a, 9);
            // assert_eq!(separate(Encoded { id: 368 }).b, 17);

            // assert_eq!(separate(Encoded { id: 360 }), Decoded { a: 17, b: 9 });
            // assert_eq!(separate(Encoded { id: 368 }), Decoded { a: 9, b: 17 });
        }
    }
}
