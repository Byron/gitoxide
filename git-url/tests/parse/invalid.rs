use crate::parse::assert_failure;

macro_rules! assertf {
    ($name:ident, $url:literal, $err:literal) => {
        #[test]
        fn $name() {
            assert_failure($url, $err)
        }
    };
}

assertf!(
    unknown_protocol,
    b"foo://host.xz/path/to/repo.git/",
    "protocol parsing failed: 'foo://host.xz/path/to/repo.git/' could not be parsed"
);

#[test]
fn missing_path() {
    assert_failure(b"ssh://host.xz", "missing path")
}
