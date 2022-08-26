use crate::protocol::Context;
use bstr::{BStr, BString};

/// The outcome of the credentials helper [invocation][crate::helper::invoke()].
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Outcome {
    /// The username to use in the identity, if set.
    pub username: Option<String>,
    /// The username to use in the identity, if set.
    pub password: Option<String>,
    /// If set, the helper asked to stop the entire process, whether the identity is complete or not.
    pub quit: bool,
    /// A handle to the action to perform next in another call to [`helper::invoke()`][crate::helper::invoke()].
    pub next: NextAction,
}

impl Outcome {
    /// Try to fetch username _and_ password to form an identity. This will fail if one of them is not set.
    ///
    /// This does nothing if only one of the fields is set, or consume both.
    pub fn consume_identity(&mut self) -> Option<git_sec::identity::Account> {
        if self.username.is_none() || self.password.is_none() {
            return None;
        }
        self.username
            .take()
            .zip(self.password.take())
            .map(|(username, password)| git_sec::identity::Account { username, password })
    }
}

/// The Result type used in [`invoke()`][crate::helper::invoke()].
pub type Result = std::result::Result<Option<Outcome>, Error>;

/// The error used in the [credentials helper invocation][crate::helper::invoke()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    ContextDecode(#[from] protocol::context::decode::Error),
    #[error("An IO error occurred while communicating to the credentials helper")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    CredentialsHelperFailed { source: std::io::Error },
}

/// The action to perform by the credentials [helper][`crate::helper::invoke()`].
#[derive(Clone, Debug)]
pub enum Action {
    /// Provide credentials using the given repository context, which must include the repository url.
    Get(Context),
    /// Approve the credentials as identified by the previous input provided as `BString`, containing information from [`Context`].
    Store(BString),
    /// Reject the credentials as identified by the previous input provided as `BString`. containing information from [`Context`].
    Erase(BString),
}

/// Initialization
impl Action {
    /// Create a `Get` action with context containing the given URL
    pub fn get_for_url(url: impl Into<BString>) -> Action {
        Action::Get(Context {
            url: Some(url.into()),
            ..Default::default()
        })
    }
}

/// Access
impl Action {
    /// Return the payload of store or erase actions.
    pub fn payload(&self) -> Option<&BStr> {
        use bstr::ByteSlice;
        match self {
            Action::Get(_) => None,
            Action::Store(p) | Action::Erase(p) => Some(p.as_bstr()),
        }
    }
    /// Return the context of a get operation, or `None`.
    ///
    /// The opposite of [`payload`][Action::payload()].
    pub fn context(&self) -> Option<&Context> {
        match self {
            Action::Get(ctx) => Some(ctx),
            Action::Erase(_) | Action::Store(_) => None,
        }
    }

    /// Returns true if this action expects output from the helper.
    pub fn expects_output(&self) -> bool {
        matches!(self, Action::Get(_))
    }

    /// The name of the argument to describe this action. If `is_external` is true, the target program is
    /// a custom credentials helper, not a built-in one.
    pub fn as_arg(&self, is_external: bool) -> &str {
        match self {
            Action::Get(_) if is_external => "get",
            Action::Get(_) => "fill",
            Action::Store(_) if is_external => "store",
            Action::Store(_) => "approve",
            Action::Erase(_) if is_external => "erase",
            Action::Erase(_) => "reject",
        }
    }
}

/// A handle to [store][NextAction::store()] or [erase][NextAction::erase()] the outcome of the initial action.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NextAction {
    previous_output: BString,
}

impl From<Context> for NextAction {
    fn from(ctx: Context) -> Self {
        let mut buf = Vec::<u8>::new();
        ctx.write_to(&mut buf).expect("cannot fail");
        NextAction {
            previous_output: buf.into(),
        }
    }
}

impl NextAction {
    /// Approve the result of the previous [Action] and store for lookup.
    pub fn store(self) -> Action {
        Action::Store(self.previous_output)
    }
    /// Reject the result of the previous [Action] and erase it as to not be returned when being looked up.
    pub fn erase(self) -> Action {
        Action::Erase(self.previous_output)
    }
}
pub(crate) mod function {
    use crate::helper::{Action, NextAction};
    use crate::helper::{Context, Error, Outcome, Result};
    use std::io::Read;

    impl Action {
        /// Send ourselves to the given `write` which is expected to be credentials-helper compatible
        pub fn send(&self, mut write: impl std::io::Write) -> std::io::Result<()> {
            match self {
                Action::Get(ctx) => ctx.write_to(write),
                Action::Store(last) | Action::Erase(last) => {
                    write.write_all(last)?;
                    write.write_all(&[b'\n'])
                }
            }
        }
    }

    /// Invoke the given `helper` with `action` in `context`.
    ///
    /// Usually the first call is performed with [`Action::Get`] to obtain `Some` identity, which subsequently can be used if it is complete.
    /// Note that it may also only contain the username or password.
    /// On successful usage, use [`NextAction::store()`], otherwise [`NextAction::erase()`], which returns `Ok(None)` as no outcome
    /// is expected.
    pub fn invoke(mut helper: impl crate::Helper, action: &Action) -> Result {
        let (stdin, stdout) = helper.start(action)?;
        if let (Action::Get(_), None) = (&action, &stdout) {
            panic!("BUG: `Helper` impls must return an output handle to read output from if Action::Get is provided")
        }
        action.send(stdin)?;
        let stdout = stdout
            .map(|mut stdout| {
                let mut buf = Vec::new();
                stdout.read_to_end(&mut buf).map(|_| buf)
            })
            .transpose()
            .map_err(|err| Error::CredentialsHelperFailed { source: err })?;
        helper.finish().map_err(|err| {
            if err.kind() == std::io::ErrorKind::Other {
                Error::CredentialsHelperFailed { source: err }
            } else {
                err.into()
            }
        })?;

        match matches!(action, Action::Get(_)).then(|| stdout).flatten() {
            None => Ok(None),
            Some(stdout) => {
                let ctx = Context::from_bytes(stdout.as_slice())?;
                Ok(Some(Outcome {
                    username: ctx.username,
                    password: ctx.password,
                    quit: ctx.quit.unwrap_or(false),
                    next: NextAction {
                        previous_output: stdout.into(),
                    },
                }))
            }
        }
    }
}
use crate::protocol;
pub use function::invoke;
