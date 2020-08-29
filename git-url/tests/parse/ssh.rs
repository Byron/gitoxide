use crate::parse::{assert_url_and, assert_url_roundtrip, url};
use git_url::Scheme;

#[test]
fn without_user_and_without_port() -> crate::Result {
    assert_url_roundtrip(
        "ssh://host.xz/path/to/repo.git/",
        url(Scheme::Ssh, None, "host.xz", None, b"/path/to/repo.git/"),
    )
}

#[test]
fn without_user_and_with_port() -> crate::Result {
    assert_url_roundtrip("ssh://host.xz:21/", url(Scheme::Ssh, None, "host.xz", 21, b"/"))
}

#[test]
fn host_is_ipv4() -> crate::Result {
    assert_url_roundtrip(
        "ssh://127.69.0.1/hello",
        url(Scheme::Ssh, None, "127.69.0.1", None, b"/hello"),
    )
}

#[test]
fn username_expansion_with_username() -> crate::Result {
    assert_url_roundtrip(
        "ssh://example.com/~byron/hello/git",
        url(Scheme::Ssh, None, "example.com", None, b"/~byron/hello/git"),
    )
}

#[test]
fn username_expansion_without_username() -> crate::Result {
    assert_url_roundtrip(
        "ssh://example.com/~/hello/git",
        url(Scheme::Ssh, None, "example.com", None, b"/~/hello/git"),
    )
}

#[test]
fn with_user_and_without_port() -> crate::Result {
    assert_url_roundtrip(
        "ssh://user@host.xz/.git",
        url(Scheme::Ssh, "user", "host.xz", None, b"/.git"),
    )
}

#[test]
fn scp_like_without_user() -> crate::Result {
    let url = assert_url_and(
        "host.xz:path/to/git",
        url(Scheme::Ssh, None, "host.xz", None, b"/path/to/git"),
    )?
    .to_string();
    assert_eq!(url, "ssh://host.xz/path/to/git");
    Ok(())
}

#[test]
fn scp_like_without_user_and_username_expansion_without_username() -> crate::Result {
    let url = assert_url_and(
        "host.xz:~/to/git",
        url(Scheme::Ssh, None, "host.xz", None, b"/~/to/git"),
    )?
    .to_string();
    assert_eq!(url, "ssh://host.xz/~/to/git");
    Ok(())
}

#[test]
fn scp_like_without_user_and_username_expansion_with_username() -> crate::Result {
    let url = assert_url_and(
        "host.xz:~byron/to/git",
        url(Scheme::Ssh, None, "host.xz", None, b"/~byron/to/git"),
    )?
    .to_string();
    assert_eq!(url, "ssh://host.xz/~byron/to/git");
    Ok(())
}

#[test]
fn scp_like_with_user_and_relative_path_turns_into_absolute_path() -> crate::Result {
    let url = assert_url_and(
        "user@host.xz:./relative",
        url(Scheme::Ssh, "user", "host.xz", None, b"/relative"),
    )?
    .to_string();
    assert_eq!(url, "ssh://user@host.xz/relative");
    Ok(())
}
