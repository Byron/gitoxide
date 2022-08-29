mod ask {
    use std::process::Command;

    #[test]
    #[ignore]
    fn username_password() {
        let mut cmd = Command::new(env!("CARGO"));
        cmd.args(["run", "--example", "credentials"]);
        let mut p = expectrl::Session::spawn(cmd).unwrap();
        p.expect("Username: ").unwrap();
    }
}
