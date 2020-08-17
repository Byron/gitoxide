use bstr::ByteSlice;
use git_url::{borrowed::UserExpansion, Protocol};

fn assert_url(url: &[u8], expected: git_url::Borrowed) -> crate::Result {
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
    port: impl Into<Option<u32>>,
    path: &'static str,
    expand_user: impl Into<Option<UserExpansion<'static>>>,
) -> git_url::Borrowed<'static> {
    git_url::Borrowed {
        protocol,
        user: user.into().map(|s| s.as_bytes().as_bstr()),
        host: host.into().map(|s| s.as_bytes().as_bstr()),
        port: port.into(),
        path: path.as_bytes().as_bstr(),
        expand_user: expand_user.into(),
    }
}

mod invalid;
mod ssh;
