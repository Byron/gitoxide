use super::{util, Error};

/// A utility to deal with the cyclic dependency between the ref store and the configuration. The ref-store needs the
/// object hash kind, and the configuration needs the current branch name to resolve conditional includes with `onbranch`.
pub(crate) struct StageOne {
    pub git_dir_config: git_config::File<'static>,
    pub buf: Vec<u8>,

    pub is_bare: bool,
    pub lossy: Option<bool>,
    pub object_hash: git_hash::Kind,
    pub reflog: Option<git_ref::store::WriteReflog>,
}

/// Initialization
impl StageOne {
    pub fn new(
        common_dir: &std::path::Path,
        git_dir: &std::path::Path,
        git_dir_trust: git_sec::Trust,
        lossy: Option<bool>,
        lenient: bool,
    ) -> Result<Self, Error> {
        let mut buf = Vec::with_capacity(512);
        let mut config = load_config(
            common_dir.join("config"),
            &mut buf,
            git_config::Source::Local,
            git_dir_trust,
            lossy,
        )?;

        let is_bare = util::config_bool(&config, "core.bare", false, lenient)?;
        let repo_format_version = config
            .value::<git_config::Integer>("core", None, "repositoryFormatVersion")
            .map_or(0, |v| v.to_decimal().unwrap_or_default());
        let object_hash = (repo_format_version != 1)
            .then(|| Ok(git_hash::Kind::Sha1))
            .or_else(|| {
                config.string("extensions", None, "objectFormat").map(|format| {
                    if format.as_ref().eq_ignore_ascii_case(b"sha1") {
                        Ok(git_hash::Kind::Sha1)
                    } else {
                        Err(Error::UnsupportedObjectFormat {
                            name: format.to_vec().into(),
                        })
                    }
                })
            })
            .transpose()?
            .unwrap_or(git_hash::Kind::Sha1);

        let extension_worktree = util::config_bool(&config, "extensions.worktreeConfig", false, lenient)?;
        if extension_worktree {
            let worktree_config = load_config(
                git_dir.join("config.worktree"),
                &mut buf,
                git_config::Source::Worktree,
                git_dir_trust,
                lossy,
            )?;
            config.append(worktree_config);
        };

        let reflog = util::query_refupdates(&config, lenient)?;
        Ok(StageOne {
            git_dir_config: config,
            buf,
            is_bare,
            lossy,
            object_hash,
            reflog,
        })
    }
}

fn load_config(
    config_path: std::path::PathBuf,
    buf: &mut Vec<u8>,
    source: git_config::Source,
    git_dir_trust: git_sec::Trust,
    lossy: Option<bool>,
) -> Result<git_config::File<'static>, Error> {
    buf.clear();
    std::io::copy(&mut std::fs::File::open(&config_path)?, buf)?;

    let config = git_config::File::from_bytes_owned(
        buf,
        git_config::file::Metadata::from(source)
            .at(config_path)
            .with(git_dir_trust),
        git_config::file::init::Options {
            includes: git_config::file::includes::Options::no_follow(),
            ..util::base_options(lossy)
        },
    )?;

    Ok(config)
}
