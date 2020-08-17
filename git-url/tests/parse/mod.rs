use git_url::{owned::UserExpansion, Protocol};

fn assert_url(url: &[u8], expected: git_url::Owned) -> crate::Result {
    assert_eq!(git_url::parse(url)?, expected);
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
    path: &'static str,
    expand_user: impl Into<Option<UserExpansion>>,
) -> git_url::Owned {
    git_url::Owned {
        protocol,
        user: user.into().map(Into::into),
        host: host.into().map(Into::into),
        port: port.into(),
        path: path.into(),
        expand_user: expand_user.into(),
    }
}

mod invalid;
mod ssh;
