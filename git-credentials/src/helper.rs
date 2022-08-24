use bstr::{BStr, BString};

/// The kind of helper program to use.
pub enum Kind {
    /// The built-in git-credential helper program, part of any git distribution.
    GitCredential,
}

/// Additional context to be passed to the credentials helper.
// TODO: fill in what's needed per configuration
#[derive(Debug, Default)]
pub struct Context;

mod error {
    /// The error used in the [credentials helper][crate::helper()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("An IO error occurred while communicating to the credentials helper")]
        Io(#[from] std::io::Error),
        #[error("Could not find {name:?} in output of credentials helper")]
        KeyNotFound { name: String },
        #[error(transparent)]
        CredentialsHelperFailed { source: std::io::Error },
    }
}
pub use error::Error;

/// The Result type used in [`helper()`][crate::helper()].
pub type Result = std::result::Result<Option<Outcome>, Error>;

/// The action to perform by the credentials [helper][`crate::helper()`].
#[derive(Clone, Debug)]
pub enum Action<'a> {
    /// Provide credentials using the given repository URL (as &str) as context.
    Fill(&'a BStr),
    /// Approve the credentials as identified by the previous input provided as `BString`.
    Approve(BString),
    /// Reject the credentials as identified by the previous input provided as `BString`.
    Reject(BString),
}

impl<'a> Action<'a> {
    /// Returns true if this action expects output from the helper.
    pub fn expects_output(&self) -> bool {
        matches!(self, Action::Fill(_))
    }
    /// The name of the argument to describe this action. If `is_custom` is true, the target program is
    /// a custom credentials helper, not a built-in one.
    pub fn as_helper_arg(&self, is_custom: bool) -> &str {
        match self {
            Action::Fill(_) if is_custom => "get",
            Action::Fill(_) => "fill",
            Action::Approve(_) if is_custom => "store",
            Action::Approve(_) => "approve",
            Action::Reject(_) if is_custom => "erase",
            Action::Reject(_) => "reject",
        }
    }
}

/// A handle to [approve][NextAction::approve()] or [reject][NextAction::reject()] the outcome of the initial action.
#[derive(Clone, Debug)]
pub struct NextAction {
    previous_output: BString,
}

impl NextAction {
    /// Approve the result of the previous [Action].
    pub fn approve(self) -> Action<'static> {
        Action::Approve(self.previous_output)
    }
    /// Reject the result of the previous [Action].
    pub fn reject(self) -> Action<'static> {
        Action::Reject(self.previous_output)
    }
}

/// The outcome of the credentials [`helper`][crate::helper()].
pub struct Outcome {
    /// The obtained identity.
    pub identity: git_sec::identity::Account,
    /// A handle to the action to perform next in another call to [`helper()`][crate::helper()].
    pub next: NextAction,
}

pub(crate) mod function {
    use crate::helper::{message, Action, Context, Error, Kind, NextAction, Outcome, Result};
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

    /// Call the `git` credentials helper program performing the given `action`, without any context from git configuration.
    ///
    /// See [`invoke()`] for a more flexible implementation.
    pub fn helper(action: Action<'_>) -> Result {
        invoke(
            crate::Program::from_kind(Kind::GitCredential),
            action,
            Context::default(),
        )
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
                        .map(|(_, n)| n.to_owned())
                };
                Ok(Some(Outcome {
                    identity: git_sec::identity::Account {
                        username: find("username")?,
                        password: find("password")?,
                    },
                    next: NextAction {
                        previous_output: stdout.into(),
                    },
                }))
            }
        }
    }
}
pub use function::invoke;

///
pub mod message {
    use bstr::BStr;

    /// Encode `url` to `out` for consumption by a `git credentials` helper program.
    pub fn encode(url: &BStr, mut out: impl std::io::Write) -> std::io::Result<()> {
        validate(url)?;
        writeln!(out, "url={}\n", url)
    }

    fn validate(url: &BStr) -> std::io::Result<()> {
        if url.contains(&0) || url.contains(&b'\n') {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "token to encode must not contain newlines or null bytes",
            ));
        }
        Ok(())
    }

    /// Decode all lines in `input` as key-value pairs produced by a `git credentials` helper program.
    pub fn decode(mut input: impl std::io::Read) -> std::io::Result<Vec<(String, String)>> {
        let mut buf = String::new();
        input.read_to_string(&mut buf)?;
        buf.lines()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let mut iter = l.splitn(2, '=').map(|s| s.to_owned());
                match (iter.next(), iter.next()) {
                    (Some(key), Some(value)) => validate(key.as_str().into())
                        .and_then(|_| validate(value.as_str().into()))
                        .map(|_| (key, value)),
                    _ => Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Invalid format, expecting key=value",
                    )),
                }
            })
            .collect::<std::io::Result<Vec<_>>>()
    }
}
