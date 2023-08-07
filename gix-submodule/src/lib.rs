//! Primitives for describing git submodules.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

use bstr::BStr;
use std::borrow::Cow;
use std::collections::BTreeMap;

/// All relevant information about a git module, typically from `.gitmodules` files.
///
/// Note that overrides from other configuration might be relevant, which is why this type
/// can be used to take these into consideration when presented with other configuration
/// from the superproject.
#[derive(Clone)]
pub struct File {
    config: gix_config::File<'static>,
}

mod access;

///
pub mod config;

/// Mutation
impl File {
    /// This can be used to let `config` override some values we know about submodules, namely…
    ///
    /// * `url`
    /// * `fetchRecurseSubmodules`
    /// * `ignore`
    /// * `update`
    /// * `branch`
    ///
    /// These values aren't validated yet, which will happen upon query.
    pub fn append_submodule_overrides(&mut self, config: &gix_config::File<'_>) -> &mut Self {
        let mut values = BTreeMap::<_, Vec<_>>::new();
        for (module_name, section) in config
            .sections_by_name("submodule")
            .into_iter()
            .flatten()
            .filter_map(|s| s.header().subsection_name().map(|n| (n, s)))
        {
            for field in ["url", "fetchRecurseSubmodules", "ignore", "update", "branch"] {
                if let Some(value) = section.value(field) {
                    values.entry((module_name, field)).or_default().push(value);
                }
            }
        }

        let values = {
            let mut v: Vec<_> = values.into_iter().collect();
            v.sort_by_key(|a| a.0 .0);
            v
        };

        let mut config_to_append = gix_config::File::new(config.meta_owned());
        let mut prev_name = None;
        let mut section = None;
        for ((module_name, field), values) in values {
            if prev_name.map_or(true, |pn: &BStr| pn != module_name) {
                section.take();
                section = Some(
                    config_to_append
                        .new_section("submodule", Cow::Owned(module_name.to_owned()))
                        .expect("all names come from valid configuration, so remain valid"),
                );
                prev_name = Some(module_name);
            }
            let section = section.as_mut().expect("always set at this point");
            section.push(
                field.try_into().expect("statically known key"),
                Some(values.last().expect("at least one value or we wouldn't be here")),
            );
        }

        self.config.append(config_to_append);
        self
    }
}

///
mod init {
    use crate::File;
    use std::path::PathBuf;

    impl std::fmt::Debug for File {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("File")
                .field("config_path", &self.config_path())
                .field("config", &format_args!("r#\"{}\"#", self.config))
                .finish()
        }
    }

    /// Lifecycle
    impl File {
        /// Parse `bytes` as git configuration, typically from `.gitmodules`, without doing any further validation.
        /// `path` can be provided to keep track of where the file was read from in the underlying [`config`](Self::config())
        /// instance.
        ///
        /// Future access to the module information is lazy and configuration errors are exposed there on a per-value basis.
        ///
        /// ### Security Considerations
        ///
        /// The information itself should be used with care as it can direct the caller to fetch from remotes. It is, however,
        /// on the caller to assure the input data can be trusted.
        pub fn from_bytes(bytes: &[u8], path: impl Into<Option<PathBuf>>) -> Result<Self, gix_config::parse::Error> {
            let metadata = path.into().map_or_else(gix_config::file::Metadata::api, |path| {
                gix_config::file::Metadata::from(gix_config::Source::Worktree).at(path)
            });
            let config = gix_config::File::from_parse_events_no_includes(
                gix_config::parse::Events::from_bytes_owned(bytes, None)?,
                metadata,
            );

            Ok(Self { config })
        }

        /// Turn ourselves into the underlying parsed configuration file.
        pub fn into_config(self) -> gix_config::File<'static> {
            self.config
        }
    }
}
