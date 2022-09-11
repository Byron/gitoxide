use crate::Permission;
use std::fmt::{Debug, Display, Formatter};

/// An error to use if an operation cannot proceed due to insufficient permissions.
///
/// It's up to the implementation to decide which permission is required for an operation, and which one
/// causes errors.
#[derive(Debug)]
pub struct Error<R: std::fmt::Debug> {
    /// The resource which cannot be used.
    pub resource: R,
}

impl<R> Display for Error<R>
where
    R: std::fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Not allowed to handle resource {:?}: permission denied",
            self.resource
        )
    }
}

impl<R> std::error::Error for Error<R> where R: std::fmt::Debug {}

impl Permission {
    /// Check this permissions and produce a reply to indicate if the `resource` can be used and in which way.
    ///
    /// Only if this permission is set to `Allow` will the resource be usable.
    pub fn check<R: std::fmt::Debug>(&self, resource: R) -> Result<Option<R>, Error<R>> {
        match self {
            Permission::Allow => Ok(Some(resource)),
            Permission::Deny => Ok(None),
            Permission::Forbid => Err(Error { resource }),
        }
    }

    /// Like [`check()`][Self::check()], but degenerates the type to an option to make it more useful in cases where
    /// `Forbid` shoudn't abort the entire operation.
    pub fn check_opt<R: std::fmt::Debug>(&self, resource: R) -> Option<R> {
        match self {
            Permission::Allow => Some(resource),
            Permission::Deny | Permission::Forbid => None,
        }
    }
}

impl Display for Permission {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(
            match self {
                Permission::Allow => "allowed",
                Permission::Deny => "denied",
                Permission::Forbid => "forbidden",
            },
            f,
        )
    }
}
