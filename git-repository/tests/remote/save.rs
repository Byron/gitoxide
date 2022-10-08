mod save_as_to {
    use crate::basic_repo;
    use git_repository as git;
    use std::convert::TryInto;

    #[test]
    fn anonymous_remotes_cannot_be_save_lacking_a_name() -> crate::Result {
        let repo = basic_repo()?;
        let remote = repo.remote_at("https://example.com/path")?;
        assert!(matches!(
            remote.save_to(&mut git::config::File::default()).unwrap_err(),
            git::remote::save::Error::NameMissing { .. }
        ));
        Ok(())
    }

    #[test]
    fn new_anonymous_remote_with_name() -> crate::Result {
        let repo = basic_repo()?;
        let mut remote = repo
            .remote_at("https://example.com/path")?
            .push_url("https://ein.hub/path")?
            .with_refspec("+refs/heads/*:refs/remotes/any/*", git::remote::Direction::Fetch)?
            .with_refspec(
                "refs/heads/special:refs/heads/special-upstream",
                git::remote::Direction::Fetch,
            )?
            .with_refspec("refs/heads/main:refs/heads/main", git::remote::Direction::Push)? // similar to 'simple' for `push.default`
            .with_refspec(":", git::remote::Direction::Push)?; // similar to 'matching'
        let remote_name = "origin";
        assert!(
            repo.find_remote(remote_name).is_err(),
            "there is no remote of that name"
        );
        assert_eq!(remote.name(), None);
        let mut config = git::config::File::default();
        remote.save_as_to(remote_name.try_into().expect("valid name"), &mut config)?;
        let expected = "[remote \"origin\"]\n\turl = https://example.com/path\n\tpushurl = https://ein.hub/path\n\tfetch = +refs/heads/*:refs/remotes/any/*\n\tfetch = refs/heads/special:refs/heads/special-upstream\n\tpush = refs/heads/main:refs/heads/main\n\tpush = :\n";
        assert_eq!(uniformize(config.to_string()), expected);

        remote.save_as_to(remote_name.try_into().expect("valid name"), &mut config)?;
        assert_eq!(
            uniformize(config.to_string()),
            expected,
            "it appears to be idempotent in this case"
        );

        {
            let mut new_section = config.section_mut_or_create_new("unrelated", None).expect("works");
            new_section.push("a".try_into().unwrap(), Some("value".into()));

            config
                .section_mut_or_create_new("initially-empty-not-removed", Some("name"))
                .expect("works");

            let mut existing_section = config
                .section_mut_or_create_new("remote", Some("origin"))
                .expect("works");
            existing_section.push("free".try_into().unwrap(), Some("should not be removed".into()))
        }
        remote.save_as_to(remote_name.try_into().expect("valid name"), &mut config)?;
        assert_eq!(
            uniformize(config.to_string()),
            "[remote \"origin\"]\n\t\n\t\n\t\n\t\n\t\n\t\n\tfree = should not be removed\n\turl = https://example.com/path\n\tpushurl = https://ein.hub/path\n\tfetch = +refs/heads/*:refs/remotes/any/*\n\tfetch = refs/heads/special:refs/heads/special-upstream\n\tpush = refs/heads/main:refs/heads/main\n\tpush = :\n[unrelated]\n\ta = value\n[initially-empty-not-removed \"name\"]\n",
            "unrelated keys are kept, and so are keys in the sections we edit"
        );
        Ok(())
    }

    fn uniformize(input: String) -> String {
        input.replace("\r\n", "\n")
    }
}
