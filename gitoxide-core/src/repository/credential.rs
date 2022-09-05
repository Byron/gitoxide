use std::convert::TryInto;

use git_repository as git;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    UrlParse(#[from] git::url::parse::Error),
    #[error(transparent)]
    Configuration(#[from] git::config::credential_helpers::Error),
    #[error(transparent)]
    Protocol(#[from] git::credentials::protocol::Error),
}

pub fn function(repo: git::Repository, action: git::credentials::program::main::Action) -> anyhow::Result<()> {
    use git::credentials::program::main::Action::*;
    git::credentials::program::main(
        Some(action.as_str().into()),
        std::io::stdin(),
        std::io::stdout(),
        |action, context| -> Result<_, Error> {
            let (mut cascade, _action, prompt_options) = repo.config_snapshot().credential_helpers(git::url::parse(
                context.url.as_ref().expect("framework assures URL is present").as_ref(),
            )?)?;
            cascade
                .invoke(
                    match action {
                        Get => git::credentials::helper::Action::Get(context),
                        Erase => git::credentials::helper::Action::Erase(context.to_bstring()),
                        Store => git::credentials::helper::Action::Store(context.to_bstring()),
                    },
                    prompt_options,
                )
                .map(|outcome| outcome.and_then(|outcome| (&outcome.next).try_into().ok()))
                .map_err(Into::into)
        },
    )
    .map_err(Into::into)
}
