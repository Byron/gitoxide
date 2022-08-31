use crate::named_repo;

#[test]
fn values_are_set_in_memory_only() {
    let mut repo = named_repo("make_config_repo.sh").unwrap();
    let repo_clone = repo.clone();
    let key = "hallo.welt";
    let key_subsection = "hallo.unter.welt";
    assert_eq!(repo.config_snapshot().boolean(key), None, "no value there just yet");
    assert_eq!(repo.config_snapshot().string(key_subsection), None);

    {
        let mut config = repo.config_snapshot_mut();
        config.set_raw_value("hallo", None, "welt", "true").unwrap();
        config.set_raw_value("hallo", Some("unter"), "welt", "value").unwrap();
    }

    assert_eq!(
        repo.config_snapshot().boolean(key),
        Some(true),
        "value was set and applied"
    );
    assert_eq!(
        repo.config_snapshot().string(key_subsection).as_deref(),
        Some("value".into())
    );

    assert_eq!(
        repo_clone.config_snapshot().boolean(key),
        None,
        "values are not written back automatically nor are they shared between clones"
    );
    assert_eq!(repo_clone.config_snapshot().string(key_subsection), None);
}

#[test]
fn apply_cli_overrides() -> crate::Result {
    let mut repo = named_repo("make_config_repo.sh").unwrap();
    repo.config_snapshot_mut().apply_cli_overrides([
        "a.b=c",
        "remote.origin.url = url",
        "implicit.bool-true",
        "implicit.bool-false = ",
    ])?;

    let config = repo.config_snapshot();
    assert_eq!(config.string("a.b").expect("present").as_ref(), "c");
    assert_eq!(config.string("remote.origin.url").expect("present").as_ref(), "url");
    assert_eq!(
        config.string("implicit.bool-true"),
        None,
        "no keysep is interpreted as 'not present' as we don't make up values"
    );
    assert_eq!(
        config.string("implicit.bool-false").expect("present").as_ref(),
        "",
        "empty values are fine"
    );
    assert_eq!(
        config.boolean("implicit.bool-false"),
        Some(false),
        "empty values are boolean true"
    );
    assert_eq!(
        config.boolean("implicit.bool-true"),
        Some(true),
        "values without key-sep are true"
    );

    Ok(())
}

mod credential_helpers {
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
                    let prompt_url = lines
                        .next()
                        .expect("fatal:")
                        .strip_prefix("fatal: could not read Username for '")
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
        pub fn agrees_with(url: &str) {
            let repo = remote::repo("credential-helpers");
            let (cascade, mut action) = repo
                .config_snapshot()
                .credential_helpers(git::url::parse(url.into()).expect("valid input URL"))
                .unwrap();

            let actual_helpers: Vec<_> = cascade
                .programs
                .iter()
                .map(|p| match &p.kind {
                    git_credentials::program::Kind::ExternalName { name_and_args } => name_and_args.to_owned(),
                    _ => panic!("need name helper"),
                })
                .collect();

            let expected = BASELINE
                .get(url)
                .unwrap_or_else(|| panic!("Url {} must be in baseline.", url));
            assert_eq!(actual_helpers, expected.helpers);

            let ctx = action.context_mut().expect("get/fill");
            ctx.destructure_url_in_place(cascade.use_http_path).unwrap();
            assert_eq!(ctx.to_url().expect("parts complete"), expected.prompt_url);
        }
    }

    #[test]
    #[ignore]
    fn any_url_calls_global() {
        baseline::agrees_with("https://hit-global.helper");
    }
}
