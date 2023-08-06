use std::collections::HashMap;

use bstr::{BStr, BString, ByteSlice};
use gix_testtools::once_cell::sync::Lazy;

struct GitDiagUrl<'a> {
    protocol: &'a BStr,
    host: GitDiagHost<'a>,
    path: Option<&'a BStr>,
}

impl GitDiagUrl<'_> {
    /// Parses the given string into a [GitDiagUrl] according to the format
    /// specified in [Git's `connect.c`][git_src].
    ///
    /// [git_src]: https://github.com/git/git/blob/master/connect.c#L1415
    fn parse(diag_url: &BStr) -> (&'_ BStr, GitDiagUrl<'_>) {
        let mut lines = diag_url.lines().map(ByteSlice::trim);

        let url = lines
            .next()
            .expect("diag_url has at least four lines")
            .strip_prefix("Diag: url=".as_bytes())
            .expect("diag_url's first line contains the url")
            .as_bstr();

        let protocol = lines
            .next()
            .expect("diag_url has at least 4 lines")
            .strip_prefix("Diag: protocol=".as_bytes())
            .expect("diag_url's second line contains the protocol")
            .as_bstr();

        let host = if protocol == "ssh" {
            let user_and_host = lines
                .next()
                .expect("diag_url for ssh protocol has 5 lines")
                .strip_prefix("Diag: userandhost=".as_bytes())
                .expect("diag_url's third line contains the userandhost because protocl is ssh")
                .as_bstr();

            let port = lines
                .next()
                .expect("diag_url for ssh protocol has 5 lines")
                .strip_prefix("Diag: port=".as_bytes())
                .expect("diag_url's fourth line contains the port because protocol is ssh")
                .as_bstr();

            GitDiagHost::Ssh {
                user_and_host: if user_and_host == "NULL" {
                    None
                } else {
                    Some(user_and_host)
                },
                port: if port == "NONE" { None } else { Some(port) },
            }
        } else {
            let host_and_port = lines
                .next()
                .expect("diag_url for non ssh protocol has 4 lines")
                .strip_prefix("Diag: hostandport=".as_bytes())
                .expect("diag_url's third line contains the hostandport because protocol is not ssh")
                .as_bstr();

            GitDiagHost::NonSsh {
                host_and_port: if host_and_port == "NULL" {
                    None
                } else {
                    Some(host_and_port)
                },
            }
        };

        let path = lines
            .next()
            .expect("diag_url has enough lines")
            .strip_prefix("Diag: path=".as_bytes())
            .expect("diag_url's last line contains the path")
            .as_bstr();

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

enum GitDiagHost<'a> {
    NonSsh {
        host_and_port: Option<&'a BStr>,
    },
    Ssh {
        user_and_host: Option<&'a BStr>,
        port: Option<&'a BStr>,
    },
}

static BASELINE: Lazy<BString> = Lazy::new(|| {
    let base = gix_testtools::scripted_fixture_read_only("make_baseline.sh").unwrap();
    BString::from(std::fs::read(base.join("git-baseline.generic")).expect("fixture file exists"))
});

static URLS: Lazy<HashMap<&'static BStr, GitDiagUrl<'static>>> = Lazy::new(|| {
    let mut map = HashMap::<&'static BStr, GitDiagUrl<'static>>::new();

    let diag_urls = BASELINE
        .split(|c| c == &b';')
        .filter(|url| !url.is_empty())
        .map(ByteSlice::trim);

    for diag_url in diag_urls {
        let (url, diag_url) = GitDiagUrl::parse(diag_url.as_bstr());
        map.insert(url, diag_url);
    }

    map
});

#[test]
fn baseline() {
    for (url, diag_url) in URLS.iter() {
        let gix_url = gix_url::parse(url);

        assert!(gix_url.is_ok(), "git did not fail to parse this url either");
        let gix_url = gix_url.unwrap();

        assert_urls_equal(diag_url, &gix_url);
    }
}

fn assert_urls_equal(diag_url: &GitDiagUrl<'_>, gix_url: &gix_url::Url) {
    assert_eq!(diag_url.protocol, gix_url.scheme.as_str());

    match diag_url.host {
        GitDiagHost::NonSsh { host_and_port } => match host_and_port {
            Some(host_and_port) => {
                assert!(gix_url.host().is_some());

                let mut gix_host_and_port = String::with_capacity(host_and_port.len());

                if let Some(user) = gix_url.user() {
                    gix_host_and_port.push_str(user);
                    gix_host_and_port.push('@');
                }

                gix_host_and_port.push_str(gix_url.host().unwrap());

                if let Some(port) = gix_url.port {
                    gix_host_and_port.push(':');
                    gix_host_and_port.push_str(&port.to_string());
                }

                assert_eq!(host_and_port, gix_host_and_port);
            }
            None => {
                assert!(gix_url.host().is_none());
                assert!(gix_url.port.is_none());
            }
        },
        GitDiagHost::Ssh { user_and_host, port } => {
            match user_and_host {
                Some(user_and_host) => {
                    assert!(gix_url.host().is_some());

                    let mut gix_user_and_host = String::with_capacity(user_and_host.len());
                    if let Some(user) = gix_url.user() {
                        gix_user_and_host.push_str(user);
                        gix_user_and_host.push('@');
                    }
                    gix_user_and_host.push_str(gix_url.host().unwrap());

                    assert_eq!(user_and_host, gix_user_and_host);
                }
                None => {
                    assert!(gix_url.host().is_none());
                    assert!(gix_url.user().is_none());
                }
            }
            match port {
                Some(port) => {
                    assert!(gix_url.port.is_some());
                    assert_eq!(port, gix_url.port.unwrap().to_string());
                }
                None => {
                    assert!(gix_url.port.is_none());
                }
            }
        }
    }

    match diag_url.path {
        Some(path) => {
            assert_eq!(path, gix_url.path);
        }
        None => {
            // I guess? This case does not happen a single time in the current fixtures...
            assert!(gix_url.path.is_empty());
        }
    }
}
