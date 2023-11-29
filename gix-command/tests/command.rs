use gix_testtools::Result;

mod context {
    use gix_command::Context;

    fn winfix(expected: impl Into<String>) -> String {
        // Unclear why it's not debug-printing the env on windows.
        if cfg!(windows) {
            "\"\"".into()
        } else {
            expected.into()
        }
    }

    #[test]
    fn git_dir_sets_git_dir_env_and_cwd() {
        let ctx = Context {
            git_dir: Some(".".into()),
            ..Default::default()
        };
        let cmd = std::process::Command::from(gix_command::prepare("").with_context(ctx));
        assert_eq!(format!("{cmd:?}"), winfix(r#"GIT_DIR="." """#));
    }

    #[test]
    fn worktree_dir_sets_env_only() {
        let ctx = Context {
            worktree_dir: Some(".".into()),
            ..Default::default()
        };
        let cmd = std::process::Command::from(gix_command::prepare("").with_context(ctx));
        assert_eq!(format!("{cmd:?}"), winfix(r#"GIT_WORK_TREE="." """#));
    }

    #[test]
    fn no_replace_objects_sets_env_only() {
        for value in [false, true] {
            let expected = usize::from(value);
            let ctx = Context {
                no_replace_objects: Some(value),
                ..Default::default()
            };
            let cmd = std::process::Command::from(gix_command::prepare("").with_context(ctx));
            assert_eq!(
                format!("{cmd:?}"),
                winfix(format!(r#"GIT_NO_REPLACE_OBJECTS="{expected}" """#))
            );
        }
    }

    #[test]
    fn ref_namespace_sets_env_only() {
        let ctx = Context {
            ref_namespace: Some("namespace".into()),
            ..Default::default()
        };
        let cmd = std::process::Command::from(gix_command::prepare("").with_context(ctx));
        assert_eq!(format!("{cmd:?}"), winfix(r#"GIT_NAMESPACE="namespace" """#));
    }

    #[test]
    fn literal_pathspecs_sets_env_only() {
        for value in [false, true] {
            let expected = usize::from(value);
            let ctx = Context {
                literal_pathspecs: Some(value),
                ..Default::default()
            };
            let cmd = std::process::Command::from(gix_command::prepare("").with_context(ctx));
            assert_eq!(
                format!("{cmd:?}"),
                winfix(format!(r#"GIT_LITERAL_PATHSPECS="{expected}" """#))
            );
        }
    }

    #[test]
    fn glob_pathspecs_sets_env_only() {
        for (value, expected) in [
            (false, "GIT_NOGLOB_PATHSPECS=\"1\""),
            (true, "GIT_GLOB_PATHSPECS=\"1\""),
        ] {
            let ctx = Context {
                glob_pathspecs: Some(value),
                ..Default::default()
            };
            let cmd = std::process::Command::from(gix_command::prepare("").with_context(ctx));
            assert_eq!(format!("{cmd:?}"), winfix(format!(r#"{expected} """#)));
        }
    }

    #[test]
    fn icase_pathspecs_sets_env_only() {
        for value in [false, true] {
            let expected = usize::from(value);
            let ctx = Context {
                icase_pathspecs: Some(value),
                ..Default::default()
            };
            let cmd = std::process::Command::from(gix_command::prepare("").with_context(ctx));
            assert_eq!(
                format!("{cmd:?}"),
                winfix(format!(r#"GIT_ICASE_PATHSPECS="{expected}" """#))
            );
        }
    }
}

mod prepare {
    #[cfg(windows)]
    const SH: &str = "sh";
    #[cfg(not(windows))]
    const SH: &str = "/bin/sh";

    fn quoted(input: &[&str]) -> String {
        input.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(" ")
    }

    #[test]
    fn empty() {
        let cmd = std::process::Command::from(gix_command::prepare(""));
        assert_eq!(format!("{cmd:?}"), "\"\"");
    }

    #[test]
    fn single_and_multiple_arguments() {
        let cmd = std::process::Command::from(gix_command::prepare("ls").arg("first").args(["second", "third"]));
        assert_eq!(format!("{cmd:?}"), quoted(&["ls", "first", "second", "third"]));
    }

    #[test]
    fn multiple_arguments_in_one_line_with_auto_split() {
        let cmd = std::process::Command::from(
            gix_command::prepare("echo first second third").with_shell_allow_argument_splitting(),
        );
        assert_eq!(
            format!("{cmd:?}"),
            quoted(&["echo", "first", "second", "third"]),
            "we split by hand which works unless one tries to rely on shell-builtins (which we can't detect)"
        );
    }

    #[test]
    fn single_and_multiple_arguments_as_part_of_command() {
        let cmd = std::process::Command::from(gix_command::prepare("ls first second third"));
        assert_eq!(
            format!("{cmd:?}"),
            quoted(&["ls first second third"]),
            "without shell, this is an invalid command"
        );
    }

    #[test]
    fn single_and_multiple_arguments_as_part_of_command_with_shell() {
        let cmd = std::process::Command::from(gix_command::prepare("ls first second third").with_shell());
        assert_eq!(
            format!("{cmd:?}"),
            if cfg!(windows) {
                quoted(&["ls", "first", "second", "third"])
            } else {
                quoted(&[SH, "-c", "ls first second third", "--"])
            },
            "with shell, this works as it performs word splitting"
        );
    }

    #[test]
    fn single_and_complex_arguments_as_part_of_command_with_shell() {
        let cmd = std::process::Command::from(gix_command::prepare("ls --foo \"a b\"").arg("additional").with_shell());
        assert_eq!(
            format!("{cmd:?}"),
            if cfg!(windows) {
                quoted(&["ls", "--foo", "a b", "additional"])
            } else {
                format!(r#""{SH}" "-c" "ls --foo \"a b\" \"$@\"" "--" "additional""#)
            },
            "with shell, this works as it performs word splitting"
        );
    }

    #[test]
    fn single_and_complex_arguments_with_auto_split() {
        let cmd =
            std::process::Command::from(gix_command::prepare("ls --foo=\"a b\"").with_shell_allow_argument_splitting());
        assert_eq!(
            format!("{cmd:?}"),
            format!(r#""ls" "--foo=a b""#),
            "splitting can also handle quotes"
        );
    }

    #[test]
    fn single_and_complex_arguments_will_not_auto_split_on_special_characters() {
        let cmd =
            std::process::Command::from(gix_command::prepare("ls --foo=~/path").with_shell_allow_argument_splitting());
        assert_eq!(
            format!("{cmd:?}"),
            format!(r#""{SH}" "-c" "ls --foo=~/path" "--""#),
            "splitting can also handle quotes"
        );
    }

    #[test]
    fn tilde_path_and_multiple_arguments_as_part_of_command_with_shell() {
        let cmd = std::process::Command::from(gix_command::prepare("~/bin/exe --foo \"a b\"").with_shell());
        assert_eq!(
            format!("{cmd:?}"),
            format!(r#""{SH}" "-c" "~/bin/exe --foo \"a b\"" "--""#),
            "this always needs a shell as we need tilde expansion"
        );
    }
}

mod spawn {
    #[cfg(unix)]
    use bstr::ByteSlice;

    #[test]
    #[cfg(unix)]
    fn environment_variables_are_passed_one_by_one() -> crate::Result {
        let out = gix_command::prepare("echo $FIRST $SECOND")
            .env("FIRST", "first")
            .env("SECOND", "second")
            .with_shell()
            .spawn()?
            .wait_with_output()?;
        assert_eq!(out.stdout.as_bstr(), "first second\n");
        Ok(())
    }

    #[test]
    #[cfg(unix)]
    fn disallow_shell() -> crate::Result {
        let out = gix_command::prepare("PATH= echo hi")
            .with_shell()
            .spawn()?
            .wait_with_output()?;
        assert_eq!(out.stdout.as_bstr(), "hi\n");

        let mut cmd: std::process::Command = gix_command::prepare("echo hi").with_shell().without_shell().into();
        assert!(
            cmd.env_remove("PATH").spawn().is_err(),
            "no command named 'echo hi' exists"
        );
        Ok(())
    }

    #[test]
    fn direct_command_execution_searches_in_path() -> crate::Result {
        assert!(gix_command::prepare(if cfg!(unix) { "ls" } else { "dir.exe" })
            .spawn()?
            .wait()?
            .success());
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn direct_command_with_absolute_command_path() -> crate::Result {
        assert!(gix_command::prepare("/bin/ls").spawn()?.wait()?.success());
        Ok(())
    }

    mod with_shell {
        use gix_testtools::bstr::ByteSlice;

        #[test]
        fn command_in_path_with_args() -> crate::Result {
            assert!(gix_command::prepare(if cfg!(unix) { "ls -l" } else { "dir.exe -a" })
                .with_shell()
                .spawn()?
                .wait()?
                .success());
            Ok(())
        }

        #[test]
        fn sh_shell_specific_script_code() -> crate::Result {
            assert!(gix_command::prepare(":;:;:").with_shell().spawn()?.wait()?.success());
            Ok(())
        }

        #[test]
        fn sh_shell_specific_script_code_with_single_extra_arg() -> crate::Result {
            let out = gix_command::prepare("printf")
                .with_shell()
                .arg("1")
                .spawn()?
                .wait_with_output()?;
            assert!(out.status.success());
            assert_eq!(out.stdout.as_bstr(), "1");
            Ok(())
        }

        #[test]
        fn sh_shell_specific_script_code_with_multiple_extra_args() -> crate::Result {
            let out = gix_command::prepare("printf")
                .with_shell()
                .arg("%s")
                .arg("arg")
                .spawn()?
                .wait_with_output()?;
            assert!(out.status.success());
            assert_eq!(out.stdout.as_bstr(), "arg");
            Ok(())
        }
    }
}
