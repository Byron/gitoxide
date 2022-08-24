#![allow(missing_docs)]
use bstr::BString;
use std::convert::TryFrom;
use std::ffi::OsString;

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Get,
    Store,
    Erase,
}

impl TryFrom<OsString> for Action {
    type Error = Error;

    fn try_from(value: OsString) -> Result<Self, Self::Error> {
        Ok(match value.to_str() {
            Some("fill") | Some("get") => Action::Get,
            Some("approve") | Some("store") => Action::Store,
            Some("reject") | Some("erase") => Action::Erase,
            _ => return Err(Error::ActionInvalid { name: value }),
        })
    }
}

#[derive(Debug, thiserror::Error)]
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
    Context(#[from] crate::helper::context::decode::Error),
    #[error("Credentials for {url:?} could not be obtained")]
    CredentialsMissing { url: BString },
    #[error("The url wasn't provided in input - the git credentials protocol mandates this")]
    UrlMissing,
}

pub(crate) mod function {
    use crate::helper::main::{Action, Error};
    use crate::helper::Context;
    use std::convert::TryInto;
    use std::ffi::OsString;

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
