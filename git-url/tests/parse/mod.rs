use git_url::Scheme;

fn assert_url_and(url: &str, expected: git_url::Url) -> Result<git_url::Url, crate::Error> {
    assert_eq!(git_url::parse(url.as_bytes())?, expected);
    Ok(expected)
}

fn assert_url_roundtrip(url: &str, expected: git_url::Url) -> crate::Result {
    assert_eq!(assert_url_and(url, expected)?.to_string(), url);
    Ok(())
}

fn assert_failure(url: &str, expected_err: &str) {
    assert_eq!(git_url::parse(url.as_bytes()).unwrap_err().to_string(), expected_err);
}

fn url(
    protocol: Scheme,
    user: impl Into<Option<&'static str>>,
    host: impl Into<Option<&'static str>>,
    port: impl Into<Option<u16>>,
    path: &'static [u8],
) -> git_url::Url {
    git_url::Url {
        scheme: protocol,
        user: user.into().map(Into::into),
        host: host.into().map(Into::into),
        port: port.into(),
        path: path.into(),
    }
}

mod file;
mod invalid;
mod ssh;

mod radicle {
    use git_url::Scheme;

    use crate::parse::{assert_url_roundtrip, url};

    #[test]
    fn basic() -> crate::Result {
        assert_url_roundtrip("rad://hynkuwzskprmswzeo4qdtku7grdrs4ffj3g9tjdxomgmjzhtzpqf81@hwd1yregyf1dudqwkx85x5ps3qsrqw3ihxpx3ieopq6ukuuq597p6m8161c.git", url(Scheme::Radicle, "hynkuwzskprmswzeo4qdtku7grdrs4ffj3g9tjdxomgmjzhtzpqf81", "hwd1yregyf1dudqwkx85x5ps3qsrqw3ihxpx3ieopq6ukuuq597p6m8161c.git", None, b""))
    }
}

mod http {
    use git_url::Scheme;

    use crate::parse::{assert_url_roundtrip, url};

    #[test]
    fn username_expansion_is_unsupported() -> crate::Result {
        assert_url_roundtrip(
            "http://example.com/~byron/hello",
            url(Scheme::Http, None, "example.com", None, b"/~byron/hello"),
        )
    }
    #[test]
    fn secure() -> crate::Result {
        assert_url_roundtrip(
            "https://github.com/byron/gitoxide",
            url(Scheme::Https, None, "github.com", None, b"/byron/gitoxide"),
        )
    }
}
mod git {
    use git_url::Scheme;

    use crate::parse::{assert_url_roundtrip, url};

    #[test]
    fn username_expansion_with_username() -> crate::Result {
        assert_url_roundtrip(
            "git://example.com/~byron/hello",
            url(Scheme::Git, None, "example.com", None, b"/~byron/hello"),
        )
    }
}
