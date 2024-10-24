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

// pub fn separate_nth(lst: &[u32], n: u8) -> &[u32] {
//   if lst.len() >= 2 {
//   } else if lst.len() == 2 {
//   } else {
//   }
//   &[32]
// }

pub fn pair(a: u32, b: u32) -> u32 {
  ((a + b) * (a + b + 1)) / 2 + b
}

pub fn pair_dec(dec: Decoded) -> Encoded {
  Encoded {
    id: pair(dec.a, dec.b),
  }
}

pub fn pair_xyz(x: u32, y: u32, z: u32) -> u32 {
  let a: u32 = ((y + z) * (y + z) + 3 * y + z) / 2;
  let b: u32 = (a + x) * (a + x);
  (3 * x + a + b) / 2
}
