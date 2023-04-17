use gix_attributes::StateRef;

const ILLFORMED_UTF8: &[u8] = b"\xC3\x28\x41";

mod value {
    use gix_attributes::state::ValueRef;

    use crate::state::ILLFORMED_UTF8;

    #[test]
    fn from_bytes() {
        assert_eq!(ValueRef::from_bytes(ILLFORMED_UTF8).as_bstr(), ILLFORMED_UTF8);
        assert_eq!(ValueRef::from_bytes("utf8".as_bytes()).as_bstr(), "utf8");
    }
}

#[test]
fn from_value() {
    assert!(std::str::from_utf8(ILLFORMED_UTF8).is_err());
    assert!(
        matches!(StateRef::from_bytes(ILLFORMED_UTF8), StateRef::Value(v) if v.as_bstr() == ILLFORMED_UTF8),
        "this can round-trip with care"
    );
}
