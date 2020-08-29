use git_url::Protocol;

fn assert_url_and(url: &str, expected: git_url::Url) -> Result<git_url::Url, crate::Error> {
    assert_eq!(git_url::parse(url.as_bytes())?, expected);
    Ok(expected)
}

fn assert_url(url: &str, expected: git_url::Url) -> crate::Result {
    assert_url_and(url, expected).map(|_| ())
}

fn assert_url_roundtrip(url: &str, expected: git_url::Url) -> crate::Result {
    assert_eq!(assert_url_and(url, expected).map(|s| s.to_string())?, url);
    Ok(())
}

fn assert_failure(url: &str, expected_err: &str) {
    assert_eq!(git_url::parse(url.as_bytes()).unwrap_err().to_string(), expected_err);
}

fn url(
    protocol: Protocol,
    user: impl Into<Option<&'static str>>,
    host: impl Into<Option<&'static str>>,
    port: impl Into<Option<u16>>,
    path: &'static [u8],
) -> git_url::Url {
    git_url::Url {
        protocol,
        user: user.into().map(Into::into),
        host: host.into().map(Into::into),
        port: port.into(),
        path: path.into(),
    }
}

mod file;
mod invalid;
mod ssh;
mod http {
    use crate::parse::{assert_url, assert_url_roundtrip, url};
    use git_url::Protocol;

    #[test]
    fn username_expansion_is_unsupported() -> crate::Result {
        assert_url_roundtrip(
            "http://example.com/~byron/hello",
            url(Protocol::Http, None, "example.com", None, b"/~byron/hello"),
        )
    }
    #[test]
    fn secure() -> crate::Result {
        assert_url(
            "https://github.com/byron/gitoxide",
            url(Protocol::Https, None, "github.com", None, b"/byron/gitoxide"),
        )
    }
}
mod git {
    use crate::parse::{assert_url, url};
    use git_url::Protocol;

    #[test]
    fn username_expansion_with_username() -> crate::Result {
        assert_url(
            "git://example.com/~byron/hello",
            url(Protocol::Git, None, "example.com", None, b"/~byron/hello"),
        )
    }
}
