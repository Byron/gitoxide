use std::{
    borrow::Cow,
    collections::{HashMap, VecDeque},
    convert::TryFrom,
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::{
    file::{
        error::GitConfigError,
        section::{MutableSection, SectionBody},
        value::{EntryData, MutableMultiValue, MutableValue},
        Index, Size,
    },
    parser,
    parser::{
        parse_from_bytes, parse_from_path, parse_from_str, Error, Event, Key, ParsedSectionHeader, Parser,
        SectionHeaderName,
    },
    values,
};

/// The section ID is a monotonically increasing ID used to refer to sections.
/// This value does not imply any ordering between sections, as new sections
/// with higher section IDs may be in between lower ID sections.
///
/// We need to use a section id because `git-config` permits sections with
/// identical names. As a result, we can't simply use the section name as a key
/// in a map.
///
/// This id guaranteed to be unique, but not guaranteed to be compact. In other
/// words, it's possible that a section may have an ID of 3 but the next section
/// has an ID of 5.
#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord, Debug)]
pub(super) struct SectionId(usize);

/// Internal data structure for the section id lookup tree used by
/// [`GitConfig`]. Note that order in Vec matters as it represents the order
/// of section ids with the matched section and name, and is used for precedence
/// management.
#[derive(PartialEq, Eq, Clone, Debug)]
pub(super) enum LookupTreeNode<'a> {
    Terminal(Vec<SectionId>),
    NonTerminal(HashMap<Cow<'a, str>, Vec<SectionId>>),
}

/// High level `git-config` reader and writer.
///
/// This is the full-featured implementation that can deserialize, serialize,
/// and edit `git-config` files without loss of whitespace or comments. As a
/// result, it's lot more complex than it's read-only variant,
/// [`ResolvedGitConfig`] that exposes a [`HashMap`]-like interface. Users that
/// only need to read `git-config` files should use that instead.
///
/// Internally, this uses various acceleration data structures to improve
/// performance of the typical usage behavior of many lookups and relatively
/// fewer insertions.
///
/// # Multivar behavior
///
/// `git` is flexible enough to allow users to set a key multiple times in
/// any number of identically named sections. When this is the case, the key
/// is known as a "multivar". In this case, `get_raw_value` follows the
/// "last one wins" approach that `git-config` internally uses for multivar
/// resolution.
///
/// Concretely, the following config has a multivar, `a`, with the values
/// of `b`, `c`, and `d`, while `e` is a single variable with the value
/// `f g h`.
///
/// ```text
/// [core]
///     a = b
///     a = c
/// [core]
///     a = d
///     e = f g h
/// ```
///
/// Calling methods that fetch or set only one value (such as [`get_raw_value`])
/// key `a` with the above config will fetch `d` or replace `d`, since the last
/// valid config key/value pair is `a = d`:
///
/// ```
/// # use git_config::file::GitConfig;
/// # use std::borrow::Cow;
/// # use std::convert::TryFrom;
/// # let git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
/// assert_eq!(git_config.get_raw_value("core", None, "a"), Ok(Cow::Borrowed("d".as_bytes())));
/// ```
///
/// Consider the `multi` variants of the methods instead, if you want to work
/// with all values instead.
///
/// [`ResolvedGitConfig`]: crate::file::ResolvedGitConfig
/// [`get_raw_value`]: Self::get_raw_value
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct GitConfig<'event> {
    /// The list of events that occur before an actual section. Since a
    /// `git-config` file prohibits global values, this vec is limited to only
    /// comment, newline, and whitespace events.
    frontmatter_events: SectionBody<'event>,
    /// Section name and subsection name to section id lookup tree. This is
    /// effectively a n-tree (opposed to a binary tree) that can have a height
    /// of at most three (including an implicit root node).
    pub(super) section_lookup_tree: HashMap<SectionHeaderName<'event>, Vec<LookupTreeNode<'event>>>,
    /// SectionId to section mapping. The value of this HashMap contains actual
    /// events.
    ///
    /// This indirection with the SectionId as the key is critical to flexibly
    /// supporting `git-config` sections, as duplicated keys are permitted.
    pub(super) sections: HashMap<SectionId, SectionBody<'event>>,
    section_headers: HashMap<SectionId, ParsedSectionHeader<'event>>,
    /// Internal monotonically increasing counter for section ids.
    section_id_counter: usize,
    /// Section order for output ordering.
    section_order: VecDeque<SectionId>,
}

pub mod from_paths {
    use crate::file::GitConfig;
    use crate::parser;
    use crate::values::path::interpolate;
    use quick_error::quick_error;
    use std::borrow::Cow;

    quick_error! {
        #[derive(Debug)]
        /// The error returned by [`GitConfig::from_paths()`][super::GitConfig::from_paths()] and [`GitConfig::from_env_paths()`][super::GitConfig::from_env_paths()].
        #[allow(missing_docs)]
        pub enum Error {
            ParserOrIoError(err: parser::ParserOrIoError<'static>) {
                display("Could not read config")
                source(err)
                from()
            }
            Interpolate(err: interpolate::Error) {
                display("Could not interpolate path")
                source(err)
                from()
            }
            IncludeDepthExceeded { max_depth: u8 } {
                display("The maximum allowed length {} of the file include chain built by following nested includes is exceeded", max_depth)
            }
        }
    }

    /// Options when loading git config using [`GitConfig::from_paths()`][super::GitConfig::from_paths()].
    #[derive(Clone)]
    pub struct Options<'a> {
        /// The location where gitoxide or git is installed
        pub git_install_dir: Option<Cow<'a, std::path::Path>>,
        /// The maximum allowed length of the file include chain built by following nested includes where top level is depth = 1.
        pub max_depth: u8,
        /// When max depth is exceeded, while following nested included, returns an error if true or unit if false   
        pub error_on_max_depth_exceeded: bool,
        /// Initial `GitConfig` (possibly empty) that gets new sections, originating from the included config files
        pub config: GitConfig<'a>,
    }

    impl<'a> Default for Options<'a> {
        fn default() -> Self {
            Options {
                git_install_dir: None,
                max_depth: 10,
                error_on_max_depth_exceeded: true,
                config: GitConfig::default(),
            }
        }
    }
}

pub mod from_env {
    use super::from_paths;
    use crate::values::path::interpolate;
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        /// Represents the errors that may occur when calling [`GitConfig::from_env`].
        ///
        /// [`GitConfig::from_env`]: crate::file::GitConfig::from_env
        #[allow(missing_docs)]
        pub enum Error {
            /// `GIT_CONFIG_COUNT` was not a positive integer
            ParseError (err: String) {
                display("GIT_CONFIG_COUNT was not a positive integer: {}", err)
            }
            /// `GIT_CONFIG_KEY_<n>` was not set.
            InvalidKeyId (key_id: usize) {
                display("GIT_CONFIG_KEY_{} was not set.", key_id)
            }
            /// An key at `GIT_CONFIG_KEY_<n>` was found, but it wasn't a valid string.
            InvalidKeyValue (key_id: usize, key_val: String) {
                display("GIT_CONFIG_KEY_{} was set to an invalid value: {}", key_id, key_val)
            }
            /// `GIT_CONFIG_VALUE_<n>` was not set.
            InvalidValueId (value_id: usize) {
                display("GIT_CONFIG_VALUE_{} was not set.", value_id)
            }
            /// Could not interpolate path while loading a config file.
            PathInterpolationError (err: interpolate::Error) {
                display("Could not interpolate path.")
                source(err)
                from()
            }
            /// Could not load config from a file
            FromPathsError (err: from_paths::Error) {
                display("Could not load config from a file")
                source(err)
                from()
            }
        }
    }
}

