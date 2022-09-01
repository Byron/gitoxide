mod options;

mod ask {
    #[test]
    #[cfg(unix)]
    fn askpass_only() {
        let mut cmd = std::process::Command::new(env!("CARGO"));
        cmd.args(["build", "--example", "use-askpass", "--example", "askpass"]);
        cmd.spawn().unwrap().wait().expect("example builds OK");

        let mut p = expectrl::spawn("../target/debug/examples/use-askpass").unwrap();
        p.expect("Password: ").unwrap();
        p.send_line(" password with space ").unwrap();
        p.expect("\" password with space \"").unwrap();
        p.expect(expectrl::Eof).unwrap();
        p.wait().unwrap();
    }

    #[test]
    #[cfg(unix)]
    fn username_password() {
        let mut cmd = std::process::Command::new(env!("CARGO"));
        cmd.args(["build", "--example", "credentials"]);
        cmd.spawn().unwrap().wait().expect("example builds OK");

        let mut p = expectrl::spawn("../target/debug/examples/credentials").unwrap();
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
