pub mod linkage;

// Test suite
#[cfg(test)]
mod test_linkage {
    use crate::linkage::*;

    #[test]
    fn test_pair() {
        assert_eq!(pair(Decoded { a: 17, b: 9 }).id, 360);
        assert_eq!(pair(Decoded { a: 9, b: 17 }).id, 368);

        assert_eq!(pair(Decoded { a: 17, b: 9 }), Encoded { id: 360 });
        assert_eq!(pair(Decoded { a: 9, b: 17 }), Encoded { id: 368 });
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
