use crate::parse::{assert_url, url};
use git_url::Protocol;

#[test]
fn without_user_and_port() -> crate::Result {
    assert_url(
        b"ssh://host.xz/path/to/repo.git/",
        url(Protocol::Ssh, None, "host.xz", None, "/path/to/repo.git/", None),
    )
}
