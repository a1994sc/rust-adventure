pub mod linkage;

// Test suite
#[cfg(test)]
mod test_linkage {
  use crate::linkage::*;

  #[test]
  fn test_pair() {
    assert_eq!(pair_dec(Decoded { a: 17, b: 9 }).id, 360);
    assert_eq!(pair_dec(Decoded { a: 9, b: 17 }).id, 368);

    assert_eq!(pair_dec(Decoded { a: 17, b: 9 }), Encoded { id: 360 });
    assert_eq!(pair_dec(Decoded { a: 9, b: 17 }), Encoded { id: 368 });

    assert_eq!(pair_xyz(17, 9, 30), 325238);
    assert_eq!(pair_xyz(17, 30, 9), 342395);
    assert_eq!(pair_xyz(9, 17, 30), 666444);
    assert_eq!(pair_xyz(9, 30, 17), 681537);
    assert_eq!(pair_xyz(30, 17, 9), 79431);
    assert_eq!(pair_xyz(30, 9, 17), 76275);
  }

  #[test]
  fn test_separate() {
    assert_eq!(separate(Encoded { id: 360 }).a, 17);
    assert_eq!(separate(Encoded { id: 360 }).b, 9);

    assert_eq!(separate(Encoded { id: 368 }).a, 9);
    assert_eq!(separate(Encoded { id: 368 }).b, 17);

    assert_eq!(separate(Encoded { id: 360 }), Decoded { a: 17, b: 9 });
    assert_eq!(separate(Encoded { id: 368 }), Decoded { a: 9, b: 17 });
  }
}