impl<'event> GitConfig<'event> {
    /// Constructs an empty `git-config` file.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a `git-config` file from the provided path.
    ///
    /// # Errors
    ///
    /// Returns an error if there was an IO error or if the file wasn't a valid
    /// git-config file.
    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, parser::ParserOrIoError<'static>> {
        parse_from_path(path).map(Self::from)
    }

    /// Constructs a `git-config` file from the provided paths in the order provided.
    /// This is neither zero-copy nor zero-alloc.
    ///
    /// # Errors
    ///
    /// Returns an error if there was an IO error or if a file wasn't a valid
    /// git-config file.
    ///
    /// [`git-config`'s documentation]: https://git-scm.com/docs/git-config#Documentation/git-config.txt-FILES
    #[inline]
    pub fn from_paths(paths: Vec<PathBuf>, options: &from_paths::Options) -> Result<Self, from_paths::Error> {
        let config = Self::new();
        config.resolve_includes(paths, options)
    }

    pub(crate) fn resolve_includes(
        mut self,
        paths: Vec<PathBuf>,
        options: &from_paths::Options,
    ) -> Result<Self, from_paths::Error> {
        fn from_paths_recursive(
            paths: Vec<PathBuf>,
            config: &mut GitConfig,
            depth: u8,
            options: &from_paths::Options,
        ) -> Result<(), from_paths::Error> {
            if depth > options.max_depth {
                return if options.error_on_max_depth_exceeded {
                    Err(from_paths::Error::IncludeDepthExceeded {
                        max_depth: options.max_depth,
                    })
                } else {
                    Ok(())
                };
            }
            for config_file_path in paths.iter() {
                let other = GitConfig::open(config_file_path)?;
                let mut section_indices: Vec<_> = other.section_headers.keys().collect();
                // header keys are numeric and ascend in insertion order, hence sorting them gives the order
                // in which they appear in the config file.
                section_indices.sort();
                for section_index in section_indices {
                    let section_header = &other.section_headers[section_index];
                    config.push_section(
                        section_header.name.0.to_owned(),
                        section_header.subsection_name.to_owned(),
                        other.sections[section_index].clone(),
                    );
                    if section_header.name.0 != "include" {
                        continue;
                    }
                    let path_values = other.sections[section_index].values(&Key::from("path"));
                    let mut paths = vec![];
                    for path_value in path_values {
                        let interpolated_path =
                            values::Path::from(path_value).interpolate(options.git_install_dir.as_deref())?;
                        let path = if interpolated_path.is_relative() {
                            config_file_path
                                .parent()
                                .expect("path is a config file which naturally lives in a directory")
                                .join(interpolated_path)
                        } else {
                            interpolated_path.into()
                        };
                        if let Ok(path) = path.canonicalize() {
                            paths.push(path);
                        }
                    }
                    from_paths_recursive(paths, config, depth + 1, options)?;
                }
            }
            Ok(())
        }

        from_paths_recursive(paths, &mut self, 1, &options)?;

        Ok(self)
    }

    /// Constructs a `git-config` from the default cascading sequence.
    /// This is neither zero-alloc nor zero-copy.
    ///
    /// See <https://git-scm.com/docs/git-config#FILES> for details.
    pub fn from_env_paths(options: &from_paths::Options) -> Result<Self, from_paths::Error> {
        use std::env;

        let mut paths = vec![];

        if env::var("GIT_CONFIG_NO_SYSTEM").is_err() {
            if let Ok(git_config_system) = env::var("GIT_CONFIG_SYSTEM") {
                paths.push(PathBuf::from(git_config_system))
            } else {
                // In git the fallback is set to a build time macro which defaults to /etc/gitconfig
                paths.push(PathBuf::from("/etc/gitconfig"));
            }
        }

        if let Ok(git_config_global) = env::var("GIT_CONFIG_GLOBAL") {
            paths.push(PathBuf::from(git_config_global));
        } else {
            // Divergence from git-config(1)
            // These two are supposed to share the same scope and override
            // rather than append according to git-config(1) documentation.
            if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
                paths.push(PathBuf::from(xdg_config_home).join("git/config"));
            } else if let Ok(home) = env::var("HOME") {
                paths.push(PathBuf::from(home).join(".config/git/config"));
            }

            if let Ok(home) = env::var("HOME") {
                paths.push(PathBuf::from(home).join(".gitconfig"));
            }
        }

        if let Ok(git_dir) = env::var("GIT_DIR") {
            paths.push(PathBuf::from(git_dir).join("config"));
        }

        Self::from_paths(paths, options)
    }

    /// Generates a config from the environment variables. This is neither
    /// zero-copy nor zero-alloc. See [`git-config`'s documentation] on
    /// environment variable for more information.
    ///
    /// # Errors
    ///
    /// Returns an error if `GIT_CONFIG_COUNT` set and is not a number, or if
    /// there was an invalid key value pair.
    ///
    /// [`git-config`'s documentation]: https://git-scm.com/docs/git-config#Documentation/git-config.txt-GITCONFIGCOUNT
    pub fn from_env(options: &from_paths::Options) -> Result<Option<Self>, from_env::Error> {
        use std::env;
        let count: usize = match env::var("GIT_CONFIG_COUNT") {
            Ok(v) => v.parse().map_err(|_| from_env::Error::ParseError(v))?,
            Err(_) => return Ok(None),
        };

        let mut config = Self::new();
        let mut paths = vec![];

        for i in 0..count {
            let key = env::var(format!("GIT_CONFIG_KEY_{}", i)).map_err(|_| from_env::Error::InvalidKeyId(i))?;
            let value = env::var(format!("GIT_CONFIG_VALUE_{}", i)).map_err(|_| from_env::Error::InvalidValueId(i))?;
            if let Some((section, maybe_subsection)) = key.split_once('.') {
                let (subsection, key) = if let Some((subsection, key)) = maybe_subsection.rsplit_once('.') {
                    (Some(subsection), key)
                } else {
                    (None, maybe_subsection)
                };

                let mut section = if let Ok(section) = config.section_mut(section, subsection) {
                    section
                } else {
                    // Need to have config own the section and subsection names
                    // else they get dropped at the end of the loop.
                    config.new_section(
                        section.to_string(),
                        subsection.map(|subsection| Cow::Owned(subsection.to_string())),
                    )
                };

                section.push(
                    Cow::<str>::Owned(key.to_string()).into(),
                    Cow::Owned(value.into_bytes()),
                );

                paths = section
                    .values(&Key::from("path"))
                    .iter()
                    .map(|val| {
                        values::Path::from(val.clone())
                            .interpolate(options.git_install_dir.as_deref())
                            .map(|path| path.into_owned())
                    })
                    .collect::<Result<Vec<_>, _>>()?;
            } else {
                return Err(from_env::Error::InvalidKeyValue(i, key.to_string()));
            }
        }

        // This occurs when `GIT_CONFIG_COUNT` is set to zero.
        if config.is_empty() {
            Ok(None)
        } else {
            let config = config.resolve_includes(paths, options)?;
            Ok(Some(config))
        }
    }

    /// Returns an interpreted value given a section, an optional subsection and
    /// key.
    ///
    /// It's recommended to use one of the values in the [`values`] module as
    /// the conversion is already implemented, but this function is flexible and
    /// will accept any type that implements [`TryFrom<&[u8]>`][`TryFrom`].
    ///
    /// Consider [`Self::multi_value`] if you want to get all values of a
    /// multivar instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use git_config::values::{Integer, Boolean};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// let config = r#"
    ///     [core]
    ///         a = 10k
    ///         c
    /// "#;
    /// let git_config = GitConfig::try_from(config)?;
    /// // You can either use the turbofish to determine the type...
    /// let a_value = git_config.value::<Integer>("core", None, "a")?;
    /// // ... or explicitly declare the type to avoid the turbofish
    /// let c_value: Boolean = git_config.value("core", None, "c")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in the requested
    /// section and subsection, if the section and subsection do not exist, or
    /// if there was an issue converting the type into the requested variant.
    ///
    /// [`values`]: crate::values
    /// [`TryFrom`]: std::convert::TryFrom
    #[inline]
    pub fn value<'lookup, T: TryFrom<Cow<'event, [u8]>>>(
        &'event self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<T, GitConfigError<'lookup>> {
        T::try_from(self.get_raw_value(section_name, subsection_name, key)?)
            .map_err(|_| GitConfigError::FailedConversion)
    }

    /// Returns all interpreted values given a section, an optional subsection
    /// and key.
    ///
    /// It's recommended to use one of the values in the [`values`] module as
    /// the conversion is already implemented, but this function is flexible and
    /// will accept any type that implements [`TryFrom<&[u8]>`][`TryFrom`].
    ///
    /// Consider [`Self::value`] if you want to get a single value
    /// (following last-one-wins resolution) instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use git_config::values::{Integer, Bytes, Boolean, TrueVariant};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// let config = r#"
    ///     [core]
    ///         a = true
    ///         c = g
    ///     [core]
    ///         a
    ///         a = false
    /// "#;
    /// let git_config = GitConfig::try_from(config).unwrap();
    /// // You can either use the turbofish to determine the type...
    /// let a_value = git_config.multi_value::<Boolean>("core", None, "a")?;
    /// assert_eq!(
    ///     a_value,
    ///     vec![
    ///         Boolean::True(TrueVariant::Explicit(Cow::Borrowed("true"))),
    ///         Boolean::True(TrueVariant::Implicit),
    ///         Boolean::False(Cow::Borrowed("false")),
    ///     ]
    /// );
    /// // ... or explicitly declare the type to avoid the turbofish
    /// let c_value: Vec<Bytes> = git_config.multi_value("core", None, "c")?;
    /// assert_eq!(c_value, vec![Bytes { value: Cow::Borrowed(b"g") }]);
    /// # Ok::<(), GitConfigError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in the requested
    /// section and subsection, if the section and subsection do not exist, or
    /// if there was an issue converting the type into the requested variant.
    ///
    /// [`values`]: crate::values
    /// [`TryFrom`]: std::convert::TryFrom
    #[inline]
    pub fn multi_value<'lookup, T: TryFrom<Cow<'event, [u8]>>>(
        &'event self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<Vec<T>, GitConfigError<'lookup>> {
        self.get_raw_multi_value(section_name, subsection_name, key)?
            .into_iter()
            .map(T::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| GitConfigError::FailedConversion)
    }

    /// Returns an immutable section reference.
    ///
    /// # Errors
    ///
    /// This function will return an error if the section and optional
    /// subsection do not exist.
    pub fn section<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
    ) -> Result<&SectionBody<'event>, GitConfigError<'lookup>> {
        let section_ids = self.get_section_ids_by_name_and_subname(section_name, subsection_name)?;
        let id = section_ids
            .last()
            .expect("Section lookup vec was empty, internal invariant violated");
        Ok(self
            .sections
            .get(id)
            .expect("Section did not have id from lookup, internal invariant violated"))
    }

    /// Returns an mutable section reference.
    ///
    /// # Errors
    ///
    /// This function will return an error if the section and optional
    /// subsection do not exist.
    pub fn section_mut<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
    ) -> Result<MutableSection<'_, 'event>, GitConfigError<'lookup>> {
        let section_ids = self.get_section_ids_by_name_and_subname(section_name, subsection_name)?;
        let id = section_ids
            .last()
            .expect("Section lookup vec was empty, internal invariant violated");
        Ok(MutableSection::new(self.sections.get_mut(id).expect(
            "Section did not have id from lookup, internal invariant violated",
        )))
    }

    /// Gets all sections that match the provided name, ignoring any subsections.
    ///
    /// # Examples
    ///
    /// Provided the following config:
    ///
    /// ```text
    /// [core]
    ///     a = b
    /// [core ""]
    ///     c = d
    /// [core "apple"]
    ///     e = f
    /// ```
    ///
    /// Calling this method will yield all sections:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use git_config::values::{Integer, Boolean, TrueVariant};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// let config = r#"
    ///     [core]
    ///         a = b
    ///     [core ""]
    ///         c = d
    ///     [core "apple"]
    ///         e = f
    /// "#;
    /// let git_config = GitConfig::try_from(config).unwrap();
    /// assert_eq!(git_config.sections_by_name("core").len(), 3);
    /// ```
    #[must_use]
    pub fn sections_by_name<'lookup>(&self, section_name: &'lookup str) -> Vec<&SectionBody<'event>> {
        self.get_section_ids_by_name(section_name)
            .unwrap_or_default()
            .into_iter()
            .map(|id| {
                self.sections
                    .get(&id)
                    .expect("section doesn't have id from from lookup")
            })
            .collect()
    }

    /// Get all sections that match the `section_name`, returning all matching section header along with their body.
    ///
    /// An empty `Vec` is returned if there is no section with `section_name`.
    ///
    /// # Example
    ///
    /// Provided the following config:
    /// ```plain
    /// [url "ssh://git@github.com/"]
    ///     insteadOf = https://github.com/
    /// [url "ssh://git@bitbucket.org"]
    ///     insteadOf = https://bitbucket.org/
    /// ```
    /// Calling this method will yield all section bodies and their header:
    ///
    /// ```rust
    /// use git_config::file::{GitConfig, GitConfigError};
    /// use git_config::parser::Key;
    /// use std::borrow::Cow;
    /// use std::convert::TryFrom;
    /// use nom::AsBytes;
    ///
    /// let input = r#"
    /// [url "ssh://git@github.com/"]
    ///    insteadOf = https://github.com/
    /// [url "ssh://git@bitbucket.org"]
    ///    insteadOf = https://bitbucket.org/
    /// "#;
    /// let config = GitConfig::try_from(input).unwrap();
    /// let url = config.sections_by_name_with_header("url");
    /// assert_eq!(url.len(), 2);
    ///
    /// for (i, (header, body)) in url.iter().enumerate() {
    ///     let url = header.subsection_name.as_ref();
    ///     let instead_of = body.value(&Key::from("insteadOf"));
    ///
    ///     // todo(unstable-order): the order is not always the same, so `i` cannot be used here
    ///     if instead_of.as_ref().unwrap().as_ref().as_bytes().eq("https://github.com/".as_bytes()) {
    ///         assert_eq!(instead_of.unwrap().as_ref(), "https://github.com/".as_bytes());
    ///         assert_eq!(url.unwrap().as_ref(), "ssh://git@github.com/");
    ///     } else {
    ///         assert_eq!(instead_of.unwrap().as_ref(), "https://bitbucket.org/".as_bytes());
    ///         assert_eq!(url.unwrap().as_ref(), "ssh://git@bitbucket.org");
    ///     }
    /// }
    /// ```
    pub fn sections_by_name_with_header<'lookup>(
        &self,
        section_name: &'lookup str,
    ) -> Vec<(&ParsedSectionHeader<'event>, &SectionBody<'event>)> {
        self.get_section_ids_by_name(section_name)
            .unwrap_or_default()
            .into_iter()
            .map(|id| {
                (
                    self.section_headers
                        .get(&id)
                        .expect("section doesn't have a section header??"),
                    self.sections
                        .get(&id)
                        .expect("section doesn't have id from from lookup"),
                )
            })
            .collect()
    }

    /// Adds a new section to config. If a subsection name was provided, then
    /// the generated header will use the modern subsection syntax. Returns a
    /// reference to the new section for immediate editing.
    ///
    /// # Examples
    ///
    /// Creating a new empty section:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::convert::TryFrom;
    /// let mut git_config = GitConfig::new();
    /// let _section = git_config.new_section("hello", Some("world".into()));
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n");
    /// ```
    ///
    /// Creating a new empty section and adding values to it:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::convert::TryFrom;
    /// let mut git_config = GitConfig::new();
    /// let mut section = git_config.new_section("hello", Some("world".into()));
    /// section.push("a".into(), "b".as_bytes().into());
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n  a=b\n");
    /// let _section = git_config.new_section("core", None);
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n  a=b\n[core]\n");
    /// ```
    pub fn new_section(
        &mut self,
        section_name: impl Into<Cow<'event, str>>,
        subsection_name: impl Into<Option<Cow<'event, str>>>,
    ) -> MutableSection<'_, 'event> {
        let mut section = self.push_section(section_name, subsection_name, SectionBody::new());
        section.push_newline();
        section
    }

    /// Removes the section, returning the events it had, if any. If multiple
    /// sections have the same name, then the last one is returned. Note that
    /// later sections with the same name have precedent over earlier ones.
    ///
    /// # Examples
    ///
    /// Creating and removing a section:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::convert::TryFrom;
    /// let mut git_config = GitConfig::try_from(
    /// r#"[hello "world"]
    ///     some-value = 4
    /// "#).unwrap();
    ///
    /// let events = git_config.remove_section("hello", Some("world".into()));
    /// assert_eq!(git_config.to_string(), "");
    /// ```
    ///
    /// Precedence example for removing sections with the same name:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::convert::TryFrom;
    /// let mut git_config = GitConfig::try_from(
    /// r#"[hello "world"]
    ///     some-value = 4
    /// [hello "world"]
    ///     some-value = 5
    /// "#).unwrap();
    ///
    /// let events = git_config.remove_section("hello", Some("world".into()));
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n    some-value = 4\n");
    /// ```
    pub fn remove_section<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: impl Into<Option<&'lookup str>>,
    ) -> Option<SectionBody> {
        let id = self
            .get_section_ids_by_name_and_subname(section_name, subsection_name.into())
            .ok()?
            .pop()?;
        self.section_order.remove(
            self.section_order
                .iter()
                .position(|v| *v == id)
                .expect("Section order does not contain section that we were trying to remove"),
        );
        self.sections.remove(&id)
    }

    /// Adds the provided section to the config, returning a mutable reference
    /// to it.
    pub fn push_section(
        &mut self,
        section_name: impl Into<Cow<'event, str>>,
        subsection_name: impl Into<Option<Cow<'event, str>>>,
        section: SectionBody<'event>,
    ) -> MutableSection<'_, 'event> {
        let subsection_name = subsection_name.into();
        if subsection_name.is_some() {
            self.push_section_internal(
                ParsedSectionHeader {
                    name: SectionHeaderName(section_name.into()),
                    separator: Some(" ".into()),
                    subsection_name,
                },
                section,
            )
        } else {
            self.push_section_internal(
                ParsedSectionHeader {
                    name: SectionHeaderName(section_name.into()),
                    separator: None,
                    subsection_name: None,
                },
                section,
            )
        }
    }

    /// Renames a section, modifying the last matching section.
    ///
    /// # Errors
    ///
    /// Returns an error if the lookup doesn't exist
    pub fn rename_section<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: impl Into<Option<&'lookup str>>,
        new_section_name: impl Into<SectionHeaderName<'event>>,
        new_subsection_name: impl Into<Option<Cow<'event, str>>>,
    ) -> Result<(), GitConfigError<'lookup>> {
        let id = self.get_section_ids_by_name_and_subname(section_name, subsection_name.into())?;
        let id = id
            .last()
            .expect("list of sections were empty, which violates invariant");
        let header = self
            .section_headers
            .get_mut(id)
            .expect("sections does not have section id from section ids");
        header.name = new_section_name.into();
        header.subsection_name = new_subsection_name.into();

        Ok(())
    }

    /// Returns the number of entries in the config. This ignores any comments.
    /// For example, a config with multiple empty sections will return 0.
    #[must_use]
    pub fn len(&self) -> usize {
        self.sections.values().fold(0, |acc, section| acc + section.len())
    }

    /// Returns if there are no entries in the config. This will return true
    /// if there are only empty sections or comments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sections.values().all(SectionBody::is_empty)
    }
}

