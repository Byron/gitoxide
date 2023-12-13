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

    /// Return the context to be passed to any spawned program that is supposed to interact with the repository, like
    /// hooks or filters.
    #[cfg(feature = "attributes")]
    pub fn command_context(&self) -> Result<gix_command::Context, config::command_context::Error> {
        use crate::config::{
            cache::util::ApplyLeniency,
            tree::{gitoxide, Key},
        };

        let pathspec_boolean = |key: &'static config::tree::keys::Boolean| {
            self.config
                .resolved
                .boolean("gitoxide", Some("pathspec".into()), key.name())
                .map(|value| key.enrich_error(value))
                .transpose()
                .with_leniency(self.config.lenient_config)
        };

        Ok(gix_command::Context {
            stderr: {
                let key = &gitoxide::Core::EXTERNAL_COMMAND_STDERR;
                self.config
                    .resolved
                    .boolean("gitoxide", Some("core".into()), key.name())
                    .map(|value| key.enrich_error(value))
                    .transpose()
                    .with_leniency(self.config.lenient_config)?
                    .unwrap_or(true)
                    .into()
            },
            git_dir: self.git_dir().to_owned().into(),
            worktree_dir: self.work_dir().map(ToOwned::to_owned),
            no_replace_objects: config::shared::is_replace_refs_enabled(
                &self.config.resolved,
                self.config.lenient_config,
                self.filter_config_section(),
            )?
            .map(|enabled| !enabled),
            ref_namespace: self.refs.namespace.as_ref().map(|ns| ns.as_bstr().to_owned()),
            literal_pathspecs: pathspec_boolean(&gitoxide::Pathspec::LITERAL)?,
            glob_pathspecs: pathspec_boolean(&gitoxide::Pathspec::GLOB)?
                .or(pathspec_boolean(&gitoxide::Pathspec::NOGLOB)?),
            icase_pathspecs: pathspec_boolean(&gitoxide::Pathspec::ICASE)?,
        })
    }

    /// The kind of object hash the repository is configured to use.
    pub fn object_hash(&self) -> gix_hash::Kind {
        self.config.object_hash
    }
}

#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
mod transport;

mod remote {
    use crate::bstr::BStr;
    use std::{borrow::Cow, collections::BTreeSet};

    use crate::config::tree::{Remote, Section};
    use crate::remote;

    /// Query configuration related to remotes.
    impl crate::Repository {
        /// Returns a sorted list unique of symbolic names of remotes that
        /// we deem [trustworthy][crate::open::Options::filter_config_section()].
        pub fn remote_names(&self) -> BTreeSet<Cow<'_, BStr>> {
            self.config
                .resolved
                .sections_by_name(Remote.name())
                .map(|it| {
                    let filter = self.filter_config_section();
                    it.filter(move |s| filter(s.meta()))
                        .filter_map(|section| section.header().subsection_name().map(Cow::Borrowed))
                        .collect()
                })
                .unwrap_or_default()
        }

        /// Obtain the branch-independent name for a remote for use in the given `direction`, or `None` if it could not be determined.
        ///
        /// For _fetching_, use the only configured remote, or default to `origin` if it exists.
        /// For _pushing_, use the `remote.pushDefault` trusted configuration key, or fall back to the rules for _fetching_.
        ///
        /// # Notes
        ///
        /// It's up to the caller to determine what to do if the current `head` is unborn or detached.
        pub fn remote_default_name(&self, direction: remote::Direction) -> Option<Cow<'_, BStr>> {
            let name = (direction == remote::Direction::Push)
                .then(|| {
                    self.config.resolved.string_filter(
                        Remote.name(),
                        None,
                        Remote::PUSH_DEFAULT.name,
                        &mut self.filter_config_section(),
                    )
                })
                .flatten();
            name.or_else(|| {
                let names = self.remote_names();
                match names.len() {
                    0 => None,
                    1 => names.into_iter().next(),
                    _more_than_one => {
                        let origin = Cow::Borrowed("origin".into());
                        names.contains(&origin).then_some(origin)
                    }
                }
            })
        }
    }
}

mod branch {
    use std::{borrow::Cow, collections::BTreeSet, convert::TryInto};

    use gix_ref::{FullName, FullNameRef};

    use crate::bstr::BStr;
    use crate::config::cache::util::ApplyLeniencyDefault;
    use crate::config::tree::{Branch, Push, Section};
    use crate::repository::branch_remote_ref_name;
    use crate::{push, remote};

    /// Query configuration related to branches.
    impl crate::Repository {
        /// Return a set of unique short branch names for which custom configuration exists in the configuration,
        /// if we deem them [trustworthy][crate::open::Options::filter_config_section()].
        ///
        /// ### Note
        ///
        /// Branch names that have illformed UTF-8 will silently be skipped.
        pub fn branch_names(&self) -> BTreeSet<&str> {
            self.subsection_str_names_of("branch")
        }

