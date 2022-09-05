use git_repository as git;
use std::convert::TryInto;

pub fn function(_repo: git::Repository, action: git::credentials::program::main::Action) -> anyhow::Result<()> {
    // TODO: use repo for configuration
    use git::credentials::program::main::Action::*;
    git::credentials::program::main(
        Some(action.as_str().into()),
        std::io::stdin(),
        std::io::stdout(),
        |action, context| {
            git::credentials::helper::Cascade::default()
                .invoke(
                    match action {
                        Get => git::credentials::helper::Action::Get(context),
                        Erase => git::credentials::helper::Action::Erase(context.to_bstring()),
                        Store => git::credentials::helper::Action::Store(context.to_bstring()),
                    },
                    git::prompt::Options::default().apply_environment(true, true, true),
                )
                .map(|outcome| outcome.and_then(|outcome| (&outcome.next).try_into().ok()))
        },
    )
    .map_err(Into::into)
}
