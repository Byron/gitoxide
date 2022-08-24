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
    #[error("Cannot handle usernames or passwords in illformed UTF-8 encoding")]
    IllformedUtf8InUsernameOrPassword,
    #[error("An IO error occurred while communicating to the credentials helper")]
    Io(#[from] std::io::Error),
    #[error("Could not find {name:?} in output of credentials helper")]
    KeyNotFound { name: String },
    #[error(transparent)]
    CredentialsHelperFailed { source: std::io::Error },
}

pub(crate) mod function {
    use crate::helper::{invoke::Error, invoke::Outcome, invoke::Result, message, Action, Context, NextAction};
    use bstr::ByteVec;
    use std::io::Read;

    impl Action<'_> {
        /// Send ourselves to the given `write` which is expected to be credentials-helper compatible
        pub fn send(&self, mut write: impl std::io::Write) -> std::io::Result<()> {
            match self {
                Action::Fill(url) => message::encode(url, write),
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
    pub fn invoke(mut helper: impl crate::Helper, action: Action<'_>, _context: Context) -> Result {
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
                let kvs = message::decode(stdout.as_slice())?;
                let find = |name: &str| {
                    kvs.iter()
                        .find(|(k, _)| k == name)
                        .ok_or_else(|| Error::KeyNotFound { name: name.into() })
                        .map(|(_, n)| n.to_vec())
                };
                Ok(Some(Outcome {
                    identity: git_sec::identity::Account {
                        username: find("username")?
                            .into_string()
                            .map_err(|_| Error::IllformedUtf8InUsernameOrPassword)?,
                        password: find("password")?
                            .into_string()
                            .map_err(|_| Error::IllformedUtf8InUsernameOrPassword)?,
                    },
                    next: NextAction {
                        previous_output: stdout.into(),
                    },
                }))
            }
        }
    }
}
