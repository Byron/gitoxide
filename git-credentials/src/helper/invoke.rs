use crate::helper::NextAction;

/// The outcome of the credentials [`helper`][crate::helper()].
pub struct Outcome {
    /// The obtained identity.
    pub identity: git_sec::identity::Account,
    /// A handle to the action to perform next in another call to [`helper()`][crate::helper()].
    pub next: NextAction,
}

/// The Result type used in [`helper()`][crate::helper()].
pub type Result = std::result::Result<Option<Outcome>, Error>;

/// The error used in the [credentials helper][crate::helper::invoke()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Context(#[from] crate::helper::context::decode::Error),
    #[error("An IO error occurred while communicating to the credentials helper")]
    Io(#[from] std::io::Error),
    #[error("Could not find {name:?} in output of credentials helper")]
    KeyNotFound { name: &'static str },
    #[error(transparent)]
    CredentialsHelperFailed { source: std::io::Error },
}

pub(crate) mod function {
    use crate::helper::{invoke::Error, invoke::Outcome, invoke::Result, Action, Context, NextAction};
    use std::io::Read;

    impl Action {
        /// Send ourselves to the given `write` which is expected to be credentials-helper compatible
        pub fn send(&self, mut write: impl std::io::Write) -> std::io::Result<()> {
            match self {
                Action::Fill(ctx) => ctx.write_to(write),
                Action::Approve(last) | Action::Reject(last) => {
                    write.write_all(last)?;
                    write.write_all(&[b'\n'])
                }
            }
        }
    }

    /// Invoke the given `helper` with `action` in `context`.
    ///
    /// Usually the first call is performed with [`Action::Fill`] to obtain an identity, which subsequently can be used.
    /// On successful usage, use [`NextAction::approve()`], otherwise [`NextAction::reject()`].
    pub fn invoke(mut helper: impl crate::Helper, action: Action) -> Result {
        let (stdin, stdout) = helper.start(&action)?;
        action.send(stdin)?;
        let stdout = stdout
            .map(|mut stdout| {
                let mut buf = Vec::new();
                stdout.read_to_end(&mut buf).map(|_| buf)
            })
            .transpose()?;
        helper.finish().map_err(|err| {
            if err.kind() == std::io::ErrorKind::Other {
                Error::CredentialsHelperFailed { source: err }
            } else {
                err.into()
            }
        })?;

        match stdout {
            None => Ok(None),
            Some(stdout) => {
                let ctx = Context::from_bytes(stdout.as_slice())?;
                let username = ctx.username.ok_or_else(|| Error::KeyNotFound { name: "username" })?;
                let password = ctx.password.ok_or_else(|| Error::KeyNotFound { name: "password" })?;
                Ok(Some(Outcome {
                    identity: git_sec::identity::Account { username, password },
                    next: NextAction {
                        previous_output: stdout.into(),
                    },
                }))
            }
        }
    }
}
