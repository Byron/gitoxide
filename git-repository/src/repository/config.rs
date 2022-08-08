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

mod remote {
    use crate::bstr::ByteSlice;
    use std::collections::BTreeSet;

    impl crate::Repository {
        /// Returns a sorted list unique of symbolic names of remotes that
        /// we deem [trustworthy][crate::open::Options::filter_config_section()].
        pub fn remote_names(&self) -> BTreeSet<&str> {
            self.config
                .resolved
                .sections_by_name("remote")
                .map(|it| {
                    let filter = self.filter_config_section();
                    let set: BTreeSet<_> = it
                        .filter(move |s| filter(s.meta()))
                        .filter_map(|section| section.header().subsection_name().and_then(|b| b.to_str().ok()))
                        .collect();
                    set.into()
                })
                .unwrap_or_default()
        }
    }
}

mod branch {
    use std::{borrow::Cow, convert::TryInto};

    use git_ref::FullNameRef;
    use git_validate::reference::name::Error as ValidateNameError;

    use crate::bstr::BStr;

    impl crate::Repository {
        // /// Return a iterator of short branch names for which information custom configuration exists.
        // pub fn branch_names(&self) -> impl Iterator<Item = &BStr> + '_ {
        //
        // }

        /// Returns a reference to the remote associated with the given `short_branch_name`, typically `main` instead of `refs/heads/main`.
        /// Returns `None` if the remote reference was not found.
        /// May return an error if the reference is invalid.
        pub fn branch_remote_ref(
            &self,
            short_branch_name: &str,
        ) -> Option<Result<Cow<'_, FullNameRef>, ValidateNameError>> {
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

impl crate::Repository {
    pub(crate) fn filter_config_section(&self) -> fn(&git_config::file::Metadata) -> bool {
        self.options
            .filter_config_section
            .unwrap_or(config::section::is_trusted)
    }
}
