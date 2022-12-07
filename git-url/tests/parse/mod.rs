use bstr::ByteSlice;
use git_url::Scheme;

fn assert_url_and(url: &str, expected: git_url::Url) -> Result<git_url::Url, crate::Error> {
    let actual = git_url::parse(url.into())?;
    assert_eq!(actual, expected);
    if actual.scheme.as_str().starts_with("http") {
        assert!(
            actual.path.starts_with_str("/"),
            "paths are never empty and at least '/': {:?}",
            actual.path
        );
        if actual.path.len() < 2 {
            assert!(actual.path_is_root())
        }
    }
    Ok(expected)
}

fn assert_url_roundtrip(url: &str, expected: git_url::Url) -> crate::Result {
    assert_eq!(assert_url_and(url, expected)?.to_bstring(), url);
    Ok(())
}

fn assert_failure(url: &str, expected_err: &str) {
    assert_eq!(git_url::parse(url.into()).unwrap_err().to_string(), expected_err);
}

fn url(
    protocol: Scheme,
    user: impl Into<Option<&'static str>>,
    host: impl Into<Option<&'static str>>,
    port: impl Into<Option<u16>>,
    path: &'static [u8],
) -> git_url::Url {
    git_url::Url::from_parts(
        protocol,
        user.into().map(Into::into),
        host.into().map(Into::into),
        port.into(),
        path.into(),
    )
    .expect("valid")
}

fn url_alternate(
    protocol: Scheme,
    user: impl Into<Option<&'static str>>,
    host: impl Into<Option<&'static str>>,
    port: impl Into<Option<u16>>,
    path: &'static [u8],
) -> git_url::Url {
    url(protocol, user, host, port, path).serialize_alternate_form(true)
}

mod file;
mod invalid;
mod ssh;

mod radicle {
    use git_url::Scheme;

    use crate::parse::{assert_url_roundtrip, url};

    #[test]
    fn basic() -> crate::Result {
        assert_url_roundtrip(
            "rad://hynkuwzskprmswzeo4qdtku7grdrs4ffj3g9tjdxomgmjzhtzpqf81@hwd1yregyf1dudqwkx85x5ps3qsrqw3ihxpx3ieopq6ukuuq597p6m8161c.git",
            url(
                Scheme::Ext("rad".into()),
                "hynkuwzskprmswzeo4qdtku7grdrs4ffj3g9tjdxomgmjzhtzpqf81",
                "hwd1yregyf1dudqwkx85x5ps3qsrqw3ihxpx3ieopq6ukuuq597p6m8161c.git",
                None,
                b"",
            ),
        )
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

mod unknown {
    use git_url::Scheme;

    use crate::parse::{assert_url_roundtrip, url};

    #[test]
    fn any_protocol_is_supported_via_the_ext_scheme() -> crate::Result {
        assert_url_roundtrip(
            "abc://example.com/~byron/hello",
            url(Scheme::Ext("abc".into()), None, "example.com", None, b"/~byron/hello"),
        )
    }
}
