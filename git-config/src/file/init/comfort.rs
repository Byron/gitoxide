use crate::file::{init, Metadata};
use crate::{source, File};
use std::path::PathBuf;

/// easy-instantiation of typical git configuration files with all configuration defaulting to typical values.
impl File<'static> {
    /// Open all global configuration files which involves the following sources:
    ///
    /// * [system][Source::System]
    /// * [git][Source::Git]
    /// * [user][Source::User]
    ///
    /// which excludes repository local configuration, as well as override-configuration from environment variables.
    ///
    /// Note that `includeIf` conditions in global files will cause failure as the required information
    /// to resolve them isn't present without a repository.
    ///
    /// Also note that relevant information to interpolate paths will be obtained from the environment or other
    /// source on unix.
    ///
    ///
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
        File::from_paths_metadata(metas, options)
    }
}
