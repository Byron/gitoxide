mod missing_config_file {

    use crate::util::named_subrepo_opts;

    #[test]
    fn bare() -> crate::Result {
        let repo = named_subrepo_opts("make_config_repos.sh", "bare-no-config", gix::open::Options::isolated())?;
        assert!(
            repo.is_bare(),
            "without config, we can't really know what the repo is actually but can guess by not having a worktree"
        );
        assert_eq!(repo.work_dir(), None);
        assert!(repo.worktree().is_none());
        assert_eq!(
            repo.config_snapshot().meta().source,
            gix::config::Source::Local,
            "config always refers to the local one for safety"
        );
        Ok(())
    }

    #[test]
    fn non_bare() -> crate::Result {
        let repo = named_subrepo_opts(
            "make_config_repos.sh",
            "worktree-no-config",
            gix::open::Options::isolated(),
        )?;
        assert!(repo.work_dir().is_some());
        assert!(repo.worktree().is_some());
        assert!(
            !repo.is_bare(),
            "without config, we can't really know what the repo is actually but can guess as there is a worktree"
        );
        assert_eq!(
            repo.config_snapshot().meta().source,
            gix::config::Source::Local,
            "config always refers to the local one for safety"
        );
        Ok(())
    }
}

mod not_a_repository {

    #[test]
    fn shows_proper_error() -> crate::Result {
        for name in ["empty-dir", "with-files"] {
            let name = format!("not-a-repo-{name}");
            let repo_path = gix_testtools::scripted_fixture_read_only("make_config_repos.sh")?.join(name);
            let err = gix::open_opts(&repo_path, gix::open::Options::isolated()).unwrap_err();
            assert!(matches!(err, gix::open::Error::NotARepository { path, .. } if path == repo_path));
        }
        Ok(())
    }
}

mod open_path_as_is {

    use crate::util::{named_subrepo_opts, repo_opts};

    fn open_path_as_is() -> gix::open::Options {
        gix::open::Options::isolated().open_path_as_is(true)
    }

    #[test]
    fn bare_repos_open_normally() -> crate::Result {
        assert!(named_subrepo_opts("make_basic_repo.sh", "bare.git", open_path_as_is())?.is_bare());
        Ok(())
    }

    #[test]
    fn worktrees_cannot_be_opened() -> crate::Result {
        let err = repo_opts("make_basic_repo.sh", open_path_as_is()).unwrap_err();
        assert!(matches!(err, gix::open::Error::NotARepository { .. }));
        Ok(())
    }

    #[test]
    fn git_dir_within_worktrees_open_normally() -> crate::Result {
        assert!(!named_subrepo_opts("make_basic_repo.sh", ".git", open_path_as_is())?.is_bare());
        Ok(())
    }
}

mod submodules {
    use std::path::Path;

    #[test]
    fn by_their_worktree_checkout_and_git_modules_dir() {
        let dir = gix_testtools::scripted_fixture_read_only("make_submodules.sh").unwrap();
        let parent_repo = Path::new("with-submodules");
        let modules = parent_repo.join(".git").join("modules");
        for module in ["m1", "dir/m1"] {
            let submodule_m1_workdir = parent_repo.join(module);
            let submodule_m1_gitdir = modules.join(module);

            for discover_dir in [
                submodule_m1_workdir.clone(),
                submodule_m1_workdir.join("subdir"),
                submodule_m1_gitdir.clone(),
            ] {
                let repo = discover_repo(discover_dir).unwrap();
                // assert_eq!(repo.kind(), gix::Kind::Submodule);
                assert_eq!(repo.work_dir().expect("non-bare"), dir.join(&submodule_m1_workdir));
                assert_eq!(repo.git_dir(), dir.join(&submodule_m1_gitdir));

                let repo = gix::open_opts(repo.work_dir().expect("non-bare"), gix::open::Options::isolated()).unwrap();
                assert_eq!(repo.kind(), gix::Kind::Submodule);
                assert_eq!(repo.work_dir().expect("non-bare"), dir.join(&submodule_m1_workdir));
                assert_eq!(repo.git_dir(), dir.join(&submodule_m1_gitdir));
            }
        }
    }

