use git_testtools::Result;

mod spawn {
    #[test]
    fn direct_command_execution_searches_in_path() -> crate::Result {
        assert!(git_command::prepare(if cfg!(unix) { "ls" } else { "dir.exe" })
            .spawn()?
            .wait()?
            .success());
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn direct_command_with_absolute_command_path() -> crate::Result {
        assert!(git_command::prepare("/bin/ls").spawn()?.wait()?.success());
        Ok(())
    }

    mod with_shell {
        use git_testtools::bstr::ByteSlice;

        #[test]
        fn command_in_path_with_args() -> crate::Result {
            assert!(git_command::prepare(if cfg!(unix) { "ls -l" } else { "dir.exe -a" })
                .with_shell()
                .spawn()?
                .wait()?
                .success());
            Ok(())
        }

        #[test]
        fn sh_shell_specific_script_code() -> crate::Result {
            assert!(git_command::prepare(":;:;:").with_shell().spawn()?.wait()?.success());
            Ok(())
        }

        #[test]
        fn sh_shell_specific_script_code_with_single_extra_arg() -> crate::Result {
            let out = git_command::prepare("echo")
                .with_shell()
                .arg("1")
                .spawn()?
                .wait_with_output()?;
            assert!(out.status.success());
            #[cfg(not(windows))]
            assert_eq!(out.stdout.as_bstr(), "1\n");
            #[cfg(windows)]
            assert_eq!(out.stdout.as_bstr(), "1\r\n");
            Ok(())
        }

        #[test]
        fn sh_shell_specific_script_code_with_multiple_extra_args() -> crate::Result {
            let out = git_command::prepare("echo")
                .with_shell()
                .arg("1")
                .arg("2")
                .spawn()?
                .wait_with_output()?;
            assert!(out.status.success());
            #[cfg(not(windows))]
            assert_eq!(out.stdout.as_bstr(), "1 2\n");
            #[cfg(windows)]
            assert_eq!(out.stdout.as_bstr(), "1 2\r\n");
            Ok(())
        }
    }
}
