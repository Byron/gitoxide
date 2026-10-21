mod bare {
    use gix_testtools::TestResult;
    use gix_testtools::tempfile;

    #[test]
    #[serial_test::serial]
    fn init_into_non_existing_directory_creates_it() -> TestResult {
        let _environment = gix_testtools::isolate_git_environment()?;
        let tmp = tempfile::tempdir()?;
        let git_dir = tmp.path().join("bare.git");
        let repo = gix::init_bare(&git_dir)?;
        assert_eq!(repo.kind(), gix::repository::Kind::Common);
        assert!(
            repo.workdir().is_none(),
            "a worktree isn't present in bare repositories"
        );
        assert_eq!(
            repo.git_dir(),
            git_dir,
            "the repository is placed into the given directory without added sub-directories"
        );
        assert_eq!(gix::open_opts(repo.git_dir(), crate::restricted())?, repo);
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn init_into_empty_directory_uses_it_directly() -> TestResult {
        let _environment = gix_testtools::isolate_git_environment()?;
        #[cfg(feature = "sha1")]
        let (expected, version, object_format) = (gix_hash::Kind::Sha1, 0, None);
        #[cfg(not(feature = "sha1"))]
        let (expected, version, object_format) = (gix_hash::Kind::Sha256, 1, Some("sha256"));
        for options in [
            gix::create::Options::default(),
            gix::create::Options {
                object_hash: None,
                ..Default::default()
            },
        ] {
            let tmp = tempfile::tempdir()?;
            let repo = gix::ThreadSafeRepository::init_opts(
                tmp.path(),
                gix::create::Kind::Bare,
                options,
                gix::open::Options::isolated(),
            )?
            .to_thread_local();
            assert_eq!(repo.kind(), gix::repository::Kind::Common);
            assert!(
                repo.workdir().is_none(),
                "a worktree isn't present in bare repositories"
            );
            assert_eq!(
                repo.git_dir(),
                tmp.path(),
                "the repository is placed into the directory itself"
            );
            assert_eq!(
                repo.object_hash(),
                expected,
                "initialization uses a supported object hash"
            );
            super::object_format::assert_on_disk_format(&repo, version, object_format)?;
            assert_eq!(
                gix_testtools::git(repo.git_dir(), "rev-parse --show-object-format")?.trim(),
                expected.to_string(),
                "git agrees on the object format"
            );
            assert_eq!(gix::open_opts(repo.git_dir(), crate::restricted())?, repo);
        }
        Ok(())
    }

    #[test]
    fn init_into_non_empty_directory_is_not_allowed() -> TestResult {
        let tmp = tempfile::tempdir()?;
        std::fs::write(tmp.path().join("existing.txt"), b"I was here before you")?;

        insta::assert_debug_snapshot!(gix_testtools::redact_debug_snapshot(&(gix::init_bare(tmp.path())
                .expect_err("init into non empty directory is not allowed")), &[(&(tmp.path()).to_string_lossy(), "<destination>")]), "init into non empty directory is not allowed", @r#"Refusing to initialize the non-empty directory as, input="<destination>""#);
        Ok(())
    }
}

mod non_bare {
    use gix_testtools::TestResult;
    use gix_testtools::tempfile;

    #[test]
    fn init_bare_with_custom_branch_name() -> TestResult {
        let tmp = tempfile::tempdir()?;
        let repo: gix::Repository = gix::ThreadSafeRepository::init_opts(
            tmp.path(),
            gix::create::Kind::Bare,
            gix::create::Options::default(),
            gix::open::Options::isolated().config_overrides([
                "user.name=a",
                "user.email=b",
                "init.defaultBranch=special",
            ]),
        )?
        .into();
        assert_eq!(repo.head()?.referent_name().expect("name"), "refs/heads/special");
        Ok(())
    }

    #[test]
    fn init_bare_with_fully_qualified_custom_branch_name_is_not_prefixed_again() -> TestResult {
        let tmp = tempfile::tempdir()?;
        let repo: gix::Repository = gix::ThreadSafeRepository::init_opts(
            tmp.path(),
            gix::create::Kind::Bare,
            gix::create::Options::default(),
            gix::open::Options::isolated().config_overrides([
                "user.name=a",
                "user.email=b",
                "init.defaultBranch=refs/heads/special",
            ]),
        )?
        .into();
        assert_eq!(repo.head()?.referent_name().expect("name"), "refs/heads/special");
        assert_eq!(
            repo.is_pristine(),
            Some(true),
            "the expected default ref uses the de-duplicated fully qualified branch name"
        );
        Ok(())
    }

    #[test]
    fn init_bare_rejects_reserved_branch_name() -> TestResult {
        let tmp = tempfile::tempdir()?;
        let err = gix::ThreadSafeRepository::init_opts(
            tmp.path(),
            gix::create::Kind::Bare,
            gix::create::Options::default(),
            gix::open::Options::isolated().config_overrides(["user.name=a", "user.email=b", "init.defaultBranch=HEAD"]),
        )
        .unwrap_err();
        insta::assert_debug_snapshot!(err, "init bare rejects reserved branch name", @r#"
        Invalid default branch name, input="HEAD"

        Caused by:
            0: Reference name is reserved and cannot be used: "refs/heads/HEAD"
        "#);
        assert!(matches!(
            err.classify().filter(|classification| classification.class() == gix_error::Class::Validation)
                .find_map(|classification| classification.error().downcast_ref::<gix_error::Message>()),
            Some(gix_error::Message { values, .. }) if values.get("input") == Some(&gix_error::MetadataValue::Bytes("HEAD".into()))
        ));
        assert!(matches!(
            err.downcast_any_ref(),
            Some(gix_validate::reference::name::Error::Reserved { name }) if name == "refs/heads/HEAD"
        ));
        Ok(())
    }

    #[test]
    fn init_bare_rejects_reserved_fully_qualified_branch_name() -> TestResult {
        let tmp = tempfile::tempdir()?;
        let err = gix::ThreadSafeRepository::init_opts(
            tmp.path(),
            gix::create::Kind::Bare,
            gix::create::Options::default(),
            gix::open::Options::isolated().config_overrides([
                "user.name=a",
                "user.email=b",
                "init.defaultBranch=refs/heads/HEAD",
            ]),
        )
        .unwrap_err();
        insta::assert_debug_snapshot!(err, "init bare rejects reserved fully qualified branch name", @r#"
        Invalid default branch name, input="refs/heads/HEAD"

        Caused by:
            0: Reference name is reserved and cannot be used: "refs/heads/HEAD"
        "#);
        assert!(matches!(
            err.classify().filter(|classification| classification.class() == gix_error::Class::Validation)
                .find_map(|classification| classification.error().downcast_ref::<gix_error::Message>()),
            Some(gix_error::Message { values, .. }) if values.get("input") == Some(&gix_error::MetadataValue::Bytes("refs/heads/HEAD".into()))
        ));
        assert!(matches!(
            err.downcast_any_ref(),
            Some(gix_validate::reference::name::Error::Reserved { name }) if name == "refs/heads/HEAD"
        ));
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn init_into_empty_directory_creates_a_dot_git_dir() -> TestResult {
        let _environment = gix_testtools::isolate_git_environment()?;
        let tmp = tempfile::tempdir()?;
        let repo = gix::init(tmp.path())?;
        assert_eq!(repo.kind(), gix::repository::Kind::Common);
        assert_eq!(repo.workdir(), Some(tmp.path()), "there is a work tree by default");
        assert_eq!(
            repo.git_dir(),
            tmp.path().join(".git"),
            "there is a work tree by default"
        );
        assert_eq!(gix::open_opts(repo.git_dir(), crate::restricted())?, repo);
        assert_eq!(
            gix::open_opts(repo.workdir().as_ref().expect("non-bare repo"), crate::restricted())?,
            repo
        );
        Ok(())
    }

    #[test]
    fn init_into_non_empty_directory_is_allowed_if_option_is_none_or_false() -> TestResult {
        for destination_must_be_empty in [None, Some(false)] {
            let tmp = tempfile::tempdir()?;
            std::fs::write(tmp.path().join("existing.txt"), b"I was here before you")?;
            let repo: gix::Repository = gix::ThreadSafeRepository::init_opts(
                tmp.path(),
                gix::create::Kind::WithWorktree,
                gix::create::Options {
                    destination_must_be_empty,
                    ..Default::default()
                },
                gix::open::Options::isolated(),
            )?
            .into();
            assert_eq!(repo.workdir().expect("present"), tmp.path());
            assert_eq!(
                repo.git_dir(),
                tmp.path().join(".git"),
                "gitdir is inside of the workdir"
            );
        }
        Ok(())
    }

    #[test]
    fn init_into_non_empty_directory_is_not_allowed_if_option_is_true() -> TestResult {
        let tmp = tempfile::tempdir()?;
        std::fs::write(tmp.path().join("existing.txt"), b"I was here before you")?;

        let err = gix::ThreadSafeRepository::init_opts(
            tmp.path(),
            gix::create::Kind::WithWorktree,
            gix::create::Options {
                destination_must_be_empty: Some(true),
                ..Default::default()
            },
            gix::open::Options::isolated(),
        )
        .unwrap_err();
        insta::assert_debug_snapshot!(gix_testtools::redact_debug_snapshot(&(err), &[(&(tmp.path()).to_string_lossy(), "<destination>")]), "init into non empty directory is not allowed if option is true", @r#"Refusing to initialize the non-empty directory as, input="<destination>""#);
        Ok(())
    }
}

mod object_format {
    use gix_testtools::{TestResult, tempfile};

    fn init(
        kind: gix::create::Kind,
        object_hash: Option<gix_hash::Kind>,
    ) -> TestResult<(tempfile::TempDir, gix::Repository)> {
        let tmp = tempfile::tempdir()?;
        let repo: gix::Repository = gix::ThreadSafeRepository::init_opts(
            tmp.path(),
            kind,
            gix::create::Options {
                object_hash,
                ..Default::default()
            },
            crate::restricted(),
        )?
        .into();
        Ok((tmp, repo))
    }

    pub(super) fn assert_on_disk_format(
        repo: &gix::Repository,
        version: i64,
        object_format: Option<&str>,
    ) -> TestResult {
        let config =
            gix::config::File::from_path_no_includes(repo.git_dir().join("config"), gix::config::Source::Local)?;
        assert_eq!(
            config.integer("core.repositoryFormatVersion")?,
            Some(version),
            "the repository format version matches what git writes for this object format"
        );
        assert_eq!(
            config.string("extensions.objectFormat"),
            object_format.map(Into::into),
            "only non-SHA-1 object formats are written as extension"
        );
        Ok(())
    }

    #[test]
    #[cfg(feature = "sha256")]
    fn sha256_is_written_as_version_1_extension_and_understood_by_git() -> TestResult {
        for kind in [gix::create::Kind::WithWorktree, gix::create::Kind::Bare] {
            let (_tmp, repo) = init(kind, Some(gix_hash::Kind::Sha256))?;
            assert_eq!(
                repo.object_hash(),
                gix_hash::Kind::Sha256,
                "the requested object format is used"
            );
            assert_on_disk_format(&repo, 1, Some("sha256"))?;
            assert_eq!(
                gix::open_opts(repo.git_dir(), crate::restricted())?.object_hash(),
                gix_hash::Kind::Sha256,
                "the format survives reopening"
            );
            assert_eq!(
                gix_testtools::git(repo.git_dir(), "rev-parse --show-object-format")?.trim(),
                "sha256",
                "git agrees on the object format"
            );
        }
        Ok(())
    }

    #[test]
    #[cfg(feature = "sha1")]
    fn sha1_is_written_like_git_without_extension() -> TestResult {
        let (_tmp, repo) = init(gix::create::Kind::Bare, Some(gix_hash::Kind::Sha1))?;
        assert_eq!(
            repo.object_hash(),
            gix_hash::Kind::Sha1,
            "the requested object format is used"
        );
        assert_on_disk_format(&repo, 0, None)?;
        assert_eq!(
            gix_testtools::git(repo.git_dir(), "rev-parse --show-object-format")?.trim(),
            "sha1",
            "git agrees on the object format"
        );
        Ok(())
    }
}
