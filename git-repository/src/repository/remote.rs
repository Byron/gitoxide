use std::borrow::Cow;
use std::convert::TryInto;

use crate::bstr::BStr;
use git_ref::FullNameRef;
use git_validate::reference::name::Error as ValidateNameError;

impl crate::Repository {
    /// Returns a reference to the remote associated with the given branch name.
    /// Returns `None` if the remote reference was not found.
    /// May return an error if the reference is invalid.
    pub fn remote_ref(&self, branch: &str) -> Option<Result<Cow<'_, FullNameRef>, ValidateNameError>> {
        self.config
            .resolved
            .string("branch", Some(branch), "merge")
            .map(|v| match v {
                Cow::Borrowed(v) => v.try_into().map(Cow::Borrowed),
                Cow::Owned(v) => v.try_into().map(Cow::Owned),
            })
    }

    /// Returns the name of the remote associated with the given branch name.
    /// In some cases, the returned name will be an URL.
    /// Returns `None` if the remote was not found.
    pub fn branch_remote_name(&self, branch: &str) -> Option<Cow<'_, BStr>> {
        self.config.resolved.string("branch", Some(branch), "remote")
    }
}
