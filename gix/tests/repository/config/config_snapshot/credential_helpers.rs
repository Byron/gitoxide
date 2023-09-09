use crate::remote;
use gix_testtools::Env;

mod baseline {
    use std::collections::HashMap;

    use gix_object::bstr::BString;
    use gix_testtools::once_cell::sync::Lazy;

    use crate::remote;

    #[derive(Debug, Eq, PartialEq)]
    struct Helpers {
        pub prompt_url: String,
        pub helpers: Vec<BString>,
    }

    static BASELINE: Lazy<HashMap<String, Helpers>> = Lazy::new(|| {
        let base = remote::repo_path("credential-helpers");

        (|| -> crate::Result<_> {
            use std::io::BufRead;
            let mut map = HashMap::new();
            let baseline = std::fs::read(base.join("baseline.git"))?;
            let mut lines = baseline.lines().map(Result::unwrap).peekable();
            while let Some(url) = lines.next() {
                let mut helpers = Vec::new();
                while let Some(helper) = lines
                    .peek()
                    .and_then(|line| line.strip_prefix("git: '"))
                    .map(|h| &h[..h.find('\'').expect("closing")])
                {
                    helpers.push(
                        helper
                            .strip_prefix("credential-")
                            .expect("helpers start with 'credential-'")
                            .to_owned()
                            .into(),
                    );
                    lines.next();
                }
                let line = lines.next().expect("fatal:");
                let prompt_url = line
                    .strip_prefix("fatal: could not read Username for '")
                    .or_else(|| line.strip_prefix("fatal: could not read Password for '"))
                    .map(|url| &url[..url.find('\'').expect("closing")])
                    .unwrap()
                    .to_owned();
                map.insert(url, Helpers { prompt_url, helpers });
            }
            Ok(map)
        })()
        .unwrap()
    });

    pub fn works_but_we_dont_parse_invalid_url(url: &str) {
        assert!(gix::url::parse(url.into()).is_err(), "{url:?} should not be parseable");
        assert!(
            BASELINE.get(url).is_some(),
            "Url {url} must be in baseline, whether it's valid or not"
        );
    }

    fn agrees_with_inner(url: &str, ignore_expected_prompt_port: bool, lowercase_prompt_host: bool) {
        let repo = remote::repo("credential-helpers");
        let (cascade, mut action, prompt_options) = repo
            .config_snapshot()
            .credential_helpers(gix::url::parse(url.into()).expect("valid input URL"))
            .unwrap();

        assert_ne!(
            prompt_options.mode,
            gix_prompt::Mode::Disable,
            "isolated repos may show prompts"
        );
        assert!(
            prompt_options.askpass.is_none(),
            "isolation does not allow environment variables to be read"
        );
        let actual_helpers: Vec<BString> = cascade
            .programs
            .iter()
            .map(|p| match &p.kind {
                gix_credentials::program::Kind::ExternalName { name_and_args } => name_and_args
                    .strip_prefix(b"git credential-")
                    .expect("resolved name")
                    .into(),
                _ => panic!("need name helper"),
            })
            .collect();

        let expected = BASELINE
            .get(url)
            .unwrap_or_else(|| panic!("Url {url} must be in baseline."));
        assert_eq!(actual_helpers, expected.helpers, "{url}");

        let ctx = action.context_mut().expect("get/fill");
        ctx.destructure_url_in_place(cascade.use_http_path).unwrap();
        let expected_prompt = lowercase_prompt_host
            .then(|| expected.prompt_url.to_ascii_lowercase())
            .unwrap_or_else(|| expected.prompt_url.to_owned());
        if ignore_expected_prompt_port {
            assert_eq!(
                ctx.to_url().expect("parts complete"),
                expected_prompt.trim_end_matches(|b: char| b == ':' || b.is_numeric())
            );
        } else {
            assert_eq!(ctx.to_url().expect("parts complete"), expected_prompt);
        }
    }

