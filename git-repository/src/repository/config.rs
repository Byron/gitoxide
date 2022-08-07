use crate::config;

/// General Configuration
impl crate::Repository {
    /// Return
    /// Return a snapshot of the configuration as seen upon opening the repository.
    pub fn config_snapshot(&self) -> config::Snapshot<'_> {
        config::Snapshot { repo: self }
    }

    /// The options used to open the repository.
    pub fn open_options(&self) -> &crate::open::Options {
        &self.options
    }

    /// The kind of object hash the repository is configured to use.
    pub fn object_hash(&self) -> git_hash::Kind {
        self.config.object_hash
    }
}

mod branch {
    use std::{borrow::Cow, convert::TryInto};

    use git_ref::FullNameRef;
    use git_validate::reference::name::Error as ValidateNameError;

    use crate::bstr::BStr;

    impl crate::Repository {
        /// Returns a reference to the remote associated with the given `short_branch_name`, typically `main` instead of `refs/heads/main`.
        /// Returns `None` if the remote reference was not found.
        /// May return an error if the reference is invalid.
        pub fn remote_ref(&self, short_branch_name: &str) -> Option<Result<Cow<'_, FullNameRef>, ValidateNameError>> {
            self.config
                .resolved
                .string("branch", Some(short_branch_name), "merge")
                .map(|v| match v {
                    Cow::Borrowed(v) => v.try_into().map(Cow::Borrowed),
                    Cow::Owned(v) => v.try_into().map(Cow::Owned),
                })
        }

        /// Returns the name of the remote associated with the given `short_branch_name`, typically `main` instead of `refs/heads/main`.
        /// In some cases, the returned name will be an URL.
        /// Returns `None` if the remote was not found.
        pub fn branch_remote_name(&self, short_branch_name: &str) -> Option<Cow<'_, BStr>> {
            self.config.resolved.string("branch", Some(short_branch_name), "remote")
        }
    }
}
