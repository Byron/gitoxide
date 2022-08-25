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
    }
}
