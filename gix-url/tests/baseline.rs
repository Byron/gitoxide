use std::any::Any;

use bstr::ByteSlice;

#[test]
fn run() {
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let mut count = 0;
    let mut failures = Vec::new();
    let (mut failed_roundtrips, mut serialized_url_does_not_match_input) = (0, 0);
    for (url, expected) in baseline::URLS.iter() {
        count += 1;
        let actual = match gix_url::parse(url) {
            Ok(actual) => actual,
            Err(err) => {
                failures.push(err.to_string());
                continue;
            }
        };
        let url_as_string = actual.to_bstring();
        serialized_url_does_not_match_input += usize::from(url_as_string != *url);
        failed_roundtrips += usize::from(gix_url::parse(url_as_string.as_ref()).ok().as_ref() != Some(&actual));
        let result = std::panic::catch_unwind(|| assert_urls_equal(expected, &actual)).map_err(|panic| {
            match downcast_panic_to_str(&panic) {
                Some(s) => format!("{url}: {s}\nexpected: {expected:?}\nactual: {actual:?}"),
                None => format!("{url}: expected: {expected:?}\nactual: {actual:?}"),
            }
        });
        if let Err(message) = result {
            failures.push(message);
        }
    }

    std::panic::set_hook(panic_hook);
    assert_ne!(count, 0, "the baseline is never empty");
    if failures.is_empty() {
        todo!("The baseline is currently meddling with hooks, thats not needed anymore since the failure rate is 0: move this into a module of the normal tests");
    }
    for message in &failures {
        eprintln!("{message}");
    }
    eprintln!(
        "{} failed out of {count} tests ({} passed)",
        failures.len(),
        count - failures.len()
    );
    assert!(
        serialized_url_does_not_match_input <= 126,
        "we shouldn't get worse when serializing to match our input URL"
    );

    let kind = baseline::Kind::new();
    assert_eq!(failed_roundtrips, 0);
    assert!(
        failures.len() <= kind.max_num_failures(),
        "Expected no more than {} failures, but got {} - this should get better, not worse",
        kind.max_num_failures(),
        failures.len(),
    )
}

fn downcast_panic_to_str<'a>(panic: &'a Box<dyn Any + Send + 'static>) -> Option<&'a str> {
    // Succeeds whenever `panic!` was given a string literal (for example if
    // `assert!` is given a string literal).
    match panic.downcast_ref::<&'static str>() {
        Some(s) => Some(s),
        None => {
            // Succeeds whenever `panic!` was given an owned String (for
            // example when using the `format!` syntax and always for
            // `assert_*!` macros).
            match panic.downcast_ref::<String>() {
                Some(s) => Some(s),
                None => None,
            }
        }
    }
}

fn assert_urls_equal(expected: &baseline::GitDiagUrl<'_>, actual: &gix_url::Url) {
    assert_eq!(
        actual.scheme,
        gix_url::Scheme::from(expected.protocol.to_str().unwrap()),
    );

    match expected.host {
        baseline::GitDiagHost::NonSsh { host_and_port } => match host_and_port {
            Some(expected_host_and_port) if !expected_host_and_port.is_empty() => {
                assert!(actual.host().is_some());

                let mut actual_host_and_port = String::new();
                if let Some(user) = actual.user() {
                    actual_host_and_port.push_str(user);
                    actual_host_and_port.push('@');
                }

                actual_host_and_port.push_str(actual.host().unwrap());

                if let Some(port) = actual.port {
                    actual_host_and_port.push(':');
                    actual_host_and_port.push_str(&port.to_string());
                }

                assert_eq!(actual_host_and_port, expected_host_and_port);
            }
            _ => {
                assert!(actual.host().is_none());
                assert!(actual.port.is_none());
            }
        },
        baseline::GitDiagHost::Ssh { user_and_host, port } => {
            match user_and_host {
                Some(expected_user_and_host) => {
                    assert!(actual.host().is_some());

                    let mut actual_user_and_host = String::new();
                    if let Some(user) = actual.user() {
                        actual_user_and_host.push_str(user);
                        actual_user_and_host.push('@');
                    }
                    actual_user_and_host.push_str(actual.host().unwrap());

                    assert_eq!(actual_user_and_host, expected_user_and_host);
                }
                None => {
                    assert!(actual.host().is_none());
                    assert!(actual.user().is_none());
                }
            }
            assert_eq!(actual.port.map(|p| p.to_string()), port.map(ToString::to_string));
        }
    }

    assert_eq!(actual.path, expected.path.unwrap_or_default());
}

mod baseline {
    use bstr::{BStr, BString, ByteSlice};
    use gix_testtools::once_cell::sync::Lazy;

    pub enum Kind {
        Unix,
        Windows,
    }

    impl Kind {
        pub const fn new() -> Self {
            if cfg!(windows) {
                Kind::Windows
            } else {
                Kind::Unix
            }
        }

        pub fn max_num_failures(&self) -> usize {
            match self {
                Kind::Unix => 165,
                Kind::Windows => 171,
            }
        }

        pub fn extension(&self) -> &'static str {
            match self {
                Kind::Unix => "unix",
                Kind::Windows => "windows",
            }
        }
    }

    static BASELINE: Lazy<BString> = Lazy::new(|| {
        let base = gix_testtools::scripted_fixture_read_only("make_baseline.sh").unwrap();
        std::fs::read(base.join(format!("git-baseline.{}", Kind::new().extension())))
            .expect("fixture file exists")
            .into()
    });

    pub static URLS: Lazy<Vec<(&'static BStr, GitDiagUrl<'static>)>> = Lazy::new(|| {
        let mut out = Vec::new();

        let blocks = BASELINE
            .split(|c| c == &b';')
            .filter(|block| !block.is_empty())
            .map(ByteSlice::trim);

        for block in blocks {
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
        /// [git_src]: https://github.com/git/git/blob/bcb6cae2966cc407ca1afc77413b3ef11103c175/connect.c#L1415
        fn parse(diag_url: &BStr) -> (&'_ BStr, GitDiagUrl<'_>) {
            fn null_is_none(input: &BStr) -> Option<&BStr> {
                if input == "NULL" || input == "NONE" {
                    None
                } else {
                    Some(input)
                }
            }
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
                    user_and_host: null_is_none(user_and_host),
                    port: null_is_none(port),
                }
            } else {
                let host_and_port = next_attr("hostandport");
                GitDiagHost::NonSsh {
                    host_and_port: null_is_none(host_and_port),
                }
            };

            let path = next_attr("path");
            assert!(lines.next().is_none(), "we consume everything");
            (
                url,
                GitDiagUrl {
                    protocol,
                    host,
                    path: null_is_none(path),
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
