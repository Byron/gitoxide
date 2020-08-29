use crate::parse::{assert_url_and, assert_url_roundtrip, url};
use git_url::Scheme;

#[test]
fn file_path_with_protocol() -> crate::Result {
    assert_url_roundtrip(
        "file:///path/to/git",
        url(Scheme::File, None, None, None, b"/path/to/git"),
    )
}

#[test]
fn file_path_without_protocol() -> crate::Result {
    let url = assert_url_and("/path/to/git", url(Scheme::File, None, None, None, b"/path/to/git"))?.to_string();
    assert_eq!(url, "file:///path/to/git");
    Ok(())
}

#[test]
fn no_username_expansion_for_file_paths_without_protocol() -> crate::Result {
    let url = assert_url_and("~/path/to/git", url(Scheme::File, None, None, None, b"~/path/to/git"))?.to_string();
    assert_eq!(url, "file://~/path/to/git");
    Ok(())
}
#[test]
fn no_username_expansion_for_file_paths_with_protocol() -> crate::Result {
    assert_url_roundtrip(
        "file://~username/path/to/git",
        url(Scheme::File, None, None, None, b"~username/path/to/git"),
    )
}

#[test]
fn non_utf8_file_path_without_protocol() -> crate::Result {
    let parsed = git_url::parse(b"/path/to\xff/git")?;
    assert_eq!(parsed, url(Scheme::File, None, None, None, b"/path/to\xff/git",));
    assert_eq!(
        parsed.to_string(),
        "file:///path/to�/git",
        "non-unicode is made unicode safe"
    );
    Ok(())
}

#[test]
fn relative_file_path_without_protocol() -> crate::Result {
    let parsed = assert_url_and(
        "../../path/to/git",
        url(Scheme::File, None, None, None, b"../../path/to/git"),
    )?
    .to_string();
    assert_eq!(parsed, "file://../../path/to/git");
    let url = assert_url_and("path/to/git", url(Scheme::File, None, None, None, b"path/to/git"))?.to_string();
    assert_eq!(url, "file://path/to/git");
    Ok(())
}

#[test]
fn interior_relative_file_path_without_protocol() -> crate::Result {
    let url = assert_url_and(
        "/abs/path/../../path/to/git",
        url(Scheme::File, None, None, None, b"/abs/path/../../path/to/git"),
    )?
    .to_string();
    assert_eq!(url, "file:///abs/path/../../path/to/git");
    Ok(())
}

mod windows {
    use crate::parse::{assert_url_and, assert_url_roundtrip, url};
    use git_url::Scheme;

    #[test]
    fn file_path_without_protocol() -> crate::Result {
        let url = assert_url_and("x:/path/to/git", url(Scheme::File, None, None, None, b"x:/path/to/git"))?.to_string();
        assert_eq!(url, "file://x:/path/to/git");
        Ok(())
    }

    #[test]
    fn file_path_with_backslashes_without_protocol() -> crate::Result {
        let url = assert_url_and(
            "x:\\path\\to\\git",
            url(Scheme::File, None, None, None, b"x:\\path\\to\\git"),
        )?
        .to_string();
        assert_eq!(url, "file://x:\\path\\to\\git");
        Ok(())
    }

    #[test]
    fn file_path_with_protocol() -> crate::Result {
        assert_url_roundtrip(
            "file://x:/path/to/git",
            url(Scheme::File, None, None, None, b"x:/path/to/git"),
        )
    }
}
