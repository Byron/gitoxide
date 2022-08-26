use git_credentials::{program, protocol};

/// Run like this `echo url=https://example.com | cargo run --example custom-helper -- get`
pub fn main() -> Result<(), git_credentials::program::main::Error> {
    git_credentials::program::main(
        std::env::args_os().skip(1),
        std::io::stdin(),
        std::io::stdout(),
        |action, credentials| -> std::io::Result<_> {
            match action {
                program::main::Action::Get => Ok(Some(protocol::Context {
                    username: Some("user".into()),
                    password: Some("pass".into()),
                    ..credentials
                })),
                program::main::Action::Erase => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Refusing to delete credentials for demo purposes",
                )),
                program::main::Action::Store => Ok(None),
            }
        },
    )
}
