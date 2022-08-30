use std::convert::TryInto;

/// Run like this `echo url=https://example.com | cargo run --example git-credential-light -- fill`
pub fn main() -> Result<(), git_credentials::program::main::Error> {
    git_credentials::program::main(
        std::env::args_os().skip(1),
        std::io::stdin(),
        std::io::stdout(),
        |action, context| {
            use git_credentials::program::main::Action::*;
            git_credentials::program::Cascade::default()
                .invoke(
                    match action {
                        Get => git_credentials::helper::Action::Get(context),
                        Erase | Store => todo!(),
                    },
                    git_prompt::Options::default().apply_environment(true, true, true),
                )
                .map(|outcome| outcome.and_then(|outcome| (&outcome.next).try_into().ok()))
        },
    )
}
