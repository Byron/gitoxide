use crate::parse::{assert_url, url};
use git_url::Protocol;

#[test]
fn without_user_and_without_port() -> crate::Result {
    assert_url(
        "ssh://host.xz/path/to/repo.git/",
        url(Protocol::Ssh, None, "host.xz", None, b"/path/to/repo.git/", None),
    )
}

#[test]
fn without_user_and_with_port() -> crate::Result {
    assert_url("ssh://host.xz:21/", url(Protocol::Ssh, None, "host.xz", 21, b"/", None))
}

#[test]
fn host_is_ipv4() -> crate::Result {
    assert_url(
        "ssh://127.69.0.1/hello",
        url(Protocol::Ssh, None, "127.69.0.1", None, b"/hello", None),
    )
}

#[test]
fn with_user_and_without_port() -> crate::Result {
    assert_url(
        "ssh://user@host.xz/.git",
        url(Protocol::Ssh, "user", "host.xz", None, b"/.git", None),
    )
}

#[test]
fn scp_like_without_user() -> crate::Result {
    assert_url(
        "host.xz:path/to/git",
        url(Protocol::Ssh, None, "host.xz", None, b"/path/to/git", None),
    )
}

#[test]
fn scp_like_with_user_and_relative_path_turns_into_absolute_path() -> crate::Result {
    assert_url(
        "user@host.xz:./relative",
        url(Protocol::Ssh, "user", "host.xz", None, b"/relative", None),
    )
}
