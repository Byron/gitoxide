mod spawn {
    #[test]
    #[ignore]
    fn direct_command_execution_searches_in_path() {
        git_command::prepare(if cfg!(unix) { "ls" } else { "dir.exe" })
            .spawn()
            .unwrap();
    }

    mod with_shell {
        #[test]
        #[ignore]
        fn command_in_path_with_args() {
            git_command::prepare(if cfg!(unix) { "ls -l" } else { "dir.exe -a" })
                .with_shell()
                .spawn()
                .unwrap();
        }
    }
}
