use bstr::ByteSlice;
use gix_url::Scheme;

use crate::parse::{assert_url_roundtrip, url, url_alternate};

#[test]
fn file_path_with_protocol() -> crate::Result {
    assert_url_roundtrip(
        "file:///path/to/git",
        url(Scheme::File, None, None, None, b"/path/to/git"),
    )
}

#[test]
fn file_to_root() -> crate::Result {
    assert_url_roundtrip("file:///", url(Scheme::File, None, None, None, b"/"))
}

#[test]
fn file_path_without_protocol() -> crate::Result {
    assert_url_roundtrip(
        "/path/to/git",
        url_alternate(Scheme::File, None, None, None, b"/path/to/git"),
    )
}

#[test]
fn no_username_expansion_for_file_paths_without_protocol() -> crate::Result {
    assert_url_roundtrip(
        "~/path/to/git",
        url_alternate(Scheme::File, None, None, None, b"~/path/to/git"),
    )
}

#[test]
fn no_username_expansion_for_file_paths_with_protocol() -> crate::Result {
    assert_url_roundtrip(
        "file:///~username/path/to/git",
        url(Scheme::File, None, None, None, b"/~username/path/to/git"),
    )?;
    assert_url_roundtrip(
        "file://~username/path/to/git",
        url(Scheme::File, None, "~username", None, b"/path/to/git"),
    )
}

#[test]
fn non_utf8_file_path_without_protocol() -> crate::Result {
    let url = gix_url::parse(b"/path/to\xff/git".as_bstr())?;
    assert_eq!(url, url_alternate(Scheme::File, None, None, None, b"/path/to\xff/git"));
    let url_lossless = url.to_bstring();
    assert_eq!(
        url_lossless.to_string(),
        "/path/to�/git",
        "non-unicode is made unicode safe after conversion"
    );
    assert_eq!(url_lossless, &b"/path/to\xff/git"[..], "otherwise it's lossless");
    Ok(())
}

#[test]
fn relative_file_path_without_protocol() -> crate::Result {
    assert_url_roundtrip(
        "../../path/to/git",
        url_alternate(Scheme::File, None, None, None, b"../../path/to/git"),
    )?;
    assert_url_roundtrip(
        "path/to/git",
        url_alternate(Scheme::File, None, None, None, b"path/to/git"),
    )
}

#[test]
fn shortest_possible_absolute_path() -> crate::Result {
    assert_url_roundtrip("/", url_alternate(Scheme::File, None, None, None, b"/"))?;
    assert_url_roundtrip("file:///", url(Scheme::File, None, None, None, b"/"))
}

#[test]
fn shortest_possible_relative_path() -> crate::Result {
    assert_url_roundtrip("a", url_alternate(Scheme::File, None, None, None, b"a"))?;
    assert_url_roundtrip("../", url_alternate(Scheme::File, None, None, None, b"../"))?;
    assert_url_roundtrip("..\\", url_alternate(Scheme::File, None, None, None, b"..\\"))?;
    assert_url_roundtrip("./", url_alternate(Scheme::File, None, None, None, b"./"))?;
    assert_url_roundtrip(".", url_alternate(Scheme::File, None, None, None, b"."))?;
    assert_url_roundtrip("..", url_alternate(Scheme::File, None, None, None, b".."))?;
    Ok(())
}

#[test]
fn no_relative_paths_if_protocol() -> crate::Result {
    assert_url_roundtrip("file://../", url(Scheme::File, None, "..", None, b"/"))?;
    assert_url_roundtrip("file://./", url(Scheme::File, None, ".", None, b"/"))?;
    assert_url_roundtrip("file://a/", url(Scheme::File, None, "a", None, b"/"))?;
    if cfg!(windows) {
        assert_eq!(
            gix_url::parse("file://.\\".into())?,
            url(Scheme::File, None, ".", None, b"\\"),
            "we are just as none-sensical as git here due to special handling."
        );
    } else {
        assert_matches::assert_matches!(
            gix_url::parse("file://.\\".into()),
            Err(gix_url::parse::Error::MissingRepositoryPath { .. }),
            "DEVIATION: on windows, this parses with git into something nonsensical Diag: url=file://./ Diag: protocol=file Diag: hostandport=./ Diag: path=//./"
        );
    }
    Ok(())
}

