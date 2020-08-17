use crate::parse::{assert_url, url};
use git_url::Protocol;

#[test]
fn file_path_with_protocol() -> crate::Result {
    assert_url(
        "file:///path/to/git",
        url(Protocol::File, None, None, None, "/path/to/git", None),
    )
}

#[test]
fn windows_file_path_with_protocol() -> crate::Result {
    assert_url(
        "file://x:/path/to/git",
        url(Protocol::File, None, None, None, "x:/path/to/git", None),
    )
}

#[test]
fn file_path_without_protocol() -> crate::Result {
    assert_url(
        "/path/to/git",
        url(Protocol::File, None, None, None, "/path/to/git", None),
    )
}

#[test]
fn relative_file_path_without_protocol() -> crate::Result {
    assert_url(
        "../../path/to/git",
        url(Protocol::File, None, None, None, "../../path/to/git", None),
    )
}

#[test]
fn interior_relative_file_path_without_protocol() -> crate::Result {
    assert_url(
        "/abs/path/../../path/to/git",
        url(Protocol::File, None, None, None, "/abs/path/../../path/to/git", None),
    )
}

#[test]
fn file_path_on_windows_without_protocol() -> crate::Result {
    assert_url(
        "x:/path/to/git",
        url(Protocol::File, None, None, None, "x:/path/to/git", None),
    )
}
