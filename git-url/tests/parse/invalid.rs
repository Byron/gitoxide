use crate::parse::assert_failure;

#[test]
fn relative_path_due_to_double_colon() {
    assert_failure(
        "invalid:://host.xz/path/to/repo.git/",
        "Relative URLs are not permitted: \"invalid:://host.xz/path/to/repo.git/\"",
    )
}

#[test]
fn missing_path() {
    assert_failure("ssh://host.xz", "Paths cannot be empty")
}

#[test]
fn missing_port_despite_indication() {
    assert_failure("ssh://host.xz:", "Paths cannot be empty")
}
