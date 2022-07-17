use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use bstr::{BStr, BString, ByteSlice, ByteVec};
use git_features::threading::OwnShared;
use git_ref::Category;

use crate::file::{init, Metadata};
use crate::{file, file::init::from_paths, File};

pub(crate) fn resolve(
    config: &mut File<'static>,
    meta: OwnShared<Metadata>,
    buf: &mut Vec<u8>,
    options: Options<'_>,
) -> Result<(), Error> {
    resolve_includes_recursive(config, meta, 0, buf, options)
}

fn resolve_includes_recursive(
    target_config: &mut File<'static>,
    meta: OwnShared<Metadata>,
    depth: u8,
    buf: &mut Vec<u8>,
    options: Options<'_>,
) -> Result<(), Error> {
    if depth == options.max_depth {
        return if options.error_on_max_depth_exceeded {
            Err(Error::IncludeDepthExceeded {
                max_depth: options.max_depth,
            })
        } else {
            Ok(())
        };
    }

    let target_config_path = meta.path.as_deref();

    let mut include_paths = Vec::new();
    for section in target_config.sections() {
        let header = &section.header;
        let header_name = header.name.as_ref();
        if header_name == "include" && header.subsection_name.is_none() {
            detach_include_paths(&mut include_paths, section)
        } else if header_name == "includeIf" {
            if let Some(condition) = &header.subsection_name {
                if include_condition_match(condition.as_ref(), target_config_path, options)? {
                    detach_include_paths(&mut include_paths, section)
                }
            }
        }
    }

    append_followed_includes_recursively(
        include_paths,
        target_config,
        target_config_path,
        depth,
        meta.clone(),
        options,
        buf,
    )
}

fn append_followed_includes_recursively(
    include_paths: Vec<crate::Path<'_>>,
    target_config: &mut File<'static>,
    target_config_path: Option<&Path>,
    depth: u8,
    meta: OwnShared<Metadata>,
    options: Options<'_>,
    buf: &mut Vec<u8>,
) -> Result<(), Error> {
    for config_path in include_paths {
        let config_path = resolve_path(config_path, target_config_path, options)?;
        if !config_path.is_file() {
            continue;
        }

        let config_meta = Metadata {
            path: None,
            trust: meta.trust,
            level: meta.level + 1,
            source: meta.source,
        };

        let no_follow_options = init::Options::default();
        let mut include_config =
            File::from_path_with_buf(config_path, buf, config_meta, no_follow_options).map_err(|err| match err {
                from_paths::Error::Io(err) => Error::Io(err),
                from_paths::Error::Init(init::Error::Parse(err)) => Error::Parse(err),
                err => unreachable!("BUG: {:?} shouldn't be possible here", err),
            })?;
        let config_meta = include_config.meta_owned();

        resolve_includes_recursive(&mut include_config, config_meta, depth + 1, buf, options)?;

        target_config.append(include_config);
    }
    Ok(())
}

fn detach_include_paths(include_paths: &mut Vec<crate::Path<'static>>, section: &file::Section<'_>) {
    include_paths.extend(
        section
            .body
            .values("path")
            .into_iter()
            .map(|path| crate::Path::from(Cow::Owned(path.into_owned()))),
    )
}

fn include_condition_match(
    condition: &BStr,
    target_config_path: Option<&Path>,
    options: Options<'_>,
) -> Result<bool, Error> {
    let mut tokens = condition.splitn(2, |b| *b == b':');
    let (prefix, condition) = match (tokens.next(), tokens.next()) {
        (Some(a), Some(b)) => (a, b),
        _ => return Ok(false),
    };
    let condition = condition.as_bstr();
    match prefix {
        b"gitdir" => gitdir_matches(
            condition,
            target_config_path,
            options,
            git_glob::wildmatch::Mode::empty(),
        ),
        b"gitdir/i" => gitdir_matches(
            condition,
            target_config_path,
            options,
            git_glob::wildmatch::Mode::IGNORE_CASE,
        ),
        b"onbranch" => Ok(onbranch_matches(condition, options.conditional).is_some()),
        _ => Ok(false),
    }
}

