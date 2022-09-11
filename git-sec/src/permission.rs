use crate::Permission;
use std::fmt::{Display, Formatter};

/// An error to use if an operation cannot proceed due to insufficient permissions.
///
/// It's up to the implementation to decide which permission is required for an operation, and which one
/// causes errors.
#[derive(Debug, thiserror::Error)]
#[error("Not allowed to handle resource {:?}: permission {}", .resource, .permission)]
pub struct Error<R: std::fmt::Debug, P: std::fmt::Debug + Display> {
    /// The resource which cannot be used.
    pub resource: R,
    /// The permission causing it to be disallowed.
    pub permission: P,
}

impl Permission {
    /// Check this permissions and produce a reply to indicate if the `resource` can be used and in which way.
    ///
    /// Only if this permission is set to `Allow` will the resource be usable.
    pub fn check<R: std::fmt::Debug>(&self, resource: R) -> Result<Option<R>, Error<R, Self>> {
        match self {
            Permission::Allow => Ok(Some(resource)),
            Permission::Deny => Ok(None),
            Permission::Forbid => Err(Error {
                resource,
                permission: *self,
            }),
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
