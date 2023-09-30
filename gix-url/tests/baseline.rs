use std::any::Any;

use bstr::ByteSlice;
use gix_testtools::once_cell::sync::Lazy;

/// To see all current failures run the following command or execute cargo-nextest directly with
/// the below shown arguments.
///
/// ```bash
/// just nt -p gix-url --test baseline --success-output immediate
/// ``
#[test]
fn run() {
    // ensure the baseline is evaluated before we disable the panic hook, otherwise we swallow
    // errors inside the baseline generation
    Lazy::force(&baseline::URLS);

    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let mut test_count = 0;
    let mut failures = Vec::new();
    let (mut failure_count_roundtrips, mut failure_count_reserialization) = (0, 0);
    for (url, expected) in baseline::URLS.iter() {
        test_count += 1;
        let actual = match gix_url::parse(url) {
            Ok(actual) => actual,
            Err(message) => {
                failures.push(format!("failure(parse): {message}"));
                continue;
            }
        };
        if let Err(message) = std::panic::catch_unwind(|| assert_urls_equal(expected, &actual)).map_err(|panic| {
            match downcast_panic_to_str(&panic) {
                Some(s) => format!("{url}: {s}\nexpected: {expected:?}\nactual: {actual:?}"),
                None => format!("{url}: expected: {expected:?}\nactual: {actual:?}"),
            }
        }) {
            failures.push(format!("failure(compare): {message}"));
            continue;
        }
        // perform additional checks only after we determined that we parsed the url correctly
        let url_serialized_again = actual.to_bstring();
        failure_count_reserialization += usize::from(url_serialized_again != *url);
        failure_count_roundtrips +=
            usize::from(gix_url::parse(url_serialized_again.as_ref()).ok().as_ref() != Some(&actual));
    }

    std::panic::set_hook(panic_hook);

    assert_ne!(test_count, 0, "the baseline is never empty");
    if failures.is_empty() {
        todo!("The baseline is currently meddling with hooks, thats not needed anymore since the failure rate is 0: move this into a module of the normal tests");
    }

    let failure_count = failures.len();
    let passed_count = test_count - failure_count;
    let expected_failure_count = baseline::Kind::new().max_num_failures();

    eprintln!("failed {failure_count}/{test_count} tests ({passed_count} passed)");

    for message in &failures {
        // print messages to out instead of err to separate them from general test information
        println!("{message}");
    }

    use core::cmp::Ordering;
    match Ord::cmp(&failure_count, &expected_failure_count) {
        Ordering::Equal => {
            eprintln!("the number of failing tests is as expected");
        }
        Ordering::Less => {
            panic!(
                "{} more passing tests than expected. Great work! Please change the expected number of failures to {failure_count} to make this panic go away",
                expected_failure_count - failure_count,
            )
        }
        Ordering::Greater => {
            panic!(
                "{} more failing tests than expected! This should get better, not worse. Please check your changes manually for any regressions",
                failure_count - expected_failure_count,
            )
        }
    }

    assert!(
        failure_count_reserialization <= 42,
        "the number of reserialization errors should ideally get better, not worse - if this panic is not due to regressions but to new passing test cases, you can set this check to {failure_count_reserialization}"
    );
    assert_eq!(failure_count_roundtrips, 0, "there should be no roundtrip errors");
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
