use crate::parse::assert_failure;

#[test]
fn unknown_protocol() {
    assert_failure(
        "foo://host.xz/path/to/repo.git/",
        "protocol parsing failed: 'foo://host.xz/path/to/repo.git/' could not be parsed",
    )
}

#[test]
fn missing_path() {
    assert_failure(
        "ssh://host.xz",
        "paths cannot be empty and start with '/': '' could not be parsed",
    )
}

#[test]
fn missing_port_despite_indication() {
    assert_failure(
        "ssh://host.xz:",
        "paths cannot be empty and start with '/': ':' could not be parsed",
    )
}