/// # Raw value API
///
/// These functions are the raw value API. Instead of returning Rust structures,
/// these functions return bytes which may or may not be owned.
impl<'event> GitConfig<'event> {
    /// Returns an uninterpreted value given a section, an optional subsection
    /// and key.
    ///
    /// Consider [`Self::get_raw_multi_value`] if you want to get all values of
    /// a multivar instead.
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in the requested
    /// section and subsection, or if the section and subsection do not exist.
    pub fn get_raw_value<'lookup>(
        &self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<Cow<'_, [u8]>, GitConfigError<'lookup>> {
        // Note: cannot wrap around the raw_multi_value method because we need
        // to guarantee that the highest section id is used (so that we follow
        // the "last one wins" resolution strategy by `git-config`).
        let key = Key(key.into());
        for section_id in self
            .get_section_ids_by_name_and_subname(section_name, subsection_name)?
            .iter()
            .rev()
        {
            if let Some(v) = self
                .sections
                .get(section_id)
                .expect("sections does not have section id from section ids")
                .value(&key)
            {
                return Ok(v.to_vec().into());
            }
        }

        Err(GitConfigError::KeyDoesNotExist)
    }

    /// Returns a mutable reference to an uninterpreted value given a section,
    /// an optional subsection and key.
    ///
    /// Consider [`Self::get_raw_multi_value_mut`] if you want to get mutable
    /// references to all values of a multivar instead.
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in the requested
    /// section and subsection, or if the section and subsection do not exist.
    pub fn get_raw_value_mut<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<MutableValue<'_, 'lookup, 'event>, GitConfigError<'lookup>> {
        let section_ids = self.get_section_ids_by_name_and_subname(section_name, subsection_name)?;
        let key = Key(key.into());

        for section_id in section_ids.iter().rev() {
            let mut size = Size(0);
            let mut index = Index(0);
            let mut found_key = false;
            // todo: iter backwards
            for (i, event) in self
                .sections
                .get(section_id)
                .expect("sections does not have section id from section ids")
                .as_ref()
                .iter()
                .enumerate()
            {
                match event {
                    Event::Key(event_key) if *event_key == key => {
                        found_key = true;
                        size = Size(1);
                        index = Index(i);
                    }
                    Event::Newline(_) | Event::Whitespace(_) | Event::ValueNotDone(_) if found_key => {
                        size += 1;
                    }
                    Event::ValueDone(_) | Event::Value(_) if found_key => {
                        found_key = false;
                        size += 1;
                    }
                    _ => (),
                }
            }

            if size.0 == 0 {
                continue;
            }

            return Ok(MutableValue::new(
                MutableSection::new(
                    self.sections
                        .get_mut(section_id)
                        .expect("sections does not have section id from section ids"),
                ),
                key,
                index,
                size,
            ));
        }

        Err(GitConfigError::KeyDoesNotExist)
    }

