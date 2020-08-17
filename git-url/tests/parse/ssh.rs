use crate::parse::{assert_url, url};
use git_url::Protocol;

#[test]
fn without_user_and_without_port() -> crate::Result {
    assert_url(
        b"ssh://host.xz/path/to/repo.git/",
        url(Protocol::Ssh, None, "host.xz", None, "/path/to/repo.git/", None),
    )
}

#[test]
fn without_user_and_with_port() -> crate::Result {
    assert_url(b"ssh://host.xz:21/", url(Protocol::Ssh, None, "host.xz", 21, "/", None))
}

#[test]
fn host_is_ipv4() -> crate::Result {
    assert_url(
        b"ssh://127.69.0.1/hello",
        url(Protocol::Ssh, None, "127.69.0.1", None, "/hello", None),
    )
}

#[test]
fn with_user_and_without_port() -> crate::Result {
    assert_url(
        b"ssh://user@host.xz/.git",
        url(Protocol::Ssh, "user", "host.xz", None, "/.git", None),
    )
}