    fn discover_repo(name: impl AsRef<Path>) -> crate::Result<gix::Repository> {
        let dir = gix_testtools::scripted_fixture_read_only("make_submodules.sh")?;
        let repo_dir = dir.join(name);
        Ok(gix::ThreadSafeRepository::discover_opts(
            repo_dir,
            Default::default(),
            gix_sec::trust::Mapping {
                full: crate::restricted(),
                reduced: crate::restricted(),
            },
        )?
        .to_thread_local())
    }
}

mod object_caches {

    use crate::util::named_subrepo_opts;

    #[test]
    fn default_git_and_custom_caches() -> crate::Result {
        let opts = gix::open::Options::isolated();
        let repo = named_subrepo_opts("make_config_repos.sh", "object-caches", opts)?;
        assert!(repo.objects.has_object_cache());
        assert!(repo.objects.has_pack_cache());
        Ok(())
    }

    #[test]
    fn disabled() -> crate::Result {
        let opts = gix::open::Options::isolated();
        let repo = named_subrepo_opts("make_config_repos.sh", "disabled-object-caches", opts)?;
        assert!(!repo.objects.has_object_cache());
        assert!(!repo.objects.has_pack_cache());
        Ok(())
    }
}

mod with_overrides {
    use std::borrow::Cow;

    use gix_object::bstr::BStr;
    use gix_sec::Permission;
    use gix_testtools::Env;
    use serial_test::serial;

    use crate::util::named_subrepo_opts;