    /// Returns all uninterpreted values given a section, an optional subsection
    /// and key.
    ///
    /// # Examples
    ///
    /// If you have the following config:
    ///
    /// ```text
    /// [core]
    ///     a = b
    /// [core]
    ///     a = c
    ///     a = d
    /// ```
    ///
    /// Attempting to get all values of `a` yields the following:
    ///
    /// ```
    /// # use git_config::file::GitConfig;
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// assert_eq!(
    ///     git_config.get_raw_multi_value("core", None, "a"),
    ///     Ok(vec![
    ///         Cow::<[u8]>::Borrowed(b"b"),
    ///         Cow::<[u8]>::Borrowed(b"c"),
    ///         Cow::<[u8]>::Borrowed(b"d"),
    ///     ]),
    /// );
    /// ```
    ///
    /// Consider [`Self::get_raw_value`] if you want to get the resolved single
    /// value for a given key, if your key does not support multi-valued values.
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in any requested
    /// section and subsection, or if no instance of the section and subsections
    /// exist.
    pub fn get_raw_multi_value<'lookup>(
        &self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<Vec<Cow<'_, [u8]>>, GitConfigError<'lookup>> {
        let mut values = vec![];
        for section_id in self.get_section_ids_by_name_and_subname(section_name, subsection_name)? {
            values.extend(
                self.sections
                    .get(&section_id)
                    .expect("sections does not have section id from section ids")
                    .values(&Key(key.into()))
                    .iter()
                    .map(|v| Cow::Owned(v.to_vec())),
            );
        }

        if values.is_empty() {
            Err(GitConfigError::KeyDoesNotExist)
        } else {
            Ok(values)
        }
    }

    /// Returns mutable references to all uninterpreted values given a section,
    /// an optional subsection and key.
    ///
    /// # Examples
    ///
    /// If you have the following config:
    ///
    /// ```text
    /// [core]
    ///     a = b
    /// [core]
    ///     a = c
    ///     a = d
    /// ```
    ///
    /// Attempting to get all values of `a` yields the following:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let mut git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// assert_eq!(
    ///     git_config.get_raw_multi_value("core", None, "a")?,
    ///     vec![
    ///         Cow::Borrowed(b"b"),
    ///         Cow::Borrowed(b"c"),
    ///         Cow::Borrowed(b"d")
    ///     ]
    /// );
    ///
    /// git_config.get_raw_multi_value_mut("core", None, "a")?.set_str_all("g");
    ///
    /// assert_eq!(
    ///     git_config.get_raw_multi_value("core", None, "a")?,
    ///     vec![
    ///         Cow::Borrowed(b"g"),
    ///         Cow::Borrowed(b"g"),
    ///         Cow::Borrowed(b"g")
    ///     ],
    /// );
    /// # Ok::<(), GitConfigError>(())
    /// ```
    ///
    /// Consider [`Self::get_raw_value`] if you want to get the resolved single
    /// value for a given key, if your key does not support multi-valued values.
    ///
    /// Note that this operation is relatively expensive, requiring a full
    /// traversal of the config.
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in any requested
    /// section and subsection, or if no instance of the section and subsections
    /// exist.
    pub fn get_raw_multi_value_mut<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<MutableMultiValue<'_, 'lookup, 'event>, GitConfigError<'lookup>> {
        let section_ids = self.get_section_ids_by_name_and_subname(section_name, subsection_name)?;
        let key = Key(key.into());

        let mut offsets = HashMap::new();
        let mut entries = vec![];
        for section_id in section_ids.iter().rev() {
            let mut last_boundary = 0;
            let mut found_key = false;
            let mut offset_list = vec![];
            let mut offset_index = 0;
            for (i, event) in self
                .sections
                .get(section_id)
                .expect("sections does not have section id from section ids")
                .as_ref()
                .iter()
                .enumerate()
            {
                match event {
                    Event::Key(event_key) if *event_key == key => {
                        found_key = true;
                        offset_list.push(i - last_boundary);
                        offset_index += 1;
                        last_boundary = i;
                    }
                    Event::Value(_) | Event::ValueDone(_) if found_key => {
                        found_key = false;
                        entries.push(EntryData::new(*section_id, offset_index));
                        offset_list.push(i - last_boundary + 1);
                        offset_index += 1;
                        last_boundary = i + 1;
                    }
                    _ => (),
                }
            }
            offsets.insert(*section_id, offset_list);
        }

        entries.sort();

        if entries.is_empty() {
            Err(GitConfigError::KeyDoesNotExist)
        } else {
            Ok(MutableMultiValue::new(&mut self.sections, key, entries, offsets))
        }
    }

    /// Sets a value in a given section, optional subsection, and key value.
    ///
    /// # Examples
    ///
    /// Given the config,
    ///
    /// ```text
    /// [core]
    ///     a = b
    /// [core]
    ///     a = c
    ///     a = d
    /// ```
    ///
    /// Setting a new value to the key `core.a` will yield the following:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let mut git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// git_config.set_raw_value("core", None, "a", vec![b'e'])?;
    /// assert_eq!(git_config.get_raw_value("core", None, "a")?, Cow::Borrowed(b"e"));
    /// # Ok::<(), GitConfigError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This errors if any lookup input (section, subsection, and key value) fails.
    pub fn set_raw_value<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
        new_value: Vec<u8>,
    ) -> Result<(), GitConfigError<'lookup>> {
        self.get_raw_value_mut(section_name, subsection_name, key)
            .map(|mut entry| entry.set_bytes(new_value))
    }

    /// Sets a multivar in a given section, optional subsection, and key value.
    ///
    /// This internally zips together the new values and the existing values.
    /// As a result, if more new values are provided than the current amount of
    /// multivars, then the latter values are not applied. If there are less
    /// new values than old ones then the remaining old values are unmodified.
    ///
    /// **Note**: Mutation order is _not_ guaranteed and is non-deterministic.
    /// If you need finer control over which values of the multivar are set,
    /// consider using [`get_raw_multi_value_mut`], which will let you iterate
    /// and check over the values instead. This is best used as a convenience
    /// function for setting multivars whose values should be treated as an
    /// unordered set.
    ///
    /// # Examples
    ///
    /// Let us use the follow config for all examples:
    ///
    /// ```text
    /// [core]
    ///     a = b
    /// [core]
    ///     a = c
    ///     a = d
    /// ```
    ///
    /// Setting an equal number of values:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let mut git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// let new_values: Vec<Cow<'_, [u8]>> = vec![
    ///     Cow::Borrowed(b"x"),
    ///     Cow::Borrowed(b"y"),
    ///     Cow::Borrowed(b"z"),
    /// ];
    /// git_config.set_raw_multi_value("core", None, "a", new_values.into_iter())?;
    /// let fetched_config = git_config.get_raw_multi_value("core", None, "a")?;
    /// assert!(fetched_config.contains(&Cow::Borrowed(b"x")));
    /// assert!(fetched_config.contains(&Cow::Borrowed(b"y")));
    /// assert!(fetched_config.contains(&Cow::Borrowed(b"z")));
    /// # Ok::<(), GitConfigError>(())
    /// ```
    ///
    /// Setting less than the number of present values sets the first ones found:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let mut git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// let new_values: Vec<Cow<'_, [u8]>> = vec![
    ///     Cow::Borrowed(b"x"),
    ///     Cow::Borrowed(b"y"),
    /// ];
    /// git_config.set_raw_multi_value("core", None, "a", new_values.into_iter())?;
    /// let fetched_config = git_config.get_raw_multi_value("core", None, "a")?;
    /// assert!(fetched_config.contains(&Cow::Borrowed(b"x")));
    /// assert!(fetched_config.contains(&Cow::Borrowed(b"y")));
    /// # Ok::<(), GitConfigError>(())
    /// ```
    ///
    /// Setting more than the number of present values discards the rest:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let mut git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// let new_values: Vec<Cow<'_, [u8]>> = vec![
    ///     Cow::Borrowed(b"x"),
    ///     Cow::Borrowed(b"y"),
    ///     Cow::Borrowed(b"z"),
    ///     Cow::Borrowed(b"discarded"),
    /// ];
    /// git_config.set_raw_multi_value("core", None, "a", new_values.into_iter())?;
    /// assert!(!git_config.get_raw_multi_value("core", None, "a")?.contains(&Cow::Borrowed(b"discarded")));
    /// # Ok::<(), GitConfigError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This errors if any lookup input (section, subsection, and key value) fails.
    ///
    /// [`get_raw_multi_value_mut`]: Self::get_raw_multi_value_mut
    pub fn set_raw_multi_value<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
        new_values: impl Iterator<Item = Cow<'event, [u8]>>,
    ) -> Result<(), GitConfigError<'lookup>> {
        self.get_raw_multi_value_mut(section_name, subsection_name, key)
            .map(|mut v| v.set_values(new_values))
    }
}

