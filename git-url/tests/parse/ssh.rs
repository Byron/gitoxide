use crate::parse::{assert_url, assert_url_and, url};
use git_url::{Protocol, UserExpansion};
use std::path::Path;

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
fn username_expansion_with_username() -> crate::Result {
    let expanded_path = assert_url_and(
        "ssh://example.com/~byron/hello/git",
        url(
            Protocol::Ssh,
            None,
            "example.com",
            None,
            b"/hello/git",
            UserExpansion::Name("byron".into()),
        ),
    )?
    .expand_path_with(|user: &UserExpansion| match user {
        UserExpansion::Current => unreachable!("we have a name"),
        UserExpansion::Name(name) => Some(user_home(name)),
    })?;
    assert_eq!(expanded_path, expected_path());
    Ok(())
}

#[cfg(windows)]
fn expected_path() -> std::path::PathBuf {
    Path::new("C:\\UserProfiles\\byron\\hello\\git").into()
}

#[cfg(not(windows))]
fn expected_path() -> std::path::PathBuf {
    Path::new("/home/byron/hello/git").into()
}

#[cfg(windows)]
fn user_home(name: &str) -> std::path::PathBuf {
    Path::new("C:\\").join("UserProfiles").join(name)
}

#[cfg(not(windows))]
fn user_home(name: &str) -> std::path::PathBuf {
    #[cfg(not(windows))]
    format!("/home/{}", name).into()
}

#[test]
fn username_expansion_without_username() -> crate::Result {
    let expanded_path = assert_url_and(
        "ssh://example.com/~/hello/git",
        url(
            Protocol::Ssh,
            None,
            "example.com",
            None,
            b"/hello/git",
            UserExpansion::Current,
        ),
    )?
    .expand_path_with(|user: &UserExpansion| match user {
        UserExpansion::Current => Some(user_home("byron")),
        UserExpansion::Name(name) => Some(format!("/home/{}", name).into()),
    })?;
    assert_eq!(expanded_path, expected_path());
    Ok(())
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
fn scp_like_without_user_and_username_expansion_without_username() -> crate::Result {
    assert_url(
        "host.xz:~/to/git",
        url(Protocol::Ssh, None, "host.xz", None, b"/to/git", UserExpansion::Current),
    )
}

#[test]
fn scp_like_without_user_and_username_expansion_with_username() -> crate::Result {
    assert_url(
        "host.xz:~byron/to/git",
        url(
            Protocol::Ssh,
            None,
            "host.xz",
            None,
            b"/to/git",
            UserExpansion::Name("byron".into()),
        ),
    )
}

#[test]
fn scp_like_with_user_and_relative_path_turns_into_absolute_path() -> crate::Result {
    assert_url(
        "user@host.xz:./relative",
        url(Protocol::Ssh, "user", "host.xz", None, b"/relative", None),
    )
}