    #[test]
    #[serial]
    fn order_from_api_and_cli_and_environment() -> crate::Result {
        let default_date = "1979-02-26 18:30:00";
        let _env = Env::new()
            .set("GIT_HTTP_USER_AGENT", "agent-from-env")
            .set("GIT_HTTP_LOW_SPEED_LIMIT", "1")
            .set("GIT_HTTP_LOW_SPEED_TIME", "1")
            .set("GIT_HTTP_PROXY_AUTHMETHOD", "proxy-auth-method-env")
            .set("GIT_CURL_VERBOSE", "true")
            .set("https_proxy", "https-lower-override")
            .set("HTTPS_PROXY", "https-upper")
            .set("http_proxy", "http-lower")
            .set("all_proxy", "all-proxy-lower")
            .set("ALL_PROXY", "all-proxy")
            .set("no_proxy", "no-proxy-lower")
            .set("NO_PROXY", "no-proxy")
            .set("GIT_PROTOCOL_FROM_USER", "file-allowed")
            .set("GIT_REPLACE_REF_BASE", "refs/replace-mine")
            .set("GIT_NO_REPLACE_OBJECTS", "no-replace")
            .set("GIT_COMMITTER_NAME", "committer name")
            .set("GIT_COMMITTER_EMAIL", "committer email")
            .set("GIT_COMMITTER_DATE", default_date)
            .set("GIT_AUTHOR_NAME", "author name")
            .set("GIT_AUTHOR_EMAIL", "author email")
            .set("GIT_AUTHOR_DATE", default_date)
            .set("EMAIL", "user email")
            .set("GITOXIDE_PACK_CACHE_MEMORY", "0")
            .set("GITOXIDE_OBJECT_CACHE_MEMORY", "5m")
            .set("GIT_SSL_CAINFO", "./env.pem")
            .set("GIT_SSL_VERSION", "tlsv1.3")
            .set("GIT_SSH_VARIANT", "ssh-variant-env")
            .set("GIT_SSH_COMMAND", "ssh-command-env")
            .set("GIT_SSH", "ssh-command-fallback-env");
        let mut opts = gix::open::Options::isolated()
            .cli_overrides([
                "http.userAgent=agent-from-cli",
                "http.lowSpeedLimit=3",
                "http.lowSpeedTime=3",
                "http.sslCAInfo=./cli.pem",
                "http.sslVersion=sslv3",
                "ssh.variant=ssh-variant-cli",
                "core.sshCommand=ssh-command-cli",
                "gitoxide.ssh.commandWithoutShellFallback=ssh-command-fallback-cli",
                "gitoxide.http.proxyAuthMethod=proxy-auth-method-cli",
            ])
            .config_overrides([
                "http.userAgent=agent-from-api",
                "http.lowSpeedLimit=2",
                "http.lowSpeedTime=2",
                "http.sslCAInfo=./api.pem",
                "http.sslVersion=tlsv1",
                "ssh.variant=ssh-variant-api",
                "core.sshCommand=ssh-command-api",
                "gitoxide.ssh.commandWithoutShellFallback=ssh-command-fallback-api",
                "gitoxide.http.proxyAuthMethod=proxy-auth-method-api",
            ]);
        opts.permissions.env.git_prefix = Permission::Allow;
        opts.permissions.env.http_transport = Permission::Allow;
        opts.permissions.env.identity = Permission::Allow;
        opts.permissions.env.objects = Permission::Allow;
        let repo = named_subrepo_opts("make_config_repos.sh", "http-config", opts)?;
        assert_eq!(
            repo.config_snapshot().meta().source,
            gix::config::Source::Local,
            "config always refers to the local one for safety"
        );
        let config = repo.config_snapshot();
        assert_eq!(
            config.strings_by_key("http.userAgent").expect("at least one value"),
            [
                cow_bstr("agentJustForHttp"),
                cow_bstr("agent-from-cli"),
                cow_bstr("agent-from-api"),
                cow_bstr("agent-from-env")
            ]
        );
        assert_eq!(
            config
                .integers_by_key("http.lowSpeedLimit")
                .transpose()?
                .expect("many values"),
            [5120, 3, 2, 1]
        );
        assert_eq!(
            config
                .integers_by_key("http.lowSpeedTime")
                .transpose()?
                .expect("many values"),
            [10, 3, 2, 1]
        );
        assert_eq!(
            config
                .strings_by_key("http.proxyAuthMethod")
                .expect("at least one value"),
            [cow_bstr("basic")],
            "this value isn't overridden directly"
        );
        assert_eq!(
            config
                .strings_by_key("gitoxide.https.proxy")
                .expect("at least one value"),
            [
                cow_bstr("https-upper"),
                cow_bstr(if cfg!(windows) {
                    "https-upper" // on windows, environment variables are case-insensitive
                } else {
                    "https-lower-override"
                })
            ]
        );
        assert_eq!(
            config
                .strings_by_key("gitoxide.http.proxy")
                .expect("at least one value"),
            [cow_bstr("http-lower")]
        );
        assert_eq!(
            config
                .strings_by_key("gitoxide.http.allProxy")
                .expect("at least one value"),
            [
                cow_bstr("all-proxy"), // on windows, environment variables are case-insensitive
                cow_bstr(if cfg!(windows) { "all-proxy" } else { "all-proxy-lower" })
            ]
        );
        assert_eq!(
            config
                .strings_by_key("gitoxide.http.noProxy")
                .expect("at least one value"),
            [
                cow_bstr("no-proxy"), // on windows, environment variables are case-insensitive
                cow_bstr(if cfg!(windows) { "no-proxy" } else { "no-proxy-lower" })
            ]
        );
        assert_eq!(
            config.strings_by_key("http.sslCAInfo").expect("at least one value"),
            [
                cow_bstr("./CA.pem"),
                cow_bstr("./cli.pem"),
                cow_bstr("./api.pem"),
                cow_bstr("./env.pem")
            ]
        );
        assert_eq!(
            config.strings_by_key("http.sslVersion").expect("at least one value"),
            [
                cow_bstr("sslv2"),
                cow_bstr("sslv3"),
                cow_bstr("tlsv1"),
                cow_bstr("tlsv1.3")
            ]
        );
        assert_eq!(
            config.strings_by_key("ssh.variant").expect("at least one value"),
            [
                cow_bstr("ssh-variant-cli"),
                cow_bstr("ssh-variant-api"),
                cow_bstr("ssh-variant-env"),
            ]
        );
        assert_eq!(
            config.strings_by_key("core.sshCommand").expect("at least one value"),
            [
                cow_bstr("ssh-command-cli"),
                cow_bstr("ssh-command-api"),
                cow_bstr("ssh-command-env"),
            ]
        );
        assert_eq!(
            config
                .strings_by_key("gitoxide.ssh.commandWithoutShellFallback")
                .expect("at least one value"),
            [
                cow_bstr("ssh-command-fallback-cli"),
                cow_bstr("ssh-command-fallback-api"),
                cow_bstr("ssh-command-fallback-env"),
            ]
        );
        assert_eq!(
            config
                .strings_by_key("gitoxide.http.proxyAuthMethod")
                .expect("at least one value"),
            [
                cow_bstr("proxy-auth-method-cli"),
                cow_bstr("proxy-auth-method-api"),
                cow_bstr("proxy-auth-method-env"),
            ]
        );
        for (key, expected) in [
            ("gitoxide.http.verbose", "true"),
            ("gitoxide.allow.protocolFromUser", "file-allowed"),
            ("gitoxide.objects.noReplace", "no-replace"),
            ("gitoxide.objects.replaceRefBase", "refs/replace-mine"),
            ("gitoxide.committer.nameFallback", "committer name"),
            ("gitoxide.committer.emailFallback", "committer email"),
            ("gitoxide.author.nameFallback", "author name"),
            ("gitoxide.author.emailFallback", "author email"),
            ("gitoxide.commit.authorDate", default_date),
            ("gitoxide.commit.committerDate", default_date),
            ("gitoxide.user.emailFallback", "user email"),
            ("core.deltaBaseCacheLimit", "0"),
            ("gitoxide.objects.cacheLimit", "5m"),
        ] {
            assert_eq!(
                config
                    .string_by_key(key)
                    .unwrap_or_else(|| panic!("no value for {key}"))
                    .as_ref(),
                expected,
                "{} == {}",
                key,
                expected
            );
        }
        Ok(())
    }

