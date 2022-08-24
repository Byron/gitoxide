use bstr::{BStr, BString};

mod error {
    /// The error used in the [credentials helper][crate::helper()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("An IO error occurred while communicating to the credentials helper")]
        Io(#[from] std::io::Error),
        #[error("Could not find {name:?} in output of credentials helper")]
        KeyNotFound { name: String },
        #[error("Credentials helper program failed with status code {code:?}")]
        CredentialsHelperFailed { code: Option<i32> },
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
    fn is_fill(&self) -> bool {
        matches!(self, Action::Fill(_))
    }
    fn as_str(&self) -> &str {
        match self {
            Action::Approve(_) => "approve",
            Action::Fill(_) => "fill",
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
    use crate::helper::{message, Action, Error, NextAction, Outcome, Result};
    use std::io::Write;
    use std::process::{Command, Stdio};

    // TODO(sec): reimplement helper execution so it won't use the `git credential` anymore to allow enforcing our own security model.
    //            Currently we support more flexible configuration than downright not working at all.
    /// Call the `git` credentials helper program performing the given `action`.
    ///
    /// Usually the first call is performed with [`Action::Fill`] to obtain an identity, which subsequently can be used.
    /// On successful usage, use [`NextAction::approve()`], otherwise [`NextAction::reject()`].
    pub fn helper(action: Action<'_>) -> Result {
        let mut cmd = Command::new(cfg!(windows).then(|| "git.exe").unwrap_or("git"));
        cmd.arg("credential")
            .arg(action.as_str())
            .stdin(Stdio::piped())
            .stdout(if action.is_fill() {
                Stdio::piped()
            } else {
                Stdio::null()
            });
        let mut child = cmd.spawn()?;
        let mut stdin = child.stdin.take().expect("stdin to be configured");

        match action {
            Action::Fill(url) => message::encode(url, stdin)?,
            Action::Approve(last) | Action::Reject(last) => {
                stdin.write_all(&last)?;
                stdin.write_all(&[b'\n'])?
            }
        }

        let output = child.wait_with_output()?;
        if !output.status.success() {
            return Err(Error::CredentialsHelperFailed {
                code: output.status.code(),
            });
        }
        let stdout = output.stdout;
        if stdout.is_empty() {
            Ok(None)
        } else {
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
