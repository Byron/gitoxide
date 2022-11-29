use std::path::Path;

use git_repository as git;
use git_sec::Permission;
use git_testtools::Env;
use serial_test::serial;

use crate::named_repo;

#[test]
#[serial]
fn author_and_committer_and_fallback() {
    for trust in [git_sec::Trust::Full, git_sec::Trust::Reduced] {
        let repo = named_repo("make_config_repo.sh").unwrap();
        let work_dir = repo.work_dir().expect("present").canonicalize().unwrap();
        let _env = Env::new()
            .set(
                "GIT_CONFIG_SYSTEM",
                work_dir.join("system.config").display().to_string(),
            )
            .set("GIT_AUTHOR_NAME", "author")
            .set("GIT_AUTHOR_EMAIL", "author@email")
            .set("GIT_AUTHOR_DATE", "1979-02-26 18:30:00")
            .set("GIT_COMMITTER_NAME", "commiter-overrider-unused")
            .set("GIT_COMMITTER_EMAIL", "committer-override-unused@email")
            .set("GIT_COMMITTER_DATE", "1980-02-26 18:30:00")
            .set("EMAIL", "general@email-unused")
            .set("GIT_CONFIG_COUNT", "1")
            .set("GIT_CONFIG_KEY_0", "include.path")
            .set("GIT_CONFIG_VALUE_0", work_dir.join("c.config").display().to_string());
        let repo = git::open_opts(
            repo.git_dir(),
            repo.open_options().clone().with(trust).permissions(git::Permissions {
                env: git::permissions::Environment {
                    xdg_config_home: Permission::Deny,
                    home: Permission::Deny,
                    ..git::permissions::Environment::all()
                },
                ..Default::default()
            }),
        )
        .unwrap();

        assert_eq!(
            repo.author(),
            Some(git_actor::SignatureRef {
                name: "author".into(),
                email: "author@email".into(),
                time: git_date::Time {
                    seconds_since_unix_epoch: 42,
                    offset_in_seconds: 1800,
                    sign: git_date::time::Sign::Plus
                }
            }),
            "the only parsesable marker time we know right now, indicating time parse success"
        );
        {
            let actual = repo.committer().expect("set");
            assert_eq!(actual.name, "committer");
            assert_eq!(actual.email, "committer@email");
        }
        {
            let actual = repo.user_default();
            assert_eq!(actual.name, "gitoxide");
            assert_eq!(actual.email, "gitoxide@localhost");
        }

        let config = repo.config_snapshot();

        assert_eq!(config.boolean("core.bare"), Some(false));
        assert_eq!(config.boolean("a.bad-bool"), None);
        assert_eq!(config.try_boolean("core.bare"), Some(Ok(false)));
        assert!(matches!(config.try_boolean("a.bad-bool"), Some(Err(_))));

        assert_eq!(config.integer("a.int"), Some(42));
        assert_eq!(config.integer("a.int-overflowing"), None);
        assert_eq!(config.integer("a.int-overflowing"), None);
        assert!(config.try_integer("a.int-overflowing").expect("present").is_err());

        assert_eq!(
            config.string("a.single-string").expect("present").as_ref(),
            "hello world"
        );

        assert_eq!(
            config.string("a.local-override").expect("present").as_ref(),
            "from-a.config"
        );
        assert_eq!(
            config.string("a.system").expect("present").as_ref(),
            "from-system.config"
        );
        assert_eq!(
            config.string("a.system-override").expect("present").as_ref(),
            "from-b.config"
        );

        assert_eq!(
            config.string("a.env-override").expect("present").as_ref(),
            "from-c.config"
        );

        assert_eq!(config.boolean("core.missing"), None);
        assert_eq!(config.try_boolean("core.missing"), None);

        let relative_path_key = "a.relative-path";
        if trust == git_sec::Trust::Full {
            assert_eq!(
                config
                    .trusted_path(relative_path_key)
                    .expect("exists")
                    .expect("no error"),
                Path::new("./something")
            );
            assert_eq!(
                config
                    .trusted_path("a.absolute-path")
                    .expect("exists")
                    .expect("no error"),
                Path::new("/etc/man.conf")
            );
            assert!(config.trusted_path("a.bad-user-path").expect("exists").is_err());
        } else {
            assert!(
                config.trusted_path(relative_path_key).is_none(),
                "trusted paths need full trust"
            );
        }
    }
}