    pub fn agrees_with(url: &str) {
        agrees_with_inner(url, false, false)
    }

    pub fn agrees_with_but_drops_default_port_in_prompt(url: &str) {
        agrees_with_inner(url, true, false)
    }
    pub fn agrees_with_but_lowercases_scheme_and_host(url: &str) {
        agrees_with_inner(url, false, true)
    }
}

#[test]
fn any_url_calls_global() {
    baseline::agrees_with("https://hit-global.helper");
}

#[test]
fn http_port_defaulting() {
    baseline::agrees_with("https://example.com");
    baseline::agrees_with("https://example.com/");
    baseline::agrees_with_but_drops_default_port_in_prompt("https://example.com:443");
    baseline::agrees_with_but_drops_default_port_in_prompt("https://example.com:443/");
}

#[test]
fn https_urls_match_the_host_without_path_as_well() {
    baseline::agrees_with("https://example.com:8080/other/path");
    baseline::agrees_with("https://example.com:8080/path");
    baseline::agrees_with("https://example.com:8080/PATH");
    baseline::agrees_with("https://example.com:8080/path/");
}

#[test]
fn empty_helper_clears_helper_list() {
    baseline::agrees_with("https://example.com:8080/clear");
}

#[test]
fn host_globs_match_as_well() {
    baseline::agrees_with("http://host");
}

#[test]
fn case_sensitive_host_matching() {
    baseline::agrees_with_but_lowercases_scheme_and_host("https://EXAMPLE.com");
    baseline::agrees_with_but_lowercases_scheme_and_host("https://example.COM");
    baseline::agrees_with_but_lowercases_scheme_and_host("HTTPS://example.com");
}

#[test]
fn subdomain_globs_match_on_their_level() {
    baseline::agrees_with("http://a.example.com");
    baseline::agrees_with("http://b.example.com/path");
    baseline::agrees_with_but_drops_default_port_in_prompt("http://c.example.com:80/path");
    baseline::agrees_with_but_drops_default_port_in_prompt("http://a.a.example.com:80/path");
    baseline::agrees_with("http://a.b.example.com/path");
    baseline::agrees_with("http://b.a.example.com/path");
}

#[test]
#[serial_test::serial]
fn http_urls_match_the_host_without_path_as_well() {
    let _env = Env::new().set("GIT_ASKPASS", "foo");
    baseline::agrees_with("http://example.com:8080/other/path");
    baseline::agrees_with_but_drops_default_port_in_prompt("http://example.com:80/");
    baseline::agrees_with_but_drops_default_port_in_prompt("http://example.com:80");
    baseline::agrees_with("http://example.com");
}

#[test]
#[serial_test::serial]
fn user_rules_only_match_urls_with_user() {
    let _env = Env::new().set("SSH_ASKPASS", "foo");
    baseline::agrees_with("https://user@example.com/with-user");
    baseline::agrees_with("https://example.com/with-user");
    baseline::agrees_with("ssh://user@host/with-user");
    baseline::agrees_with("ssh://host/with-user");
}

#[test]
fn ssh_host_with_path_via_url_match() {
    baseline::agrees_with("ssh://host/path");
    baseline::agrees_with("ssh://host/PATH");
}

#[test]
fn ssh_host_and_port_with_path_via_url_match() {
    baseline::agrees_with("ssh://host:21/path");
}

#[test]
fn invalid_urls_are_rejected_early() {
    baseline::works_but_we_dont_parse_invalid_url("ssh://host");
    baseline::works_but_we_dont_parse_invalid_url("ssh://host:21");
    baseline::works_but_we_dont_parse_invalid_url("git://host.org");
}

#[test]
fn empty_core_askpass_is_ignored() -> crate::Result {
    let repo = remote::repo("empty-core-askpass");
    let _ = repo
        .config_snapshot()
        .credential_helpers("does-not-matter".try_into()?)?;
    Ok(())
}