    fn cow_bstr(s: &str) -> Cow<BStr> {
        Cow::Borrowed(s.into())
    }
}

mod worktree {
    use gix::open;

    #[test]
    fn with_worktree_configs() -> gix_testtools::Result {
        let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
        let fixture_dir = gix_testtools::scripted_fixture_read_only("make_worktree_repo_with_configs.sh")?;
        let worktree_base = manifest_dir.join(&fixture_dir).join("repo/.git/worktrees");

        {
            let base = open(fixture_dir.join("repo"))?;
            let base_config = base.config_snapshot();

            assert_eq!(
                base.work_dir(),
                Some(fixture_dir.join("repo").as_path()),
                "the main worktree"
            );
            assert_eq!(base.git_dir(), fixture_dir.join("repo/.git"), "git dir and…");
            assert_eq!(
                base.common_dir(),
                fixture_dir.join("repo/.git"),
                "…common dir are the same"
            );

            assert_eq!(
                base_config.string("worktree.setting").expect("exists").as_ref(),
                "set in the main worktree"
            );
            assert_eq!(
                base_config.string("shared.setting").expect("exists").as_ref(),
                "set in the shared config"
            );
            assert_eq!(
                base_config.string("override.setting").expect("exists").as_ref(),
                "set in the shared config"
            );
        }

        {
            let wt1 = open(fixture_dir.join("wt-1"))?;
            let wt1_config = wt1.config_snapshot();
            assert_eq!(
                wt1.work_dir(),
                Some(fixture_dir.join("wt-1").as_path()),
                "a linked worktree in its own location"
            );
            assert_eq!(
                wt1.git_dir(),
                worktree_base.join("wt-1"),
                "whose git-dir is within the common dir"
            );
            assert_eq!(
                wt1.common_dir(),
                worktree_base.join("wt-1/../.."),
                "the common dir is the `git-dir` of the repository with the main worktree"
            );

            assert_eq!(
                wt1_config.string("worktree.setting").expect("exists").as_ref(),
                "set in wt-1"
            );
            assert_eq!(
                wt1_config.string("shared.setting").expect("exists").as_ref(),
                "set in the shared config"
            );
            assert_eq!(
                wt1_config.string("override.setting").expect("exists").as_ref(),
                "set in the shared config"
            );
        }

        {
            let wt2 = open(fixture_dir.join("wt-2"))?;
            let wt2_config = wt2.config_snapshot();
            assert_eq!(
                wt2.work_dir(),
                Some(fixture_dir.join("wt-2").as_path()),
                "another linked worktree as sibling to wt-1"
            );
            assert_eq!(wt2.git_dir(), worktree_base.join("wt-2"));
            assert_eq!(wt2.common_dir(), worktree_base.join("wt-2/../.."));

            assert_eq!(
                wt2_config.string("worktree.setting").expect("exists").as_ref(),
                "set in wt-2"
            );
            assert_eq!(
                wt2_config.string("shared.setting").expect("exists").as_ref(),
                "set in the shared config"
            );
            assert_eq!(
                wt2_config.string("override.setting").expect("exists").as_ref(),
                "override in wt-2"
            );
        }

        Ok(())
    }
}
