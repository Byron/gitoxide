use crate::file::{init, Metadata};
use crate::{source, File};
use std::path::PathBuf;

/// Easy-instantiation of typical non-repository git configuration files with all configuration defaulting to typical values.
///
/// ### Limitations
///
/// Note that `includeIf` conditions in global files will cause failure as the required information
/// to resolve them isn't present without a repository.
///
/// Also note that relevant information to interpolate paths will be obtained from the environment or other
/// source on unix.
impl File<'static> {
    /// Open all global configuration files which involves the following sources:
    ///
    /// * [system][crate::Source::System]
    /// * [git][crate::Source::Git]
    /// * [user][crate::Source::User]
    ///
    /// which excludes repository local configuration, as well as override-configuration from environment variables.
    ///
    /// Note that the file might [be empty][File::is_void()] in case no configuration file was found.
    pub fn new_globals() -> Result<File<'static>, init::from_paths::Error> {
        let metas = [source::Kind::System, source::Kind::Global]
            .iter()
            .flat_map(|kind| kind.sources())
            .filter_map(|source| {
                let path = source
                    .storage_location(&mut |name| std::env::var_os(name))
                    .and_then(|p| p.is_file().then(|| p))
                    .map(|p| p.into_owned());

                Metadata {
                    path,
                    source: *source,
                    level: 0,
                    trust: git_sec::Trust::Full,
                }
                .into()
            });

        let home = std::env::var("HOME").ok().map(PathBuf::from);
        let options = init::Options {
            includes: init::includes::Options::follow_without_conditional(home.as_deref()),
            ..Default::default()
        };
        File::from_paths_metadata(metas, options).map(Option::unwrap_or_default)
    }

    /// Generates a config from `GIT_CONFIG_*` environment variables and return a possibly empty `File`.
    /// A typical use of this is to [`append`][File::append()] this configuration to another one with lower
    /// precedence to obtain overrides.
    ///
    /// See [`git-config`'s documentation] for more information on the environment variables in question.
    ///
    /// [`git-config`'s documentation]: https://git-scm.com/docs/git-config#Documentation/git-config.txt-GITCONFIGCOUNT
    pub fn new_environment_overrides() -> Result<File<'static>, init::from_env::Error> {
        let home = std::env::var("HOME").ok().map(PathBuf::from);
        let options = init::Options {
            includes: init::includes::Options::follow_without_conditional(home.as_deref()),
            ..Default::default()
        };

        File::from_environment(options).map(Option::unwrap_or_default)
    }
}

/// An easy way to provide complete configuration for a repository.
impl File<'static> {
    /// TODO
    pub fn from_git_dir(_dir: impl AsRef<std::path::Path>) -> Result<File<'static>, init::from_paths::Error> {
        todo!()
    }
}
