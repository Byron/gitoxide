use std::{convert::TryFrom, ffi::OsString};

use bstr::BString;

/// The action passed to the credential helper implementation in [`main()`][crate::program::main()].
#[derive(Debug, Copy, Clone)]
pub enum Action {
    /// Get credentials for a url.
    Get,
    /// Store credentials provided in the given context.
    Store,
    /// Erase credentials identified by the given context.
    Erase,
}

impl TryFrom<OsString> for Action {
    type Error = Error;

    fn try_from(value: OsString) -> Result<Self, Self::Error> {
        Ok(match value.to_str() {
            Some("fill" | "get") => Action::Get,
            Some("approve" | "store") => Action::Store,
            Some("reject" | "erase") => Action::Erase,
            _ => return Err(Error::ActionInvalid { name: value }),
        })
    }
}

impl Action {
    /// Return ourselves as string representation, similar to what would be passed as argument to a credential helper.
    pub fn as_str(&self) -> &'static str {
        match self {
            Action::Get => "get",
            Action::Store => "store",
            Action::Erase => "erase",
        }
    }
}

/// The error of [`main()`][crate::program::main()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Action named {name:?} is invalid, need 'get', 'store', 'erase' or 'fill', 'approve', 'reject'")]
    ActionInvalid { name: OsString },
    #[error("The first argument must be the action to perform")]
    ActionMissing,
    #[error(transparent)]
    Helper {
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Context(#[from] crate::protocol::context::decode::Error),
    #[error("Credentials for {url:?} could not be obtained")]
    CredentialsMissing { url: BString },
    #[error("The url wasn't provided in input - the git credentials protocol mandates this")]
    UrlMissing,
}

pub(crate) mod function {
    use std::{convert::TryInto, ffi::OsString};

    use crate::{
        program::main::{Action, Error},
        protocol::Context,
    };

    /// Invoke a custom credentials helper which receives program `args`, with the first argument being the
    /// action to perform (as opposed to the program name).
    /// Then read context information from `stdin` and if the action is `Action::Get`, then write the result to `stdout`.
    /// `credentials` is the API version of such call, where`Ok(Some(context))` returns credentials, and `Ok(None)` indicates
    /// no credentials could be found for `url`, which is always set when called.
    ///
    /// Call this function from a programs `main`, passing `std::env::args_os()`, `stdin()` and `stdout` accordingly, along with
    /// your own helper implementation.
    pub fn main<CredentialsFn, E>(
        args: impl IntoIterator<Item = OsString>,
        mut stdin: impl std::io::Read,
        stdout: impl std::io::Write,
        credentials: CredentialsFn,
    ) -> Result<(), Error>
    where
        CredentialsFn: FnOnce(Action, Context) -> Result<Option<Context>, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        let action: Action = args.into_iter().next().ok_or(Error::ActionMissing)?.try_into()?;
        let mut buf = Vec::<u8>::with_capacity(512);
        stdin.read_to_end(&mut buf)?;
        let ctx = Context::from_bytes(&buf)?;
        if ctx.url.is_none() {
            return Err(Error::UrlMissing);
        }
        let res = credentials(action, ctx).map_err(|err| Error::Helper { source: Box::new(err) })?;
        match (action, res) {
            (Action::Get, None) => {
                return Err(Error::CredentialsMissing {
                    url: Context::from_bytes(&buf)?.url.expect("present and checked above"),
                })
            }
            (Action::Get, Some(ctx)) => ctx.write_to(stdout)?,
            (Action::Erase | Action::Store, None) => {}
            (Action::Erase | Action::Store, Some(_)) => {
                panic!("BUG: credentials helper must not return context for erase or store actions")
            }
        }
        Ok(())
    }
}