fn onbranch_matches(
    condition: &BStr,
    conditional::Context { branch_name, .. }: conditional::Context<'_>,
) -> Option<()> {
    let branch_name = branch_name?;
    let (_, branch_name) = branch_name
        .category_and_short_name()
        .filter(|(cat, _)| *cat == Category::LocalBranch)?;

    let condition = if condition.ends_with(b"/") {
        let mut condition: BString = condition.into();
        condition.push_str("**");
        Cow::Owned(condition)
    } else {
        condition.into()
    };

    git_glob::wildmatch(
        condition.as_ref(),
        branch_name,
        git_glob::wildmatch::Mode::NO_MATCH_SLASH_LITERAL,
    )
    .then(|| ())
}

fn gitdir_matches(
    condition_path: &BStr,
    target_config_path: Option<&Path>,
    Options {
        conditional: conditional::Context { git_dir, .. },
        interpolate: context,
        ..
    }: Options<'_>,
    wildmatch_mode: git_glob::wildmatch::Mode,
) -> Result<bool, Error> {
    let git_dir = git_path::to_unix_separators_on_windows(git_path::into_bstr(git_dir.ok_or(Error::MissingGitDir)?));

    let mut pattern_path: Cow<'_, _> = {
        let path = crate::Path::from(Cow::Borrowed(condition_path)).interpolate(context)?;
        git_path::into_bstr(path).into_owned().into()
    };
    // NOTE: yes, only if we do path interpolation will the slashes be forced to unix separators on windows
    if pattern_path != condition_path {
        pattern_path = git_path::to_unix_separators_on_windows(pattern_path);
    }

    if let Some(relative_pattern_path) = pattern_path.strip_prefix(b"./") {
        let parent_dir = target_config_path
            .ok_or(Error::MissingConfigPath)?
            .parent()
            .expect("config path can never be /");
        let mut joined_path = git_path::to_unix_separators_on_windows(git_path::into_bstr(parent_dir)).into_owned();
        joined_path.push(b'/');
        joined_path.extend_from_slice(relative_pattern_path);
        pattern_path = joined_path.into();
    }

    // NOTE: this special handling of leading backslash is needed to do it like git does
    if pattern_path.iter().next() != Some(&(std::path::MAIN_SEPARATOR as u8))
        && !git_path::from_bstr(pattern_path.clone()).is_absolute()
    {
        let mut prefixed = pattern_path.into_owned();
        prefixed.insert_str(0, "**/");
        pattern_path = prefixed.into()
    }
    if pattern_path.ends_with(b"/") {
        let mut suffixed = pattern_path.into_owned();
        suffixed.push_str("**");
        pattern_path = suffixed.into();
    }

    let match_mode = git_glob::wildmatch::Mode::NO_MATCH_SLASH_LITERAL | wildmatch_mode;
    let is_match = git_glob::wildmatch(pattern_path.as_bstr(), git_dir.as_bstr(), match_mode);
    if is_match {
        return Ok(true);
    }

    let expanded_git_dir = git_path::into_bstr(git_path::realpath(git_path::from_byte_slice(&git_dir))?);
    Ok(git_glob::wildmatch(
        pattern_path.as_bstr(),
        expanded_git_dir.as_bstr(),
        match_mode,
    ))
}

fn resolve_path(
    path: crate::Path<'_>,
    target_config_path: Option<&Path>,
    Options {
        interpolate: context, ..
    }: Options<'_>,
) -> Result<PathBuf, Error> {
    let path = path.interpolate(context)?;
    let path: PathBuf = if path.is_relative() {
        target_config_path
            .ok_or(Error::MissingConfigPath)?
            .parent()
            .expect("path is a config file which naturally lives in a directory")
            .join(path)
    } else {
        path.into()
    };
    Ok(path)
}

