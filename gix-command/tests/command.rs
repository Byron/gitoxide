use gix_testtools::Result;

mod prepare {
    fn quoted(input: &[&str]) -> String {
        input.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(" ")
    }
    #[test]
    fn single_and_multiple_arguments() {
        let cmd = std::process::Command::from(gix_command::prepare("ls").arg("first").args(["second", "third"]));
        assert_eq!(format!("{cmd:?}"), quoted(&["ls", "first", "second", "third"]));
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
