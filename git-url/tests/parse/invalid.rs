use crate::parse::assert_failure;

#[test]
fn unknown_protocol() {
    assert_failure("foo://host.xz/path/to/repo.git/", "Protocol 'foo' is not supported")
}

#[test]
fn missing_path() {
    assert_failure("ssh://host.xz", "Paths cannot be empty")
}

#[test]
fn missing_port_despite_indication() {
    assert_failure("ssh://host.xz:", "Paths cannot be empty")
}
