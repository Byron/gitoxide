mod from_hex_len {
    use gix_hash::Kind;

    #[test]
    fn some_sha1() {
        assert_eq!(Kind::from_hex_len(0), Some(Kind::Sha1));
        assert_eq!(Kind::from_hex_len(10), Some(Kind::Sha1));
        assert_eq!(Kind::from_hex_len(20), Some(Kind::Sha1));
        assert_eq!(Kind::from_hex_len(40), Some(Kind::Sha1));
    }

    #[test]
    fn none_if_there_is_no_fit() {
        assert_eq!(Kind::from_hex_len(65), None);
    }
}
