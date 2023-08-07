use bstr::ByteSlice;

#[test]
fn we_match_git_for_typical_cases() {
    let mut count = 0;
    let mut errs = Vec::new();
    for (url, expected) in baseline::URLS.iter() {
        count += 1;
        if std::panic::catch_unwind(|| {
            let actual = gix_url::parse(url).expect("valid urls can be parsed");
            assert_urls_equal(expected, &actual);
        })
        .is_err()
        {
            errs.push((url, expected, gix_url::parse(url)));
        }
    }
    if !errs.is_empty() {
        let valid = count - errs.len();
        dbg!(errs);
        assert_eq!(count, valid, "All parsed URLs should match the baseline");
    }
}

fn assert_urls_equal(expected: &baseline::GitDiagUrl<'_>, actual: &gix_url::Url) {
    assert_eq!(
        gix_url::Scheme::from(expected.protocol.to_str().unwrap()),
        actual.scheme
    );

    match expected.host {
        baseline::GitDiagHost::NonSsh { host_and_port } => match host_and_port {
            Some(host_and_port) => {
                assert!(actual.host().is_some());

                let mut gix_host_and_port = String::with_capacity(host_and_port.len());

                if let Some(user) = actual.user() {
                    gix_host_and_port.push_str(user);
                    gix_host_and_port.push('@');
                }

                gix_host_and_port.push_str(actual.host().unwrap());

                if let Some(port) = actual.port {
                    gix_host_and_port.push(':');
                    gix_host_and_port.push_str(&port.to_string());
                }

                assert_eq!(host_and_port, gix_host_and_port);
            }
            None => {
                assert!(actual.host().is_none());
                assert!(actual.port.is_none());
            }
        },
        baseline::GitDiagHost::Ssh { user_and_host, port } => {
            match user_and_host {
                Some(user_and_host) => {
                    assert!(actual.host().is_some());

                    let mut gix_user_and_host = String::with_capacity(user_and_host.len());
                    if let Some(user) = actual.user() {
                        gix_user_and_host.push_str(user);
                        gix_user_and_host.push('@');
                    }
                    gix_user_and_host.push_str(actual.host().unwrap());

                    assert_eq!(user_and_host, gix_user_and_host);
                }
                None => {
                    assert!(actual.host().is_none());
                    assert!(actual.user().is_none());
                }
            }
            match port {
                Some(port) => {
                    assert!(actual.port.is_some());
                    assert_eq!(port, actual.port.unwrap().to_string());
                }
                None => {
                    assert!(actual.port.is_none());
                }
            }
        }
    }

    match expected.path {
        Some(path) => {
            assert_eq!(path, actual.path);
        }
        None => {
            // I guess? This case does not happen a single time in the current fixtures...
            assert!(actual.path.is_empty());
        }
    }
}

mod baseline {
    use bstr::{BStr, BString, ByteSlice};
    use gix_testtools::once_cell::sync::Lazy;

    static BASELINE: Lazy<BString> = Lazy::new(|| {
        let base = gix_testtools::scripted_fixture_read_only("make_baseline.sh").unwrap();
        BString::from(std::fs::read(base.join("git-baseline.generic")).expect("fixture file exists"))
    });

    pub static URLS: Lazy<Vec<(&'static BStr, GitDiagUrl<'static>)>> = Lazy::new(|| {
        let mut out = Vec::new();

        let url_block = BASELINE
            .split(|c| c == &b';')
            .filter(|url| !url.is_empty())
            .map(ByteSlice::trim);

        for block in url_block {
            let (url, diag_url) = GitDiagUrl::parse(block.as_bstr());
            out.push((url, diag_url));
        }
        out
    });

    #[derive(Debug)]
    pub struct GitDiagUrl<'a> {
        pub protocol: &'a BStr,
        pub host: GitDiagHost<'a>,
        pub path: Option<&'a BStr>,
    }

    impl GitDiagUrl<'_> {
        /// Parses the given string into a [GitDiagUrl] according to the format
        /// specified in [Git's `connect.c`][git_src].
        ///
        /// [git_src]: https://github.com/git/git/blob/master/connect.c#L1415
        fn parse(diag_url: &BStr) -> (&'_ BStr, GitDiagUrl<'_>) {
            let mut lines = diag_url.lines().map(ByteSlice::trim);
            let mut next_attr = |name: &str| {
                lines
                    .next()
                    .expect("well-known format")
                    .strip_prefix(format!("Diag: {name}=").as_bytes())
                    .expect("attribute is at the correct location")
                    .as_bstr()
            };

            let url = next_attr("url");
            let protocol = next_attr("protocol");

            let host = if protocol == "ssh" {
                let user_and_host = next_attr("userandhost");
                let port = next_attr("port");
                GitDiagHost::Ssh {
                    user_and_host: if user_and_host == "NULL" {
                        None
                    } else {
                        Some(user_and_host)
                    },
                    port: if port == "NONE" { None } else { Some(port) },
                }
            } else {
                let host_and_port = next_attr("hostandport");
                GitDiagHost::NonSsh {
                    host_and_port: if host_and_port == "NULL" {
                        None
                    } else {
                        Some(host_and_port)
                    },
                }
            };

            let path = next_attr("path");
            assert!(lines.next().is_none(), "we consume everything");
            (
                url,
                GitDiagUrl {
                    protocol,
                    host,
                    path: if path == "NULL" { None } else { Some(path) },
                },
            )
        }
    }

    #[derive(Debug)]
    pub enum GitDiagHost<'a> {
        NonSsh {
            host_and_port: Option<&'a BStr>,
        },
        Ssh {
            user_and_host: Option<&'a BStr>,
            port: Option<&'a BStr>,
        },
    }
}