#[test]
fn interior_relative_file_path_without_protocol() -> crate::Result {
    assert_url_roundtrip(
        "/abs/path/../../path/to/git",
        url_alternate(Scheme::File, None, None, None, b"/abs/path/../../path/to/git"),
    )
}

#[test]
fn url_from_relative_path_with_colon_in_name() -> crate::Result {
    assert_url_roundtrip(
        "./weird/directory/na:me",
        url_alternate(Scheme::File, None, None, None, b"./weird/directory/na:me"),
    )
}

#[cfg(windows)]
mod windows {
    use gix_url::Scheme;

    use crate::parse::{assert_url, assert_url_roundtrip, url, url_alternate};

    #[test]
    fn reproduce_1063() -> crate::Result {
        let input = "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\tmp.vIa4tyjv17";
        let url_input = "file://C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\tmp.vIa4tyjv17";
        assert_url(url_input, url(Scheme::File, None, None, None, input.as_bytes()))?;
        assert_url(input, url_alternate(Scheme::File, None, None, None, input.as_bytes()))?;
        Ok(())
    }

    #[test]
    fn url_from_absolute_path() -> crate::Result {
        assert_url(
            url::Url::from_directory_path("c:\\users\\1")
                .expect("valid")
                .to_file_path()
                .expect("valid path")
                .to_string_lossy()
                .as_ref(),
            url_alternate(Scheme::File, None, None, None, b"C:\\users\\1\\"),
        )?;
        // A special hack to support URLs on windows that are prefixed with `/` even though absolute.
        let url = assert_url("file:///c:/users/2", url(Scheme::File, None, None, None, b"c:/users/2"))?;
        assert_eq!(url.to_bstring(), "file://c:/users/2");
        Ok(())
    }

    #[test]
    fn file_path_without_protocol() -> crate::Result {
        assert_url_roundtrip(
            "x:/path/to/git",
            url_alternate(Scheme::File, None, None, None, b"x:/path/to/git"),
        )
    }

    #[test]
    fn file_path_with_backslashes_without_protocol() -> crate::Result {
        assert_url_roundtrip(
            "x:\\path\\to\\git",
            url_alternate(Scheme::File, None, None, None, b"x:\\path\\to\\git"),
        )
    }

    #[test]
    fn file_path_with_protocol() -> crate::Result {
        assert_url_roundtrip(
            "file://x:/path/to/git",
            url(Scheme::File, None, None, None, b"x:/path/to/git"),
        )
    }
}

#[cfg(not(windows))]
mod unix {
    use gix_url::Scheme;

    use crate::parse::{assert_url_roundtrip, url, url_alternate};

    #[test]
    fn url_from_absolute_path() -> crate::Result {
        assert_url_roundtrip(
            url::Url::from_directory_path("/users/foo")
                .expect("valid")
                .to_file_path()
                .expect("valid path")
                .to_string_lossy()
                .as_ref(),
            url_alternate(Scheme::File, None, None, None, b"/users/foo/"),
        )
    }

    #[test]
    fn file_path_without_protocol() -> crate::Result {
        assert_url_roundtrip(
            "x:/path/to/git",
            url_alternate(Scheme::Ssh, None, "x", None, b"/path/to/git"),
        )
    }

    #[test]
    fn file_path_with_backslashes_without_protocol() -> crate::Result {
        assert_url_roundtrip(
            "x:\\path\\to\\git",
            url_alternate(Scheme::Ssh, None, "x", None, b"\\path\\to\\git"),
        )
    }

    #[test]
    fn file_path_with_protocol() -> crate::Result {
        assert_url_roundtrip(
            "file://x:/path/to/git",
            url(Scheme::File, None, "x:", None, b"/path/to/git"),
        )
    }
}
