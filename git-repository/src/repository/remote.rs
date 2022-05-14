use std::borrow::Cow;
use std::convert::TryInto;

use crate::bstr::BStr;
use git_ref::FullName;
use git_validate::reference::name::Error as ValidateNameError;

impl crate::Repository {
    /// Returns a reference to the remote associated with the given branch name.
    /// Returns `None` if the remote refernce was not found.
    /// May return an error if the reference is invalid.
    // TODO: Use `Cow<FullNameRef>` instead of `FullName`
    pub fn remote_ref(&self, branch: &str) -> Option<Result<FullName, ValidateNameError>> {
        self.config
            .resolved
            .string("branch", Some(branch), "merge")
            .map(|v| v.into_owned().try_into())
    }

    /// Returns the name of the remote associated with the given branch name.
    /// In some cases, the returned name will be an URL.
    /// Returns `None` if the remote was not found.
    pub fn branch_remote_name<'a>(&'a self, branch: &str) -> Option<Cow<'a, BStr>> {
        self.config.resolved.string("branch", Some(branch), "remote")
    }
}