        /// Returns the validated reference on the remote associated with the given `name`,
        /// which will be used when *merging*.
        /// The returned value corresponds to the `branch.<short_branch_name>.merge` configuration key.
        ///
        /// Returns `None` if there is no value at the given key, or if no remote or remote ref is configured.
        /// May return an error if the reference name to be returned is invalid.
        ///
        /// ### Note
        ///
        /// This name refers to what Git calls upstream branch (as opposed to upstream *tracking* branch).
        #[doc(alias = "branch_upstream_name", alias = "git2")]
        pub fn branch_remote_ref_name(
            &self,
            name: &FullNameRef,
            direction: remote::Direction,
        ) -> Option<Result<Cow<'_, FullNameRef>, branch_remote_ref_name::Error>> {
            match direction {
                remote::Direction::Fetch => {
                    let short_name = name.shorten();
                    self.config
                        .resolved
                        .string("branch", Some(short_name), Branch::MERGE.name)
                        .map(|name| crate::config::tree::branch::Merge::try_into_fullrefname(name).map_err(Into::into))
                }
                remote::Direction::Push => {
                    let remote = match self.branch_remote(name.shorten(), direction)? {
                        Ok(r) => r,
                        Err(err) => return Some(Err(err.into())),
                    };
                    if remote.push_specs.is_empty() {
                        let push_default = match self
                            .config
                            .resolved
                            .string(Push.name(), None, Push::DEFAULT.name)
                            .map_or(Ok(Default::default()), |v| {
                                Push::DEFAULT
                                    .try_into_default(v)
                                    .with_lenient_default(self.config.lenient_config)
                            }) {
                            Ok(v) => v,
                            Err(err) => return Some(Err(err.into())),
                        };
                        match push_default {
                            push::Default::Nothing => None,
                            push::Default::Current | push::Default::Matching => Some(Ok(Cow::Owned(name.to_owned()))),
                            push::Default::Upstream => self.branch_remote_ref_name(name, remote::Direction::Fetch),
                            push::Default::Simple => {
                                match self.branch_remote_ref_name(name, remote::Direction::Fetch)? {
                                    Ok(fetch_ref) if fetch_ref.as_ref() == name => Some(Ok(fetch_ref)),
                                    Err(err) => Some(Err(err)),
                                    Ok(_different_fetch_ref) => None,
                                }
                            }
                        }
                    } else {
                        let search = gix_refspec::MatchGroup::from_push_specs(
                            remote
                                .push_specs
                                .iter()
                                .map(gix_refspec::RefSpec::to_ref)
                                .filter(|spec| spec.destination().is_some()),
                        );
                        let null_id = self.object_hash().null();
                        let out = search.match_remotes(
                            Some(gix_refspec::match_group::Item {
                                full_ref_name: name.as_bstr(),
                                target: &null_id,
                                object: None,
                            })
                            .into_iter(),
                        );
                        out.mappings.into_iter().next().and_then(|m| {
                            m.rhs.map(|name| {
                                FullName::try_from(name.into_owned())
                                    .map(Cow::Owned)
                                    .map_err(Into::into)
                            })
                        })
                    }
                }
            }
        }

        /// Returns the unvalidated name of the remote associated with the given `short_branch_name`,
        /// typically `main` instead of `refs/heads/main`.
        /// In some cases, the returned name will be an URL.
        /// Returns `None` if the remote was not found or if the name contained illformed UTF-8.
        ///
        /// * if `direction` is [remote::Direction::Fetch], we will query the `branch.<short_name>.remote` configuration.
        /// * if `direction` is [remote::Direction::Push], the push remote will be queried by means of `branch.<short_name>.pushRemote`
        ///   or `remote.pushDefault` as fallback.
        ///
        /// See also [`Reference::remote_name()`][crate::Reference::remote_name()] for a more typesafe version
        /// to be used when a `Reference` is available.
        ///
        /// `short_branch_name` can typically be obtained by [shortening a full branch name](FullNameRef::shorten()).
        #[doc(alias = "branch_upstream_remote", alias = "git2")]
        pub fn branch_remote_name<'a>(
            &self,
            short_branch_name: impl Into<&'a BStr>,
            direction: remote::Direction,
        ) -> Option<remote::Name<'_>> {
            let name = short_branch_name.into();
            let config = &self.config.resolved;
            (direction == remote::Direction::Push)
                .then(|| {
                    config
                        .string("branch", Some(name), Branch::PUSH_REMOTE.name)
                        .or_else(|| config.string("remote", None, crate::config::tree::Remote::PUSH_DEFAULT.name))
                })
                .flatten()
                .or_else(|| config.string("branch", Some(name), Branch::REMOTE.name))
                .and_then(|name| name.try_into().ok())
        }

        /// Like [`branch_remote_name(…)`](Self::branch_remote_name()), but returns a [Remote](crate::Remote).
        /// `short_branch_name` is the name to use for looking up `branch.<short_branch_name>.*` values in the
        /// configuration.
        pub fn branch_remote<'a>(
            &self,
            short_branch_name: impl Into<&'a BStr>,
            direction: remote::Direction,
        ) -> Option<Result<crate::Remote<'_>, remote::find::existing::Error>> {
            let name = self.branch_remote_name(short_branch_name, direction)?;
            self.try_find_remote(name.as_bstr())
                .map(|res| res.map_err(Into::into))
                .or_else(|| match name {
                    remote::Name::Url(url) => gix_url::parse(url.as_ref())
                        .map_err(Into::into)
                        .and_then(|url| {
                            self.remote_at(url)
                                .map_err(|err| remote::find::existing::Error::Find(remote::find::Error::Init(err)))
                        })
                        .into(),
                    remote::Name::Symbol(_) => None,
                })
        }
    }
}

impl crate::Repository {
    pub(crate) fn filter_config_section(&self) -> fn(&gix_config::file::Metadata) -> bool {
        self.options
            .filter_config_section
            .unwrap_or(config::section::is_trusted)
    }

    fn subsection_str_names_of<'a>(&'a self, header_name: &'a str) -> BTreeSet<&'a str> {
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
