mod ask {
    use std::process::Command;

    #[test]
    fn username_password() {
        let mut cmd = Command::new(env!("CARGO"));
        cmd.args(["run", "--example", "credentials"]);
        let mut p = expectrl::Session::spawn(cmd).unwrap();
        p.expect("Username: ").unwrap();
        p.send_line("user").unwrap();
    }
}
