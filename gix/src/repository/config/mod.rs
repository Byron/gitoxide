use std::collections::BTreeSet;

use crate::{bstr::ByteSlice, config};

/// General Configuration
impl crate::Repository {
    /// Return a snapshot of the configuration as seen upon opening the repository.
    pub fn config_snapshot(&self) -> config::Snapshot<'_> {
        config::Snapshot { repo: self }
    }

    /// Return a mutable snapshot of the configuration as seen upon opening the repository, starting a transaction.
    /// When the returned instance is dropped, it is applied in full, even if the reason for the drop is an error.
    ///
    /// Note that changes to the configuration are in-memory only and are observed only the this instance
    /// of the [`Repository`][crate::Repository].
    pub fn config_snapshot_mut(&mut self) -> config::SnapshotMut<'_> {
        let config = self.config.resolved.as_ref().clone();
        config::SnapshotMut {
            repo: Some(self),
            config,
        }
    }

    /// Return filesystem options as retrieved from the repository configuration.
    ///
    /// Note that these values have not been [probed](gix_fs::Capabilities::probe()).
    pub fn filesystem_options(&self) -> Result<gix_fs::Capabilities, config::boolean::Error> {
        self.config.fs_capabilities()
    }

    /// Return filesystem options on how to perform stat-checks, typically in relation to the index.
    ///
    /// Note that these values have not been [probed](gix_fs::Capabilities::probe()).
    #[cfg(feature = "index")]
    pub fn stat_options(&self) -> Result<gix_index::entry::stat::Options, config::stat_options::Error> {
        self.config.stat_options()
    }

    /// The options used to open the repository.
    pub fn open_options(&self) -> &crate::open::Options {
        &self.options
    }

    /// Obtain options for use when connecting via `ssh`.
    #[cfg(feature = "blocking-network-client")]
    pub fn ssh_connect_options(
        &self,
    ) -> Result<gix_protocol::transport::client::ssh::connect::Options, config::ssh_connect_options::Error> {
        use crate::config::{
            cache::util::ApplyLeniency,
            tree::{gitoxide, Core, Ssh},
        };

        let config = &self.config.resolved;
        let mut trusted = self.filter_config_section();
        let mut fallback_active = false;
        let ssh_command = config
            .string_filter("core", None, Core::SSH_COMMAND.name, &mut trusted)
            .or_else(|| {
                fallback_active = true;
                config.string_filter(
                    "gitoxide",
                    Some("ssh".into()),
                    gitoxide::Ssh::COMMAND_WITHOUT_SHELL_FALLBACK.name,
                    &mut trusted,
                )
            })
            .map(|cmd| gix_path::from_bstr(cmd).into_owned().into());
        let opts = gix_protocol::transport::client::ssh::connect::Options {
            disallow_shell: fallback_active,
            command: ssh_command,
            kind: config
                .string_filter_by_key("ssh.variant", &mut trusted)
                .and_then(|variant| Ssh::VARIANT.try_into_variant(variant).transpose())
                .transpose()
                .with_leniency(self.options.lenient_config)?,
        };
        Ok(opts)
    }

    /// The kind of object hash the repository is configured to use.
    pub fn object_hash(&self) -> gix_hash::Kind {
        self.config.object_hash
    }
}

#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
mod transport;

mod remote {
    use std::{borrow::Cow, collections::BTreeSet};

    use crate::{bstr::ByteSlice, remote};

    impl crate::Repository {
        /// Returns a sorted list unique of symbolic names of remotes that
        /// we deem [trustworthy][crate::open::Options::filter_config_section()].
        // TODO: Use `remote::Name` here
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
        // TODO: use remote::Name here
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
    use std::{borrow::Cow, collections::BTreeSet, convert::TryInto};

    use gix_ref::FullNameRef;
    use gix_validate::reference::name::Error as ValidateNameError;

    use crate::bstr::BStr;

    impl crate::Repository {
        /// Return a set of unique short branch names for which custom configuration exists in the configuration,
        /// if we deem them [trustworthy][crate::open::Options::filter_config_section()].
        pub fn branch_names(&self) -> BTreeSet<&str> {
            self.subsection_names_of("branch")
        }

        /// Returns the validated reference on the remote associated with the given `short_branch_name`,
        /// always `main` instead of `refs/heads/main`.
        ///
        /// The returned reference is the one we track on the remote side for merging and pushing.
        /// Returns `None` if the remote reference was not found.
        /// May return an error if the reference is invalid.
        pub fn branch_remote_ref<'a>(
            &self,
            short_branch_name: impl Into<&'a BStr>,
        ) -> Option<Result<Cow<'_, FullNameRef>, ValidateNameError>> {
            self.config
                .resolved
                .string("branch", Some(short_branch_name.into()), "merge")
                .map(crate::config::tree::branch::Merge::try_into_fullrefname)
        }

        /// Returns the unvalidated name of the remote associated with the given `short_branch_name`,
        /// typically `main` instead of `refs/heads/main`.
        /// In some cases, the returned name will be an URL.
        /// Returns `None` if the remote was not found or if the name contained illformed UTF-8.
        ///
        /// See also [`Reference::remote_name()`][crate::Reference::remote_name()] for a more typesafe version
        /// to be used when a `Reference` is available.
        pub fn branch_remote_name<'a>(
            &self,
            short_branch_name: impl Into<&'a BStr>,
        ) -> Option<crate::remote::Name<'_>> {
            self.config
                .resolved
                .string("branch", Some(short_branch_name.into()), "remote")
                .and_then(|name| name.try_into().ok())
        }
    }
}

impl crate::Repository {
    pub(crate) fn filter_config_section(&self) -> fn(&gix_config::file::Metadata) -> bool {
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