/// Private helper functions
impl<'event> GitConfig<'event> {
    /// Adds a new section to the config file.
    fn push_section_internal(
        &mut self,
        // current_section_name: Option<SectionHeaderName<'event>>,
        // current_subsection_name: Option<Cow<'event, str>>,
        header: ParsedSectionHeader<'event>,
        section: SectionBody<'event>,
    ) -> MutableSection<'_, 'event> {
        let new_section_id = SectionId(self.section_id_counter);
        self.section_headers.insert(new_section_id, header.clone());
        self.sections.insert(new_section_id, section);
        let lookup = self.section_lookup_tree.entry(header.name).or_default();

        let mut found_node = false;
        if let Some(subsection_name) = header.subsection_name {
            for node in lookup.iter_mut() {
                if let LookupTreeNode::NonTerminal(subsection) = node {
                    found_node = true;
                    subsection
                        // Clones the cow, not the inner borrowed str.
                        .entry(subsection_name.clone())
                        .or_default()
                        .push(new_section_id);
                    break;
                }
            }
            if !found_node {
                let mut map = HashMap::new();
                map.insert(subsection_name, vec![new_section_id]);
                lookup.push(LookupTreeNode::NonTerminal(map));
            }
        } else {
            for node in lookup.iter_mut() {
                if let LookupTreeNode::Terminal(vec) = node {
                    found_node = true;
                    vec.push(new_section_id);
                    break;
                }
            }
            if !found_node {
                lookup.push(LookupTreeNode::Terminal(vec![new_section_id]));
            }
        }
        self.section_order.push_back(new_section_id);
        self.section_id_counter += 1;
        self.sections.get_mut(&new_section_id).map(MutableSection::new).unwrap()
    }

    /// Returns the mapping between section and subsection name to section ids.
    fn get_section_ids_by_name_and_subname<'lookup>(
        &self,
        section_name: impl Into<SectionHeaderName<'lookup>>,
        subsection_name: Option<&'lookup str>,
    ) -> Result<Vec<SectionId>, GitConfigError<'lookup>> {
        let section_name = section_name.into();
        let section_ids = self
            .section_lookup_tree
            .get(&section_name)
            .ok_or(GitConfigError::SectionDoesNotExist(section_name))?;
        let mut maybe_ids = None;
        // Don't simplify if and matches here -- the for loop currently needs
        // `n + 1` checks, while the if and matches will result in the for loop
        // needing `2n` checks.
        if let Some(subsection_name) = subsection_name {
            for node in section_ids {
                if let LookupTreeNode::NonTerminal(subsection_lookup) = node {
                    maybe_ids = subsection_lookup.get(subsection_name);
                    break;
                }
            }
        } else {
            for node in section_ids {
                if let LookupTreeNode::Terminal(subsection_lookup) = node {
                    maybe_ids = Some(subsection_lookup);
                    break;
                }
            }
        }
        maybe_ids
            .map(Vec::to_owned)
            .ok_or(GitConfigError::SubSectionDoesNotExist(subsection_name))
    }

    fn get_section_ids_by_name<'lookup>(
        &self,
        section_name: impl Into<SectionHeaderName<'lookup>>,
    ) -> Result<Vec<SectionId>, GitConfigError<'lookup>> {
        let section_name = section_name.into();
        self.section_lookup_tree
            .get(&section_name)
            .map(|lookup| {
                lookup
                    .iter()
                    .flat_map(|node| match node {
                        LookupTreeNode::Terminal(v) => v.clone(),
                        LookupTreeNode::NonTerminal(v) => v.values().flatten().copied().collect(),
                    })
                    .collect()
            })
            .ok_or(GitConfigError::SectionDoesNotExist(section_name))
    }
}

impl<'a> TryFrom<&'a str> for GitConfig<'a> {
    type Error = Error<'a>;

    /// Convenience constructor. Attempts to parse the provided string into a
    /// [`GitConfig`]. See [`parse_from_str`] for more information.
    ///
    /// [`parse_from_str`]: crate::parser::parse_from_str
    #[inline]
    fn try_from(s: &'a str) -> Result<GitConfig<'a>, Self::Error> {
        parse_from_str(s).map(Self::from)
    }
}

impl<'a> TryFrom<&'a [u8]> for GitConfig<'a> {
    type Error = Error<'a>;

    /// Convenience constructor. Attempts to parse the provided byte string into
    //// a [`GitConfig`]. See [`parse_from_bytes`] for more information.
    ///
    /// [`parse_from_bytes`]: crate::parser::parse_from_bytes
    #[inline]
    fn try_from(value: &'a [u8]) -> Result<GitConfig<'a>, Self::Error> {
        parse_from_bytes(value).map(GitConfig::from)
    }
}

impl<'a> TryFrom<&'a Vec<u8>> for GitConfig<'a> {
    type Error = Error<'a>;

    /// Convenience constructor. Attempts to parse the provided byte string into
    //// a [`GitConfig`]. See [`parse_from_bytes`] for more information.
    ///
    /// [`parse_from_bytes`]: crate::parser::parse_from_bytes
    #[inline]
    fn try_from(value: &'a Vec<u8>) -> Result<GitConfig<'a>, Self::Error> {
        parse_from_bytes(value).map(GitConfig::from)
    }
}

impl<'a> From<Parser<'a>> for GitConfig<'a> {
    fn from(parser: Parser<'a>) -> Self {
        let mut new_self = Self::default();

        // Current section that we're building
        let mut prev_section_header = None;
        let mut section_events = SectionBody::new();

        #[allow(clippy::explicit_into_iter_loop)] // it's not really an iterator (yet), needs streaming iterator support
        for event in parser.into_iter() {
            match event {
                Event::SectionHeader(header) => {
                    if let Some(prev_header) = prev_section_header.take() {
                        new_self.push_section_internal(prev_header, section_events);
                    } else {
                        new_self.frontmatter_events = section_events;
                    }
                    prev_section_header = Some(header);
                    section_events = SectionBody::new();
                }
                e @ Event::Key(_)
                | e @ Event::Value(_)
                | e @ Event::ValueNotDone(_)
                | e @ Event::ValueDone(_)
                | e @ Event::KeyValueSeparator => section_events.as_mut().push(e),
                e @ Event::Comment(_) | e @ Event::Newline(_) | e @ Event::Whitespace(_) => {
                    section_events.as_mut().push(e);
                }
            }
        }

        // The last section doesn't get pushed since we only push if there's a
        // new section header, so we need to call push one more time.
        if let Some(header) = prev_section_header {
            new_self.push_section_internal(header, section_events);
        } else {
            new_self.frontmatter_events = section_events;
        }

        new_self
    }
}

impl From<GitConfig<'_>> for Vec<u8> {
    #[inline]
    fn from(c: GitConfig) -> Self {
        c.into()
    }
}

impl From<&GitConfig<'_>> for Vec<u8> {
    fn from(config: &GitConfig) -> Self {
        let mut value = Self::new();

        for events in config.frontmatter_events.as_ref() {
            value.extend(events.to_vec());
        }

        for section_id in &config.section_order {
            value.extend(
                config
                    .section_headers
                    .get(section_id)
                    .expect("section_header does not contain section id from section_order")
                    .to_vec(),
            );

            for event in config
                .sections
                .get(section_id)
                .expect("sections does not contain section id from section_order")
                .as_ref()
            {
                value.extend(event.to_vec());
            }
        }

        value
    }
}

impl Display for GitConfig<'_> {
    /// Note that this is a best-effort attempt at printing a `GitConfig`. If
    /// there are non UTF-8 values in your config, this will _NOT_ render as
    /// read.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for front_matter in self.frontmatter_events.as_ref() {
            front_matter.fmt(f)?;
        }

        for section_id in &self.section_order {
            self.section_headers.get(section_id).unwrap().fmt(f)?;
            for event in self.sections.get(section_id).unwrap().as_ref() {
                event.fmt(f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod mutable_value {
    use std::convert::TryFrom;

    use super::GitConfig;

    fn init_config() -> GitConfig<'static> {
        GitConfig::try_from(
            r#"[core]
                a=b"100"
            [core]
                c=d
                e=f"#,
        )
        .unwrap()
    }

    #[test]
    fn value_is_correct() {
        let mut git_config = init_config();

        let value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        assert_eq!(&*value.get().unwrap(), b"b100");
    }

    #[test]
    fn set_string_cleanly_updates() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        value.set_string("hello world".to_string());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=hello world
            [core]
                c=d
                e=f"#,
        );

        let mut value = git_config.get_raw_value_mut("core", None, "e").unwrap();
        value.set_string(String::new());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=hello world
            [core]
                c=d
                e="#,
        );
    }

    #[test]
    fn delete_value() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        value.delete();
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]
                c=d
                e=f",
        );

        let mut value = git_config.get_raw_value_mut("core", None, "c").unwrap();
        value.delete();
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]\n                \n                e=f",
        );
    }

    #[test]
    fn get_value_after_deleted() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        value.delete();
        assert!(value.get().is_err());
    }

    #[test]
    fn set_string_after_deleted() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        value.delete();
        value.set_string("hello world".to_string());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=hello world
            [core]
                c=d
                e=f"#,
        );
    }

    #[test]
    fn subsequent_delete_calls_are_noop() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        for _ in 0..10 {
            value.delete();
        }
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]
                c=d
                e=f",
        );
    }

    #[test]
    fn partial_values_are_supported() {
        let mut git_config = GitConfig::try_from(
            r#"[core]
                a=b"100"\
b
            [core]
                c=d
                e=f"#,
        )
        .unwrap();
        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        assert_eq!(&*value.get().unwrap(), b"b100b");
        value.delete();
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]
                c=d
                e=f",
        );
    }
}