mod types {
    use crate::parse;
    use crate::path::interpolate;

    /// The error returned by [`File::from_paths_metadata()`] and [`File::from_env_paths()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Io(#[from] std::io::Error),
        #[error(transparent)]
        Parse(#[from] parse::Error),
        #[error(transparent)]
        Interpolate(#[from] interpolate::Error),
        #[error("The maximum allowed length {} of the file include chain built by following nested resolve_includes is exceeded", .max_depth)]
        IncludeDepthExceeded { max_depth: u8 },
        #[error(
            "Include paths from environment variables must not be relative as no config file paths exists as root"
        )]
        MissingConfigPath,
        #[error("The git directory must be provided to support `gitdir:` conditional includes")]
        MissingGitDir,
        #[error(transparent)]
        Realpath(#[from] git_path::realpath::Error),
    }

    /// Options to handle includes, like `include.path` or `includeIf.<condition>.path`,
    #[derive(Clone, Copy)]
    pub struct Options<'a> {
        /// The maximum allowed length of the file include chain built by following nested resolve_includes where base level is depth = 0.
        pub max_depth: u8,
        /// When max depth is exceeded while following nested includes,
        /// return an error if true or silently stop following resolve_includes.
        ///
        /// Setting this value to false allows to read configuration with cycles,
        /// which otherwise always results in an error.
        pub error_on_max_depth_exceeded: bool,

        /// Used during path interpolation, both for include paths before trying to read the file, and for
        /// paths used in conditional `gitdir` includes.
        pub interpolate: crate::path::interpolate::Context<'a>,

        /// Additional context for conditional includes to work.
        pub conditional: conditional::Context<'a>,
    }

    impl Options<'_> {
        /// Provide options to never follow include directives at all.
        pub fn no_follow() -> Self {
            Options {
                max_depth: 0,
                error_on_max_depth_exceeded: false,
                interpolate: Default::default(),
                conditional: Default::default(),
            }
        }
    }

    impl<'a> Options<'a> {
        /// Provide options to follow includes like git does, provided the required `conditional` and `interpolate` contexts
        /// to support `gitdir` and `onbranch` based `includeIf` directives as well as standard `include.path` resolution.
        /// Note that the follow-mode is `git`-style, following at most 10 indirections while
        /// producing an error if the depth is exceeded.
        pub fn follow(
            interpolate: crate::path::interpolate::Context<'a>,
            conditional: conditional::Context<'a>,
        ) -> Self {
            Options {
                max_depth: 10,
                error_on_max_depth_exceeded: true,
                interpolate,
                conditional,
            }
        }

        /// Set the context used for interpolation when interpolating paths to include as well as the paths
        /// in `gitdir` conditional includes.
        pub fn interpolate_with(mut self, context: crate::path::interpolate::Context<'a>) -> Self {
            self.interpolate = context;
            self
        }
    }

    impl Default for Options<'_> {
        fn default() -> Self {
            Self::no_follow()
        }
    }

    ///
    pub mod conditional {
        /// Options to handle conditional includes like `includeIf.<condition>.path`.
        #[derive(Clone, Copy, Default)]
        pub struct Context<'a> {
            /// The location of the .git directory. If `None`, `gitdir` conditions cause an error.
            ///
            /// Used for conditional includes, e.g. `includeIf.gitdir:…` or `includeIf:gitdir/i…`.
            pub git_dir: Option<&'a std::path::Path>,
            /// The name of the branch that is currently checked out. If `None`, `onbranch` conditions cause an error.
            ///
            /// Used for conditional includes, e.g. `includeIf.onbranch:main.…`
            pub branch_name: Option<&'a git_ref::FullNameRef>,
        }
    }
}
pub use types::{conditional, Error, Options};
