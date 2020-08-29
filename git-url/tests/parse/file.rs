use crate::parse::{assert_url, url};
use git_url::Protocol;

#[test]
fn file_path_with_protocol() -> crate::Result {
    assert_url(
        "file:///path/to/git",
        url(Protocol::File, None, None, None, b"/path/to/git"),
    )
}

#[test]
fn file_path_without_protocol() -> crate::Result {
    assert_url("/path/to/git", url(Protocol::File, None, None, None, b"/path/to/git"))
}

#[test]
fn no_username_expansion_for_file_paths_without_protocol() -> crate::Result {
    assert_url("~/path/to/git", url(Protocol::File, None, None, None, b"~/path/to/git"))
}
#[test]
fn no_username_expansion_for_file_paths_with_protocol() -> crate::Result {
    assert_url(
        "file://~username/path/to/git",
        url(Protocol::File, None, None, None, b"~username/path/to/git"),
    )
}

#[test]
fn non_utf8_file_path_without_protocol() -> crate::Result {
    assert_eq!(
        git_url::parse(b"/path/to\xff/git")?,
        url(Protocol::File, None, None, None, b"/path/to\xff/git",)
    );
    Ok(())
}

#[test]
fn relative_file_path_without_protocol() -> crate::Result {
    assert_url(
        "../../path/to/git",
        url(Protocol::File, None, None, None, b"../../path/to/git"),
    )?;
    assert_url("path/to/git", url(Protocol::File, None, None, None, b"path/to/git"))
}

#[test]
fn interior_relative_file_path_without_protocol() -> crate::Result {
    assert_url(
        "/abs/path/../../path/to/git",
        url(Protocol::File, None, None, None, b"/abs/path/../../path/to/git"),
    )
}

mod windows {
    use crate::parse::{assert_url, url};
    use git_url::Protocol;

    #[test]
    fn file_path_without_protocol() -> crate::Result {
        assert_url(
            "x:/path/to/git",
            url(Protocol::File, None, None, None, b"x:/path/to/git"),
        )
    }

    #[test]
    fn file_path_with_backslashes_without_protocol() -> crate::Result {
        assert_url(
            "x:\\path\\to\\git",
            url(Protocol::File, None, None, None, b"x:\\path\\to\\git"),
        )
    }

    #[test]
    fn file_path_with_protocol() -> crate::Result {
        assert_url(
            "file://x:/path/to/git",
            url(Protocol::File, None, None, None, b"x:/path/to/git"),
        )
    }
}
