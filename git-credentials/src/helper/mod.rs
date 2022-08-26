use bstr::BString;

/// The outcome of the credentials top-level functions to obtain a complete identity.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Outcome {
    /// The identity provide by the helper.
    pub identity: git_sec::identity::Account,
    /// A handle to the action to perform next in another call to [`helper::invoke()`][crate::helper::invoke()].
    pub next: invoke::NextAction,
}

/// The Result type used in credentials top-level functions to obtain a complete identity.
pub type Result = std::result::Result<Option<Outcome>, Error>;

/// The error returned top-level credential functions.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Invoke(#[from] invoke::Error),
    #[error("Could not obtain identity for context: {}", { let mut buf = Vec::<u8>::new(); context.write_to(&mut buf).ok(); String::from_utf8_lossy(&buf).into_owned() })]
    IdentityMissing { context: Context },
    #[error("The handler asked to stop trying to obtain credentials")]
    Quit,
}

/// Additional context to be passed to the credentials helper.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Context {
    /// The protocol over which the credential will be used (e.g., https).
    pub protocol: Option<String>,
    /// The remote hostname for a network credential. This includes the port number if one was specified (e.g., "example.com:8088").
    pub host: Option<String>,
    /// The path with which the credential will be used. E.g., for accessing a remote https repository, this will be the repository’s path on the server.
    /// It can also be a path on the file system.
    pub path: Option<BString>,
    /// The credential’s username, if we already have one (e.g., from a URL, the configuration, the user, or from a previously run helper).
    pub username: Option<String>,
    /// The credential’s password, if we are asking it to be stored.
    pub password: Option<String>,
    /// When this special attribute is read by git credential, the value is parsed as a URL and treated as if its constituent
    /// parts were read (e.g., url=<https://example.com> would behave as if
    /// protocol=https and host=example.com had been provided). This can help callers avoid parsing URLs themselves.
    pub url: Option<BString>,
    /// If true, the caller should stop asking for credentials immediately without calling more credential helpers in the chain.
    pub quit: Option<bool>,
}

/// Convert the outcome of a helper invocation to a helper result, assuring that the identity is complete in the process.
pub fn invoke_outcome_to_helper_result(outcome: Option<invoke::Outcome>, action: invoke::Action) -> Result {
    match (action, outcome) {
        (invoke::Action::Get(context), None) => Err(Error::IdentityMissing { context }),
        (invoke::Action::Get(context), Some(mut outcome)) => match outcome.consume_identity() {
            Some(identity) => Ok(Some(Outcome {
                identity,
                next: outcome.next,
            })),
            None => Err(if outcome.quit {
                Error::Quit
            } else {
                Error::IdentityMissing { context }
            }),
        },
        (invoke::Action::Store(_) | invoke::Action::Erase(_), _ignore) => Ok(None),
    }
}

/// A list of helper programs to run in order to obtain credentials.
#[allow(dead_code)]
pub struct Cascade {
    /// the programs to run in order.
    run: Vec<crate::Program>,
}

///
pub mod context;

///
pub mod invoke;
pub use invoke::function::invoke;

///
pub mod main;
pub use main::function::main;
