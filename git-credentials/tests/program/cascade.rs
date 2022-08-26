mod invoke {
    use bstr::ByteVec;
    use git_credentials::helper::Action;
    use git_credentials::program::Cascade;
    use git_credentials::{program, protocol, Program};
    use git_sec::identity::Account;
    use git_testtools::fixture_path;

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
