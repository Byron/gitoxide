mod submodules {
    use std::path::Path;

    use git_repository as git;

    #[test]
    fn by_their_worktree_checkout_and_git_modules_dir() {
        let dir = git_testtools::scripted_fixture_repo_read_only("make_submodules.sh").unwrap();
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
                // assert_eq!(repo.kind(), git::Kind::Submodule);
                assert_eq!(repo.work_dir().expect("non-bare"), dir.join(&submodule_m1_workdir));
                assert_eq!(repo.git_dir(), dir.join(&submodule_m1_gitdir));

                let repo = git::open_opts(repo.work_dir().expect("non-bare"), git::open::Options::isolated()).unwrap();
                assert_eq!(repo.kind(), git::Kind::Submodule);
                assert_eq!(repo.work_dir().expect("non-bare"), dir.join(&submodule_m1_workdir));
                assert_eq!(repo.git_dir(), dir.join(&submodule_m1_gitdir));
            }
        }
    }

    fn discover_repo(name: impl AsRef<Path>) -> crate::Result<git::Repository> {
        let dir = git_testtools::scripted_fixture_repo_read_only("make_submodules.sh")?;
        let repo_dir = dir.join(name);
        Ok(git::ThreadSafeRepository::discover_opts(
            repo_dir,
            Default::default(),
            git_sec::trust::Mapping {
                full: crate::restricted(),
                reduced: crate::restricted(),
            },
        )?
        .to_thread_local())
    }
}

mod with_overrides {
    use crate::util::named_subrepo_opts;
    use git_object::bstr::BStr;
    use git_repository as git;
    use git_sec::Permission;
    use git_testtools::Env;
    use serial_test::serial;
    use std::borrow::Cow;

    #[test]
    #[serial]
    fn order_from_api_and_cli_and_environment() -> crate::Result {
        let default_date = "1979-02-26 18:30:00";
        let _env = Env::new()
            .set("GIT_HTTP_USER_AGENT", "agent-from-env")
            .set("GIT_HTTP_LOW_SPEED_LIMIT", "1")
            .set("GIT_HTTP_LOW_SPEED_TIME", "1")
            .set("GIT_HTTP_PROXY_AUTHMETHOD", "negotiate")
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
            .set("EMAIL", "user email");
        let mut opts = git::open::Options::isolated()
            .config_overrides([
                "http.userAgent=agent-from-api",
                "http.lowSpeedLimit=2",
                "http.lowSpeedTime=2",
            ])
            .cli_overrides([
                "http.userAgent=agent-from-cli",
                "http.lowSpeedLimit=3",
                "http.lowSpeedTime=3",
            ]);
        opts.permissions.env.git_prefix = Permission::Allow;
        opts.permissions.env.http_transport = Permission::Allow;
        opts.permissions.env.identity = Permission::Allow;
        let repo = named_subrepo_opts("make_config_repos.sh", "http-config", opts)?;
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
            [cow_bstr("basic"), cow_bstr("negotiate"),]
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
