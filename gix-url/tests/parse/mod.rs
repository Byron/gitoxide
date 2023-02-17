use bstr::ByteSlice;
use gix_url::Scheme;

fn assert_url(url: &str, expected: gix_url::Url) -> Result<gix_url::Url, crate::Error> {
    let actual = gix_url::parse(url.into())?;
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

fn assert_url_roundtrip(url: &str, expected: gix_url::Url) -> crate::Result {
    assert_eq!(assert_url(url, expected)?.to_bstring(), url);
    Ok(())
}

fn assert_failure(url: &str, expected_err: impl ToString) {
    assert_eq!(
        gix_url::parse(url.into()).unwrap_err().to_string(),
        expected_err.to_string()
    );
}

fn url<'a, 'b>(
    protocol: Scheme,
    user: impl Into<Option<&'a str>>,
    host: impl Into<Option<&'b str>>,
    port: impl Into<Option<u16>>,
    path: &[u8],
) -> gix_url::Url {
    gix_url::Url::from_parts(
        protocol,
        user.into().map(Into::into),
        host.into().map(Into::into),
        port.into(),
        path.into(),
    )
    .unwrap_or_else(|err| panic!("'{}' failed: {err:?}", path.as_bstr()))
}

fn url_alternate<'a, 'b>(
    protocol: Scheme,
    user: impl Into<Option<&'a str>>,
    host: impl Into<Option<&'b str>>,
    port: impl Into<Option<u16>>,
    path: &[u8],
) -> gix_url::Url {
    let url = gix_url::Url::from_parts_as_alternative_form(
        protocol.clone(),
        user.into().map(Into::into),
        host.into().map(Into::into),
        port.into(),
        path.into(),
    )
    .expect("valid");
    assert_eq!(url.scheme, protocol);
    url
}

mod file;
mod invalid;
mod ssh;

mod radicle {
    use gix_url::Scheme;

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
    use gix_url::Scheme;

    use crate::parse::{assert_url, assert_url_roundtrip, url};

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

    #[test]
    fn http_missing_path() -> crate::Result {
        assert_url_roundtrip("http://host.xz/", url(Scheme::Http, None, "host.xz", None, b"/"))?;
        assert_url("http://host.xz", url(Scheme::Http, None, "host.xz", None, b"/"))?;
        Ok(())
    }
}
mod git {
    use gix_url::Scheme;

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
    use gix_url::Scheme;

    use crate::parse::{assert_url_roundtrip, url};

    #[test]
    fn any_protocol_is_supported_via_the_ext_scheme() -> crate::Result {
        assert_url_roundtrip(
            "abc://example.com/~byron/hello",
            url(Scheme::Ext("abc".into()), None, "example.com", None, b"/~byron/hello"),
        )
    }
}
