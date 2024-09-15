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