#[cfg(test)]
mod mutable_multi_value {
    use std::{borrow::Cow, convert::TryFrom};

    use super::GitConfig;

    fn init_config() -> GitConfig<'static> {
        GitConfig::try_from(
            r#"[core]
                a=b"100"
            [core]
                a=d
                a=f"#,
        )
        .unwrap()
    }

    #[test]
    fn value_is_correct() {
        let mut git_config = init_config();

        let value = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        assert_eq!(
            &*value.get().unwrap(),
            vec![
                Cow::<[u8]>::Owned(b"b100".to_vec()),
                Cow::<[u8]>::Borrowed(b"d"),
                Cow::<[u8]>::Borrowed(b"f"),
            ]
        );
    }

    #[test]
    fn non_empty_sizes_are_correct() {
        let mut git_config = init_config();
        assert_eq!(git_config.get_raw_multi_value_mut("core", None, "a").unwrap().len(), 3);
        assert!(!git_config
            .get_raw_multi_value_mut("core", None, "a")
            .unwrap()
            .is_empty());
    }

    #[test]
    fn set_value_at_start() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.set_string(0, "Hello".to_string());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=Hello
            [core]
                a=d
                a=f"#,
        );
    }

    #[test]
    fn set_value_at_end() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.set_string(2, "Hello".to_string());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=b"100"
            [core]
                a=d
                a=Hello"#,
        );
    }

    #[test]
    fn set_values_all() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.set_owned_values_all(b"Hello");
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=Hello
            [core]
                a=Hello
                a=Hello"#,
        );
    }

    #[test]
    fn delete() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.delete(0);
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]
                a=d
                a=f",
        );
    }

    #[test]
    fn delete_all() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.delete_all();
        assert!(values.get().is_err());
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]\n                \n                ",
        );
    }

    #[test]
    fn partial_values_are_supported() {
        let mut git_config = GitConfig::try_from(
            r#"[core]
                a=b\
"100"
            [core]
                a=d\
b
                a=f\
a"#,
        )
        .unwrap();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();

        assert_eq!(
            &*values.get().unwrap(),
            vec![
                Cow::<[u8]>::Owned(b"b100".to_vec()),
                Cow::<[u8]>::Borrowed(b"db"),
                Cow::<[u8]>::Borrowed(b"fa"),
            ]
        );

        values.delete_all();
        assert!(values.get().is_err());
    }
}

#[cfg(test)]
mod from_paths_tests {
    use std::path::PathBuf;
    use std::{fs, io};

    use crate::file::from_paths;
    use tempfile::tempdir;

    use super::{from_paths::Error, Cow, GitConfig};
    use crate::parser::ParserOrIoError;

    /// Escapes backslash when writing a path as string so that it is a valid windows path
    fn to_backslash_escaped_string(path: &PathBuf) -> String {
        path.as_path().to_str().unwrap().replace("\\", "\\\\")
    }

    #[test]
    fn file_not_found() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config");

        let paths = vec![config_path];
        let error = GitConfig::from_paths(paths, &Default::default()).unwrap_err();
        assert!(
            matches!(error,  Error::ParserOrIoError(ParserOrIoError::Io(io_error)) if io_error.kind() == io::ErrorKind::NotFound)
        );
    }

    #[test]
    fn single_path() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config");
        fs::write(config_path.as_path(), b"[core]\nboolean = true").expect("Unable to write config file");

        let paths = vec![config_path];
        let config = GitConfig::from_paths(paths, &Default::default()).unwrap();

        assert_eq!(
            config.get_raw_value("core", None, "boolean"),
            Ok(Cow::<[u8]>::Borrowed(b"true"))
        );

        assert_eq!(config.len(), 1);
    }

    #[test]
    fn multiple_include() {
        let dir = tempdir().unwrap();

        let a_path = dir.path().join("a");
        fs::write(
            a_path.as_path(),
            "
            [core]
              a = false
              sslVerify = true
              d = 41",
        )
        .unwrap();

        let b_path = dir.path().join("b");
        let relative_b_path = std::path::PathBuf::from("b");
        fs::write(
            b_path.as_path(),
            "
            [diff]
              renames = true",
        )
        .unwrap();

        let a_path_string = to_backslash_escaped_string(&a_path.as_path().parent().unwrap().to_path_buf());
        let non_canonical_path_a = format!("{}/./a", a_path_string);
        let non_existing_path = "/dfgwfsghfdsfs";
        let c_path = dir.path().join("c");
        fs::write(
            c_path.as_path(),
            format!(
                "
            [core]
              c = 12
              d = 42
            [include]
              path = {}
              path = {}
              path = {}
            [http]
	          sslVerify = false",
                non_existing_path,
                non_canonical_path_a,
                relative_b_path.as_path().to_str().unwrap()
            ),
        )
        .unwrap();

        let config = GitConfig::from_paths(vec![c_path], &Default::default()).unwrap();

        assert_eq!(
            config.get_raw_value("core", None, "c"),
            Ok(Cow::<[u8]>::Borrowed(b"12"))
        );
        assert_eq!(
            config.get_raw_value("core", None, "d"),
            Ok(Cow::<[u8]>::Borrowed(b"41"))
        );
        assert_eq!(
            config.get_raw_value("http", None, "sslVerify"),
            Ok(Cow::<[u8]>::Borrowed(b"false"))
        );

        assert_eq!(
            config.get_raw_value("diff", None, "renames"),
            Ok(Cow::<[u8]>::Borrowed(b"true"))
        );

        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(Cow::<[u8]>::Borrowed(b"false"))
        );
    }

    #[test]
    fn nested_include_respects_max_depth() {
        let dir = tempdir().unwrap();

        // 1 includes 2
        // 2 includes 3
        // 3 includes 4
        // 4 has no includes
        let four = 4u8;
        for (i, next_i) in (1..four).zip(2..=four) {
            let path = dir.path().join(i.to_string());
            let next_path = dir.path().join(next_i.to_string());
            fs::write(
                path.as_path(),
                format!(
                    "
                    [core]
                      i = {i} 
                    [include]
                      path = {}",
                    to_backslash_escaped_string(&next_path),
                ),
            )
            .unwrap();
        }

        fs::write(
            dir.path().join("4"),
            "
                    [core]
                      i = 4",
        )
        .unwrap();

        let options = from_paths::Options::default();
        let config = GitConfig::from_paths(vec![dir.path().join("1")], &options).unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "i").unwrap(),
            vec![
                Cow::Borrowed(b"1"),
                Cow::Borrowed(b"2"),
                Cow::Borrowed(b"3"),
                Cow::Borrowed(b"4")
            ]
        );

        // with max_allowed_depth of 1 and 4 levels of includes and error_on_max_depth_exceeded: false, max_allowed_depth is exceeded and the value of level 1 is returned
        // this is equivalent to running git with --no-includes option
        let options = from_paths::Options {
            max_depth: 1,
            error_on_max_depth_exceeded: false,
            ..Default::default()
        };
        let config = GitConfig::from_paths(vec![dir.path().join("1")], &options).unwrap();
        assert_eq!(config.get_raw_value("core", None, "i"), Ok(Cow::<[u8]>::Borrowed(b"1")));

        // with default max_allowed_depth of 10 and 4 levels of includes, last level is read
        let options = from_paths::Options::default();
        let config = GitConfig::from_paths(vec![dir.path().join("1")], &options).unwrap();
        assert_eq!(config.get_raw_value("core", None, "i"), Ok(Cow::<[u8]>::Borrowed(b"4")));

        // with max_allowed_depth of 4 and 4 levels of includes, last level is read
        let options = from_paths::Options {
            max_depth: 4,
            ..Default::default()
        };
        let config = GitConfig::from_paths(vec![dir.path().join("1")], &options).unwrap();
        assert_eq!(config.get_raw_value("core", None, "i"), Ok(Cow::<[u8]>::Borrowed(b"4")));

        // with max_allowed_depth of 2 and 4 levels of includes, max_allowed_depth is exceeded and error is returned
        let options = from_paths::Options {
            max_depth: 2,
            ..Default::default()
        };
        let config = GitConfig::from_paths(vec![dir.path().join("1")], &options);
        assert!(matches!(
            config.unwrap_err(),
            from_paths::Error::IncludeDepthExceeded { max_depth: 2 }
        ));

        // with max_allowed_depth of 2 and 4 levels of includes and error_on_max_depth_exceeded: false , max_allowed_depth is exceeded and the value of level 2 is returned
        let options = from_paths::Options {
            max_depth: 2,
            error_on_max_depth_exceeded: false,
            ..Default::default()
        };
        let config = GitConfig::from_paths(vec![dir.path().join("1")], &options).unwrap();
        assert_eq!(config.get_raw_value("core", None, "i"), Ok(Cow::<[u8]>::Borrowed(b"2")));

        // with max_allowed_depth of 0 and 4 levels of includes, max_allowed_depth is exceeded and error is returned
        let options = from_paths::Options {
            max_depth: 0,
            ..Default::default()
        };
        let config = GitConfig::from_paths(vec![dir.path().join("1")], &options);
        assert!(matches!(
            config.unwrap_err(),
            from_paths::Error::IncludeDepthExceeded { max_depth: 0 }
        ));
    }

    #[test]
    fn visits_same_path_twice_when_max_depth_is_not_reached() {
        let dir = tempdir().unwrap();

        let a_path = dir.path().join("a");
        let b_path = dir.path().join("b");

        fs::write(
            a_path.as_path(),
            format!(
                "
            [core]
              b = true
            [include]
              path = {}
            [core]
              b = true
            [include]
              path = {}",
                to_backslash_escaped_string(&b_path),
                to_backslash_escaped_string(&b_path)
            ),
        )
        .unwrap();

        fs::write(
            b_path.as_path(),
            format!(
                "
            [core]
              b = false"
            ),
        )
        .unwrap();

        let config = GitConfig::from_paths(vec![a_path], &Default::default()).unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "b"),
            Ok(Cow::<[u8]>::Borrowed(b"false"))
        );
    }

    #[test]
    fn handles_cycle() {
        let dir = tempdir().unwrap();

        let a_path = dir.path().join("a");
        let b_path = dir.path().join("b");

        fs::write(
            a_path.as_path(),
            format!(
                "
            [core]
              b = 0
            [include]
              path = {}",
                to_backslash_escaped_string(&b_path),
            ),
        )
        .unwrap();

        fs::write(
            b_path.as_path(),
            format!(
                "
            [core]
              b = 1
            [include]
              path = {}",
                to_backslash_escaped_string(&a_path),
            ),
        )
        .unwrap();

        let options = from_paths::Options {
            max_depth: 4,
            ..Default::default()
        };
        let config = GitConfig::from_paths(vec![a_path.clone()], &options);
        assert!(matches!(
            config.unwrap_err(),
            from_paths::Error::IncludeDepthExceeded { max_depth: 4 }
        ));

        let options = from_paths::Options {
            max_depth: 4,
            error_on_max_depth_exceeded: false,
            ..Default::default()
        };
        let config = GitConfig::from_paths(vec![a_path], &options).unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "b").unwrap(),
            vec![
                Cow::Borrowed(b"0"),
                Cow::Borrowed(b"1"),
                Cow::Borrowed(b"0"),
                Cow::Borrowed(b"1")
            ]
        );
    }

    #[test]
    fn nested_include() {
        let dir = tempdir().unwrap();

        let a_path = dir.path().join("a");
        fs::write(
            a_path.as_path(),
            "
            [core]
              a = false
              c = 1",
        )
        .unwrap();

        let b_path = dir.path().join("b");
        fs::write(
            b_path.as_path(),
            format!(
                "
            [core]
              b = true
            [include]
              path = {}",
                to_backslash_escaped_string(&a_path)
            ),
        )
        .unwrap();

        let c_path = dir.path().join("c");
        fs::write(
            c_path.as_path(),
            format!(
                "
            [core]
              c = 12
            [include]
              path = {}",
                to_backslash_escaped_string(&b_path)
            ),
        )
        .unwrap();

        let config = GitConfig::from_paths(vec![c_path], &Default::default()).unwrap();

        assert_eq!(config.get_raw_value("core", None, "c"), Ok(Cow::<[u8]>::Borrowed(b"1")));

        assert_eq!(
            config.get_raw_value("core", None, "b"),
            Ok(Cow::<[u8]>::Borrowed(b"true"))
        );

        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(Cow::<[u8]>::Borrowed(b"false"))
        );
    }

    #[test]
    fn multiple_paths_single_value() {
        let dir = tempdir().unwrap();

        let a_path = dir.path().join("a");
        fs::write(a_path.as_path(), b"[core]\na = true").expect("Unable to write config file");

        let b_path = dir.path().join("b");
        fs::write(b_path.as_path(), b"[core]\nb = true").expect("Unable to write config file");

        let c_path = dir.path().join("c");
        fs::write(c_path.as_path(), b"[core]\nc = true").expect("Unable to write config file");

        let d_path = dir.path().join("d");
        fs::write(d_path.as_path(), b"[core]\na = false").expect("Unable to write config file");

        let paths = vec![a_path, b_path, c_path, d_path];
        let config = GitConfig::from_paths(paths, &Default::default()).unwrap();

        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(Cow::<[u8]>::Borrowed(b"false"))
        );

        assert_eq!(
            config.get_raw_value("core", None, "b"),
            Ok(Cow::<[u8]>::Borrowed(b"true"))
        );

        assert_eq!(
            config.get_raw_value("core", None, "c"),
            Ok(Cow::<[u8]>::Borrowed(b"true"))
        );

        assert_eq!(config.len(), 4);
    }

    #[test]
    fn multiple_paths_multi_value() {
        let dir = tempdir().unwrap();

        let a_path = dir.path().join("a");
        fs::write(a_path.as_path(), b"[core]\nkey = a").expect("Unable to write config file");

        let b_path = dir.path().join("b");
        fs::write(b_path.as_path(), b"[core]\nkey = b").expect("Unable to write config file");

        let c_path = dir.path().join("c");
        fs::write(c_path.as_path(), b"[core]\nkey = c").expect("Unable to write config file");

        let d_path = dir.path().join("d");
        fs::write(d_path.as_path(), b"[include]\npath = d_path").expect("Unable to write config file");

        let e_path = dir.path().join("e");
        fs::write(e_path.as_path(), b"[include]\npath = e_path").expect("Unable to write config file");

        let paths = vec![a_path, b_path, c_path, d_path, e_path];
        let config = GitConfig::from_paths(paths, &Default::default()).unwrap();

        assert_eq!(
            config.get_raw_multi_value("core", None, "key").unwrap(),
            vec![Cow::Borrowed(b"a"), Cow::Borrowed(b"b"), Cow::Borrowed(b"c")]
        );

        assert_eq!(
            config.get_raw_multi_value("include", None, "path").unwrap(),
            vec![Cow::Borrowed(b"d_path"), Cow::Borrowed(b"e_path")]
        );

        assert_eq!(config.len(), 5);
    }
}

