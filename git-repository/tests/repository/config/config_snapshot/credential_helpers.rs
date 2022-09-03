mod baseline {
    use crate::remote;
    use git_object::bstr::BString;
    use git_testtools::once_cell::sync::Lazy;
    use std::collections::HashMap;

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
                    helpers.push(helper.to_owned().into());
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

    use git_repository as git;
    pub fn works_but_we_dont_parse_invalid_url(url: &str) {
        assert!(
            git::url::parse(url.into()).is_err(),
            "{:?} should not be parseable",
            url
        );
        assert!(
            BASELINE.get(url).is_some(),
            "Url {} must be in baseline, whether it's valid or not",
            url
        );
    }

    fn agrees_with_inner(url: &str, ignore_expected_prompt_port: bool) {
        let repo = remote::repo("credential-helpers");
        let (cascade, mut action) = repo
            .config_snapshot()
            .credential_helpers(git::url::parse(url.into()).expect("valid input URL"))
            .unwrap();

        let actual_helpers: Vec<BString> = cascade
            .programs
            .iter()
            .map(|p| match &p.kind {
                git_credentials::program::Kind::ExternalName { name_and_args } => {
                    name_and_args.strip_prefix(b"git ").expect("resolved name").into()
                }
                _ => panic!("need name helper"),
            })
            .collect();

        let expected = BASELINE
            .get(url)
            .unwrap_or_else(|| panic!("Url {} must be in baseline.", url));
        assert_eq!(actual_helpers, expected.helpers, "{}", url);

        let ctx = action.context_mut().expect("get/fill");
        ctx.destructure_url_in_place(cascade.use_http_path).unwrap();
        if ignore_expected_prompt_port {
            assert_eq!(
                ctx.to_url().expect("parts complete"),
                expected
                    .prompt_url
                    .trim_end_matches(|b: char| b == ':' || b.is_numeric())
            );
        } else {
            assert_eq!(ctx.to_url().expect("parts complete"), expected.prompt_url);
        }
    }

    pub fn agrees_with(url: &str) {
        agrees_with_inner(url, false)
    }

    pub fn agrees_with_but_drops_default_port_in_prompt(url: &str) {
        agrees_with_inner(url, true)
    }
}

#[test]
fn any_url_calls_global() {
    baseline::agrees_with("https://hit-global.helper");
}

#[test]
fn https_urls_match_the_host_without_path_as_well() {
    baseline::agrees_with("https://example.com:8080/other/path");
    baseline::agrees_with("https://example.com:8080/path");
    baseline::agrees_with("https://example.com:8080/path/");
}

#[test]
fn empty_helper_clears_helper_list() {
    baseline::agrees_with("https://example.com:8080/clear");
}

#[test]
fn http_urls_match_the_host_without_path_as_well() {
    baseline::agrees_with("http://example.com:8080/other/path");
    baseline::agrees_with_but_drops_default_port_in_prompt("http://example.com:80/");
    baseline::agrees_with_but_drops_default_port_in_prompt("http://example.com:80");
    baseline::agrees_with("http://example.com");
}

#[test]
fn user_rules_only_match_urls_with_user() {
    baseline::agrees_with("https://user@example.com/with-user");
    baseline::agrees_with("https://example.com/with-user");
    baseline::agrees_with("ssh://user@host/with-user");
    baseline::agrees_with("ssh://host/with-user");
}

#[test]
fn ssh_host_with_path_via_url_match() {
    baseline::agrees_with("ssh://host/path");
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
