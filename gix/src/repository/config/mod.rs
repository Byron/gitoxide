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

mod branch;
mod remote;
#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
mod transport;

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
