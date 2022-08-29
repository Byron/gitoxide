mod ask {
    use std::process::Command;

    #[test]
    #[cfg(unix)]
    fn username_password() {
        let mut cmd = Command::new(env!("CARGO"));
        cmd.args(["run", "--example", "credentials"]);
        let mut p = expectrl::Session::spawn(cmd).unwrap();
        p.expect("Username: ").unwrap();
        p.send_line(" user with space ").unwrap();
        p.expect("\" user with space\"").unwrap();
        p.expect("Password: ").unwrap();
        p.send_line(" password with space ").unwrap();
        p.expect("\" password with space \"").unwrap();
        p.expect(expectrl::Eof).unwrap();
        p.wait().unwrap();
    }

    #[test]
    #[cfg(not(unix))]
    #[ignore]
    fn username_password_not_available() {}
}
