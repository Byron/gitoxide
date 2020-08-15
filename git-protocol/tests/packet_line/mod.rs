fn assert_err_display<T: std::fmt::Debug, E: std::error::Error>(res: Result<T, E>, expected: impl AsRef<str>) {
    match res {
        Ok(v) => assert!(false, "Expected error '{}', got value {:?}", expected.as_ref(), v),
        Err(err) => assert_eq!(err.to_string(), expected.as_ref()),
    }
}

mod read {
    use git_protocol::packet_line;
    use std::path::PathBuf;

    fn fixture_path(path: &str) -> PathBuf {
        PathBuf::from("tests/fixtures").join(path)
    }
    fn fixture_bytes(path: &str) -> Vec<u8> {
        std::fs::read(fixture_path(path)).expect("readable fixture")
    }

    #[test]
    fn read_from_file() {
        let bytes = fixture_bytes("v1/fetch/01-many-refs.response");
        let _rd = packet_line::Reader::new(&bytes[..]);
    }
}
mod decode;
mod encode;
