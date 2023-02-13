use std::convert::TryInto;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    UrlParse(#[from] gix::url::parse::Error),
    #[error(transparent)]
    Configuration(#[from] gix::config::credential_helpers::Error),
    #[error(transparent)]
    Protocol(#[from] gix::credentials::protocol::Error),
}

pub fn function(repo: gix::Repository, action: gix::credentials::program::main::Action) -> anyhow::Result<()> {
    use gix::credentials::program::main::Action::*;
    gix::credentials::program::main(
        Some(action.as_str().into()),
        std::io::stdin(),
        std::io::stdout(),
        |action, context| -> Result<_, Error> {
            let (mut cascade, _action, prompt_options) = repo.config_snapshot().credential_helpers(gix::url::parse(
                context.url.as_ref().expect("framework assures URL is present").as_ref(),
            )?)?;
            cascade
                .invoke(
                    match action {
                        Get => gix::credentials::helper::Action::Get(context),
                        Erase => gix::credentials::helper::Action::Erase(context.to_bstring()),
                        Store => gix::credentials::helper::Action::Store(context.to_bstring()),
                    },
                    prompt_options,
                )
                .map(|outcome| outcome.and_then(|outcome| (&outcome.next).try_into().ok()))
                .map_err(Into::into)
        },
    )
    .map_err(Into::into)
}
