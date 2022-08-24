use git_credentials::helper;

/// Run like this `echo url=https://example.com | cargo run --example custom-helper -- get`
pub fn main() -> Result<(), git_credentials::helper::main::Error> {
    git_credentials::helper::main(
        std::env::args_os().skip(1),
        std::io::stdin(),
        std::io::stdout(),
        |action, credentials| -> std::io::Result<_> {
            match action {
                helper::main::Action::Get => Ok(Some(helper::Context {
                    username: Some("user".into()),
                    password: Some("pass".into()),
                    ..credentials
                })),
                helper::main::Action::Erase => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Refusing to delete credentials for demo purposes",
                )),
                helper::main::Action::Store => Ok(None),
            }
        },
    )
}
