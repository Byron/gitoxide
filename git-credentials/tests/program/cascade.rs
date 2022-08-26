mod invoke {
    use bstr::{ByteSlice, ByteVec};
    use git_credentials::helper::Action;
    use git_credentials::program::Cascade;
    use git_credentials::protocol::Context;
    use git_credentials::{program, protocol, Program};
    use git_sec::identity::Account;
    use git_testtools::fixture_path;
    use std::convert::TryInto;

    #[test]
    fn credentials_are_filled_in_one_by_one_and_stop_when_complete() {
        let actual = invoke_cascade(
            ["username", "password", "custom-helper"],
            Action::get_for_url("does/not/matter"),
        )
        .unwrap()
        .expect("credentials");
        assert_eq!(actual.identity, identity("user", "pass"));
    }

    #[test]
    #[cfg(not(target_os = "linux"))]
    fn failing_helpers_for_filling_dont_interrupt() {
        let actual = invoke_cascade(["fail", "custom-helper"], action_get())
            .unwrap()
            .expect("credentials");
        assert_eq!(actual.identity, identity("user-script", "pass-script"));
    }

    #[test]
    #[cfg(not(target_os = "linux"))]
    fn urls_are_split_in_get_to_support_scripts() {
        let actual = invoke_cascade(
            ["reflect", "custom-helper"],
            Action::get_for_url("https://example.com:8080/path/git/"),
        )
        .unwrap()
        .expect("credentials");

        let ctx: Context = (&actual.next).try_into().unwrap();
        assert_eq!(ctx.protocol.as_deref().expect("protocol"), "https");
        assert_eq!(ctx.host.as_deref().expect("host"), "example.com:8080");
        assert_eq!(ctx.path.as_deref().expect("path").as_bstr(), "path/git");
    }

    #[test]
    #[cfg(not(target_os = "linux"))]
    fn urls_are_split_in_get_but_can_skip_the_path_in_host_only_urls() {
        let actual = invoke_cascade(["reflect", "custom-helper"], Action::get_for_url("http://example.com"))
            .unwrap()
            .expect("credentials");

        let ctx: Context = (&actual.next).try_into().unwrap();
        assert_eq!(ctx.protocol.as_deref().expect("protocol"), "http");
        assert_eq!(ctx.host.as_deref().expect("host"), "example.com");
        assert_eq!(ctx.path, None);
    }

    #[test]
    #[cfg(not(target_os = "linux"))]
    fn helpers_can_set_any_context_value() {
        let actual = invoke_cascade(
            ["all-but-credentials", "custom-helper"],
            Action::get_for_url("http://github.com"),
        )
        .unwrap()
        .expect("credentials");

        let ctx: Context = (&actual.next).try_into().unwrap();
        assert_eq!(ctx.protocol.as_deref().expect("protocol"), "ftp");
        assert_eq!(ctx.host.as_deref().expect("host"), "example.com:8080");
        assert_eq!(
            ctx.path.expect("set by helper"),
            "/path/to/git/",
            "values are passed verbatim even if they would otherwise look different"
        );
    }

    #[test]
    #[cfg(not(target_os = "linux"))]
    fn helpers_can_set_any_context_value_using_the_url_only() {
        let actual = invoke_cascade(["url", "custom-helper"], Action::get_for_url("http://github.com"))
            .unwrap()
            .expect("credentials");

        let ctx: Context = (&actual.next).try_into().unwrap();
        assert_eq!(
            ctx.protocol.as_deref().expect("protocol"),
            "http",
            "url is processed last, it overwrites what came before"
        );
        assert_eq!(ctx.host.as_deref().expect("host"), "example.com:8080");
        assert_eq!(
            ctx.path.expect("set by helper"),
            "path/to/git",
            "the url is processed like any other"
        );
    }

    #[test]
    #[cfg(not(target_os = "linux"))]
    fn helpers_can_quit_and_their_creds_are_taken_if_complete() {
        let actual = invoke_cascade(["last-pass", "custom-helper"], Action::get_for_url("http://github.com"))
            .unwrap()
            .expect("credentials");

        assert_eq!(actual.identity, identity("user-last", "pass-last"));
    }

    fn action_get() -> Action {
        Action::get_for_url("does/not/matter")
    }

    fn identity(user: &str, pass: &str) -> Account {
        Account {
            username: user.into(),
            password: pass.into(),
        }
    }

    fn invoke_cascade<'a>(names: impl IntoIterator<Item = &'a str>, action: Action) -> protocol::Result {
        Cascade::default().extend(fixtures(names)).invoke(action)
    }

    fn fixtures<'a>(names: impl IntoIterator<Item = &'a str>) -> Vec<Program> {
        names
            .into_iter()
            .map(|name| git_path::realpath(fixture_path(format!("{}.sh", name))).unwrap())
            .map(|path| {
                let mut script = git_path::to_unix_separators_on_windows(git_path::into_bstr(path)).into_owned();
                script.insert_str(0, "sh ");
                Program::from_kind(program::Kind::ExternalShellScript(script))
            })
            .collect()
    }
}
