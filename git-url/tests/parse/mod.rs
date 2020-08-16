fn assert_url(url: &[u8], expected: git_url::Borrowed) -> crate::Result {
    assert_eq!(git_url::parse(url)?, expected);
    Ok(())
}

fn assert_failure(url: &[u8], expected_err: &str) {
    assert_eq!(git_url::parse(url).unwrap_err().to_string(), expected_err);
}

mod invalid {
    use crate::parse::assert_failure;

    #[test]
    fn unknown_protocol() {
        assert_failure(
            b"foo://host.xz/path/to/repo.git/",
            "protocol parsing failed: 'foo://host.xz/path/to/repo.git/' could not be parsed",
        )
    }
}

mod ssh {
    use crate::parse::assert_url;
    use bstr::ByteSlice;
    use git_url::Protocol;

    #[test]
    fn without_user_and_port() -> crate::Result {
        assert_url(
            b"ssh://host.xz/path/to/repo.git/",
            git_url::Borrowed {
                protocol: Protocol::Ssh,
                user: None,
                host: Some(b"host.xz".as_bstr()),
                port: None,
                path: b"/path/to/repo.git/".as_bstr(),
                expand_user: None,
            },
        )
    }
}
