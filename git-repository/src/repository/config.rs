use crate::bstr::ByteSlice;
use crate::config;
use std::collections::BTreeSet;

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
    use crate::remote;
    use std::borrow::Cow;
    use std::collections::BTreeSet;

    impl crate::Repository {
        /// Returns a sorted list unique of symbolic names of remotes that
        /// we deem [trustworthy][crate::open::Options::filter_config_section()].
        pub fn remote_names(&self) -> BTreeSet<&str> {
            self.subsection_names_of("remote")
        }

        /// Obtain the branch-independent name for a remote for use in the given `direction`, or `None` if it could not be determined.
        ///
        /// For _fetching_, use the only configured remote, or default to `origin` if it exists.
        /// For _pushing_, use the `remote.pushDefault` trusted configuration key, or fall back to the rules for _fetching_.
        ///
        /// # Notes
        ///
        /// It's up to the caller to determine what to do if the current `head` is unborn or detached.
        pub fn remote_default_name(&self, direction: remote::Direction) -> Option<Cow<'_, str>> {
            let name = (direction == remote::Direction::Push)
                .then(|| {
                    self.config
                        .resolved
                        .string_filter("remote", None, "pushDefault", &mut self.filter_config_section())
                        .and_then(|s| match s {
                            Cow::Borrowed(s) => s.to_str().ok().map(Cow::Borrowed),
                            Cow::Owned(s) => s.to_str().ok().map(|s| Cow::Owned(s.into())),
                        })
                })
                .flatten();
            name.or_else(|| {
                let names = self.remote_names();
                match names.len() {
                    0 => None,
                    1 => names.iter().next().copied().map(Cow::Borrowed),
                    _more_than_one => names.get("origin").copied().map(Cow::Borrowed),
                }
            })
        }
    }
}

mod branch {
    use std::collections::BTreeSet;
    use std::{borrow::Cow, convert::TryInto};

    use git_ref::FullNameRef;
    use git_validate::reference::name::Error as ValidateNameError;

    use crate::bstr::BStr;

    impl crate::Repository {
        /// Return a set of unique short branch names for which custom configuration exists in the configuration,
        /// if we deem them [trustworthy][crate::open::Options::filter_config_section()].
        pub fn branch_names(&self) -> BTreeSet<&str> {
            self.subsection_names_of("branch")
        }

        /// Returns a reference to the remote associated with the given `short_branch_name`,
        /// always `main` instead of `refs/heads/main`.
        ///
        /// The remote-ref is the one we track on the remote side for merging and pushing.
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

    fn subsection_names_of<'a>(&'a self, header_name: &'a str) -> BTreeSet<&'a str> {
        self.config
            .resolved
            .sections_by_name(header_name)
            .map(|it| {
                let filter = self.filter_config_section();
                it.filter(move |s| filter(s.meta()))
                    .filter_map(|section| section.header().subsection_name().and_then(|b| b.to_str().ok()))
                    .collect()
            })
            .unwrap_or_default()
    }
}