#[cfg(test)]
pub mod from_env_paths {}

#[cfg(test)]
mod from_env_tests {
    use std::{env, fs};

    use crate::file::from_paths::Options;
    use serial_test::serial;
    use tempfile::tempdir;

    use super::{from_env, Cow, GitConfig};

    struct Env<'a> {
        altered_vars: Vec<&'a str>,
    }

    impl<'a> Env<'a> {
        fn new() -> Self {
            Env {
                altered_vars: Vec::new(),
            }
        }

        fn set(mut self, var: &'a str, value: &'a str) -> Self {
            env::set_var(var, value);
            self.altered_vars.push(var);
            self
        }
    }

    impl<'a> Drop for Env<'a> {
        fn drop(&mut self) {
            for var in &self.altered_vars {
                env::remove_var(var);
            }
        }
    }

    #[test]
    #[serial]
    fn empty_without_relevant_environment() {
        let config = GitConfig::from_env(&Options::default()).unwrap();
        assert!(config.is_none());
    }

    #[test]
    #[serial]
    fn empty_with_zero_count() {
        let _env = Env::new().set("GIT_CONFIG_COUNT", "0");
        let config = GitConfig::from_env(&Options::default()).unwrap();
        assert!(config.is_none());
    }

    #[test]
    #[serial]
    fn parse_error_with_invalid_count() {
        let _env = Env::new().set("GIT_CONFIG_COUNT", "invalid");
        let err = GitConfig::from_env(&Options::default()).unwrap_err();
        assert!(matches!(err, from_env::Error::ParseError(_)));
    }

    #[test]
    #[serial]
    fn single_key_value_pair() {
        let _env = Env::new()
            .set("GIT_CONFIG_COUNT", "1")
            .set("GIT_CONFIG_KEY_0", "core.key")
            .set("GIT_CONFIG_VALUE_0", "value");

        let config = GitConfig::from_env(&Options::default()).unwrap().unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "key"),
            Ok(Cow::<[u8]>::Borrowed(b"value"))
        );

        assert_eq!(config.len(), 1);
    }

    #[test]
    #[serial]
    fn multiple_key_value_pairs() {
        let _env = Env::new()
            .set("GIT_CONFIG_COUNT", "3")
            .set("GIT_CONFIG_KEY_0", "core.a")
            .set("GIT_CONFIG_VALUE_0", "a")
            .set("GIT_CONFIG_KEY_1", "core.b")
            .set("GIT_CONFIG_VALUE_1", "b")
            .set("GIT_CONFIG_KEY_2", "core.c")
            .set("GIT_CONFIG_VALUE_2", "c");

        let config = GitConfig::from_env(&Options::default()).unwrap().unwrap();

        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"a")));
        assert_eq!(config.get_raw_value("core", None, "b"), Ok(Cow::<[u8]>::Borrowed(b"b")));
        assert_eq!(config.get_raw_value("core", None, "c"), Ok(Cow::<[u8]>::Borrowed(b"c")));
        assert_eq!(config.len(), 3);
    }

    #[test]
    #[serial]
    fn follows_includes() {
        let dir = tempdir().unwrap();
        let a_path = dir.path().join("a");
        fs::write(&a_path, "[core]\nkey = changed").unwrap();
        let path_str = a_path.to_str().unwrap();

        let _env = Env::new()
            .set("GIT_CONFIG_COUNT", "2")
            .set("GIT_CONFIG_KEY_0", "core.key")
            .set("GIT_CONFIG_VALUE_0", "value")
            .set("GIT_CONFIG_KEY_1", "include.path")
            .set("GIT_CONFIG_VALUE_1", path_str);

        let config = GitConfig::from_env(&Options::default()).unwrap().unwrap();

        assert_eq!(
            config.get_raw_value("core", None, "key"),
            Ok(Cow::<[u8]>::Borrowed(b"changed"))
        );
        assert_eq!(config.len(), 3);
    }
}

#[cfg(test)]
mod from_parser {
    use super::{Cow, Event, GitConfig, HashMap, LookupTreeNode, SectionBody, SectionId, TryFrom};
    use crate::{
        parser::SectionHeaderName,
        test_util::{name_event, newline_event, section_header, value_event},
    };

