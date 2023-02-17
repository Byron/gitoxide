mod apply_environment {
    use std::{borrow::Cow, path::Path};

    use gix_prompt::{Mode, Options};
    use gix_testtools::Env;
    use serial_test::serial;

    #[test]
    #[serial]
    fn git_askpass_overrides_everything_and_ssh_askpass_does_not() {
        let _env = Env::new()
            .set("GIT_ASKPASS", "override")
            .set("SSH_ASKPASS", "does not matter");

        assert_eq!(
            Options {
                askpass: Some(Cow::Borrowed(Path::new("current"))),
                ..Default::default()
            }
            .apply_environment(true, true, false)
            .askpass
            .expect("set")
            .as_ref(),
            Path::new("override")
        );
    }

    #[test]
    #[serial]
    fn git_askpass_is_used_first_and_sets_unset_askpass_values() {
        let _env = Env::new()
            .set("GIT_ASKPASS", "from-env")
            .set("SSH_ASKPASS", "does not matter");

        assert_eq!(
            Options::default()
                .apply_environment(true, true, false)
                .askpass
                .expect("set")
                .as_ref(),
            Path::new("from-env")
        );
    }

    #[test]
    #[serial]
    fn ssh_askpass_is_used_as_fallback() {
        let _env = Env::new().set("SSH_ASKPASS", "fallback");

        assert_eq!(
            Options {
                mode: Mode::Visible,
                ..Default::default()
            }
            .apply_environment(true, true, false)
            .askpass
            .expect("set")
            .as_ref(),
            Path::new("fallback")
        );
    }

    #[test]
    #[serial]
    fn ssh_askpass_does_not_override_current_value() {
        let _env = Env::new().set("SSH_ASKPASS", "fallback");

        assert_eq!(
            Options {
                askpass: Some(Cow::Borrowed(Path::new("current"))),
                ..Default::default()
            }
            .apply_environment(true, true, false)
            .askpass
            .expect("set")
            .as_ref(),
            Path::new("current")
        );
    }

    #[test]
    #[serial]
    fn mode_is_left_untouched_if_terminal_prompt_is_trueish() {
        let _env = Env::new().set("GIT_TERMINAL_PROMPT", "true");

        assert_eq!(
            Options {
                mode: Mode::Hidden,
                ..Default::default()
            }
            .apply_environment(false, false, true)
            .mode,
            Mode::Hidden
        );
    }

    #[test]
    #[serial]
    fn mode_is_disabled_if_terminal_prompt_is_falseish() {
        let _env = Env::new().set("GIT_TERMINAL_PROMPT", "0");

        assert_eq!(
            Options {
                mode: Mode::Hidden,
                ..Default::default()
            }
            .apply_environment(false, false, true)
            .mode,
            Mode::Disable
        );
    }

    #[test]
    #[serial]
    fn mode_is_unchanged_if_git_terminal_prompt_is_not_set() {
        assert_eq!(
            Options {
                mode: Mode::Hidden,
                ..Default::default()
            }
            .apply_environment(false, false, true)
            .mode,
            Mode::Hidden
        );
    }
}