    #[test]
    fn parse_empty() {
        let config = GitConfig::try_from("").unwrap();
        assert!(config.section_headers.is_empty());
        assert_eq!(config.section_id_counter, 0);
        assert!(config.section_lookup_tree.is_empty());
        assert!(config.sections.is_empty());
        assert!(config.section_order.is_empty());
    }

    #[test]
    fn parse_single_section() {
        let mut config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), section_header("core", None));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 1);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            tree.insert(
                SectionHeaderName(Cow::Borrowed("core")),
                vec![LookupTreeNode::Terminal(vec![SectionId(0)])],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionId(0),
                SectionBody::from(vec![
                    newline_event(),
                    name_event("a"),
                    Event::KeyValueSeparator,
                    value_event("b"),
                    newline_event(),
                    name_event("c"),
                    Event::KeyValueSeparator,
                    value_event("d"),
                ]),
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(config.section_order.make_contiguous(), &[SectionId(0)]);
    }

    #[test]
    fn parse_single_subsection() {
        let mut config = GitConfig::try_from("[core.sub]\na=b\nc=d").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), section_header("core", (".", "sub")));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 1);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            let mut inner_tree = HashMap::new();
            inner_tree.insert(Cow::Borrowed("sub"), vec![SectionId(0)]);
            tree.insert(
                SectionHeaderName(Cow::Borrowed("core")),
                vec![LookupTreeNode::NonTerminal(inner_tree)],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionId(0),
                SectionBody::from(vec![
                    newline_event(),
                    name_event("a"),
                    Event::KeyValueSeparator,
                    value_event("b"),
                    newline_event(),
                    name_event("c"),
                    Event::KeyValueSeparator,
                    value_event("d"),
                ]),
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(config.section_order.make_contiguous(), &[SectionId(0)]);
    }

    #[test]
    fn parse_multiple_sections() {
        let mut config = GitConfig::try_from("[core]\na=b\nc=d\n[other]e=f").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), section_header("core", None));
            map.insert(SectionId(1), section_header("other", None));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 2);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            tree.insert(
                SectionHeaderName(Cow::Borrowed("core")),
                vec![LookupTreeNode::Terminal(vec![SectionId(0)])],
            );
            tree.insert(
                SectionHeaderName(Cow::Borrowed("other")),
                vec![LookupTreeNode::Terminal(vec![SectionId(1)])],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionId(0),
                SectionBody::from(vec![
                    newline_event(),
                    name_event("a"),
                    Event::KeyValueSeparator,
                    value_event("b"),
                    newline_event(),
                    name_event("c"),
                    Event::KeyValueSeparator,
                    value_event("d"),
                    newline_event(),
                ]),
            );
            sections.insert(
                SectionId(1),
                SectionBody::from(vec![name_event("e"), Event::KeyValueSeparator, value_event("f")]),
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(config.section_order.make_contiguous(), &[SectionId(0), SectionId(1)]);
    }

    #[test]
    fn parse_multiple_duplicate_sections() {
        let mut config = GitConfig::try_from("[core]\na=b\nc=d\n[core]e=f").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), section_header("core", None));
            map.insert(SectionId(1), section_header("core", None));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 2);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            tree.insert(
                SectionHeaderName(Cow::Borrowed("core")),
                vec![LookupTreeNode::Terminal(vec![SectionId(0), SectionId(1)])],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionId(0),
                SectionBody::from(vec![
                    newline_event(),
                    name_event("a"),
                    Event::KeyValueSeparator,
                    value_event("b"),
                    newline_event(),
                    name_event("c"),
                    Event::KeyValueSeparator,
                    value_event("d"),
                    newline_event(),
                ]),
            );
            sections.insert(
                SectionId(1),
                SectionBody::from(vec![name_event("e"), Event::KeyValueSeparator, value_event("f")]),
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(config.section_order.make_contiguous(), &[SectionId(0), SectionId(1)]);
    }
}

#[cfg(test)]
mod get_raw_value {
    use super::{Cow, GitConfig, GitConfigError, TryFrom};
    use crate::parser::SectionHeaderName;

    #[test]
    fn single_section() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"b")));
        assert_eq!(config.get_raw_value("core", None, "c"), Ok(Cow::<[u8]>::Borrowed(b"d")));
    }

    #[test]
    fn last_one_wins_respected_in_section() {
        let config = GitConfig::try_from("[core]\na=b\na=d").unwrap();
        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"d")));
    }

    #[test]
    fn last_one_wins_respected_across_section() {
        let config = GitConfig::try_from("[core]\na=b\n[core]\na=d").unwrap();
        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"d")));
    }

    #[test]
    fn section_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("foo", None, "a"),
            Err(GitConfigError::SectionDoesNotExist(SectionHeaderName("foo".into())))
        );
    }

    #[test]
    fn subsection_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", Some("a"), "a"),
            Err(GitConfigError::SubSectionDoesNotExist(Some("a")))
        );
    }

    #[test]
    fn key_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "aaaaaa"),
            Err(GitConfigError::KeyDoesNotExist)
        );
    }

    #[test]
    fn subsection_must_be_respected() {
        let config = GitConfig::try_from("[core]a=b\n[core.a]a=c").unwrap();
        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"b")));
        assert_eq!(
            config.get_raw_value("core", Some("a"), "a"),
            Ok(Cow::<[u8]>::Borrowed(b"c"))
        );
    }
}

#[cfg(test)]
mod get_value {
    use std::error::Error;

    use super::{Cow, GitConfig, TryFrom};
    use crate::values::{Boolean, Bytes, TrueVariant};

    #[test]
    fn single_section() -> Result<(), Box<dyn Error>> {
        let config = GitConfig::try_from("[core]\na=b\nc").unwrap();
        let first_value: Bytes = config.value("core", None, "a")?;
        let second_value: Boolean = config.value("core", None, "c")?;

        assert_eq!(
            first_value,
            Bytes {
                value: Cow::Borrowed(b"b")
            }
        );
        assert_eq!(second_value, Boolean::True(TrueVariant::Implicit));

        Ok(())
    }

    #[test]
    fn sections_by_name() {
        let config = r#"
        [core]
            repositoryformatversion = 0
            filemode = true
            bare = false
            logallrefupdates = true
        [remote "origin"]
            url = git@github.com:Byron/gitoxide.git
            fetch = +refs/heads/*:refs/remotes/origin/*
        "#;

        let config = GitConfig::try_from(config).unwrap();
        let value = config.value::<Bytes>("remote", Some("origin"), "url").unwrap();
        assert_eq!(
            value,
            Bytes {
                value: Cow::Borrowed(b"git@github.com:Byron/gitoxide.git")
            }
        );
    }
}

#[cfg(test)]
mod get_raw_multi_value {
    use super::{Cow, GitConfig, GitConfigError, TryFrom};
    use crate::parser::SectionHeaderName;

    #[test]
    fn single_value_is_identical_to_single_value_query() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            vec![config.get_raw_value("core", None, "a").unwrap()],
            config.get_raw_multi_value("core", None, "a").unwrap()
        );
    }

    #[test]
    fn multi_value_in_section() {
        let config = GitConfig::try_from("[core]\na=b\na=c").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![Cow::Borrowed(b"b"), Cow::Borrowed(b"c")]
        );
    }

    #[test]
    fn multi_value_across_sections() {
        let config = GitConfig::try_from("[core]\na=b\na=c\n[core]a=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![Cow::Borrowed(b"b"), Cow::Borrowed(b"c"), Cow::Borrowed(b"d")]
        );
    }

    #[test]
    fn section_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("foo", None, "a"),
            Err(GitConfigError::SectionDoesNotExist(SectionHeaderName("foo".into())))
        );
    }

    #[test]
    fn subsection_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", Some("a"), "a"),
            Err(GitConfigError::SubSectionDoesNotExist(Some("a")))
        );
    }

    #[test]
    fn key_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "aaaaaa"),
            Err(GitConfigError::KeyDoesNotExist)
        );
    }

    #[test]
    fn subsection_must_be_respected() {
        let config = GitConfig::try_from("[core]a=b\n[core.a]a=c").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![Cow::Borrowed(b"b")]
        );
        assert_eq!(
            config.get_raw_multi_value("core", Some("a"), "a").unwrap(),
            vec![Cow::Borrowed(b"c")]
        );
    }

    #[test]
    fn non_relevant_subsection_is_ignored() {
        let config = GitConfig::try_from("[core]\na=b\na=c\n[core]a=d\n[core]g=g").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![Cow::Borrowed(b"b"), Cow::Borrowed(b"c"), Cow::Borrowed(b"d")]
        );
    }
}

#[cfg(test)]
mod display {
    use super::{GitConfig, TryFrom};

    #[test]
    fn can_reconstruct_empty_config() {
        let config = r#"

        "#;
        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }

    #[test]
    fn can_reconstruct_non_empty_config() {
        let config = r#"
            [user]
                email = code@eddie.sh
            [core]
                autocrlf = input
            [push]
                default = simple
            [commit]
                gpgsign = true
            [gpg]
                program = gpg
            [url "ssh://git@github.com/"]
                insteadOf = "github://"
            [url "ssh://git@git.eddie.sh/edward/"]
                insteadOf = "gitea://"
            [pull]
                ff = only
            [init]
                defaultBranch = master
        "#;

        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }

    #[test]
    fn can_reconstruct_configs_with_implicits() {
        let config = r#"
            [user]
                email
                name
            [core]
                autocrlf
            [push]
                default
            [commit]
                gpgsign
        "#;

        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }

    #[test]
    fn can_reconstruct_configs_without_whitespace_in_middle() {
        let config = r#"
            [core]
                autocrlf=input
            [push]
                default=simple
            [commit]
                gpgsign=true
            [pull]
                ff = only
            [init]
                defaultBranch = master
        "#;

        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }
}
