use crate::bstr::BStr;
use crate::{config, dirwalk, Repository};
use std::path::Path;

/// The error returned by [dirwalk()](Repository::dirwalk()).
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Walk(#[from] gix_dir::walk::Error),
    #[error("A working tree is required to perform a directory walk")]
    MissinWorkDir,
    #[error(transparent)]
    Excludes(#[from] config::exclude_stack::Error),
    #[error(transparent)]
    Pathspec(#[from] crate::pathspec::init::Error),
    #[error(transparent)]
    Prefix(#[from] gix_path::realpath::Error),
    #[error(transparent)]
    FilesystemOptions(#[from] config::boolean::Error),
}

impl Repository {
    /// Return default options suitable for performing a directory walk on this repository.
    ///
    /// Used in conjunction with [`dirwalk()`](Self::dirwalk())
    pub fn dirwalk_options(&self) -> Result<dirwalk::Options, config::boolean::Error> {
        Ok(dirwalk::Options::from_fs_caps(self.filesystem_options()?))
    }

    /// Perform a directory walk configured with `options` under control of the `delegate`. Use `patterns` to
    /// further filter entries.
    ///
    /// The `index` is used to determine if entries are tracked, and for excludes and attributes
    /// lookup. Note that items will only count as tracked if they have the [`gix_index::entry::Flags::UPTODATE`]
    /// flag set.
    ///
    /// See [`gix_dir::walk::delegate::Collect`] for a delegate that collects all seen entries.
    pub fn dirwalk(
        &self,
        index: &gix_index::State,
        patterns: impl IntoIterator<Item = impl AsRef<BStr>>,
        options: dirwalk::Options,
        delegate: &mut dyn gix_dir::walk::Delegate,
    ) -> Result<gix_dir::walk::Outcome, Error> {
        let workdir = self.work_dir().ok_or(Error::MissinWorkDir)?;
        let mut excludes = self
            .excludes(
                index,
                None,
                crate::worktree::stack::state::ignore::Source::WorktreeThenIdMappingIfNotSkipped,
            )?
            .detach();
        let (mut pathspec, mut maybe_attributes) = self
            .pathspec(
                patterns,
                true, /* inherit ignore case */
                index,
                crate::worktree::stack::state::attributes::Source::WorktreeThenIdMapping,
            )?
            .into_parts();

        let prefix = self.prefix()?.unwrap_or(Path::new(""));
        let git_dir_realpath =
            crate::path::realpath_opts(self.git_dir(), self.current_dir(), crate::path::realpath::MAX_SYMLINKS)?;
        let fs_caps = self.filesystem_options()?;
        let accelerate_lookup = fs_caps.ignore_case.then(|| index.prepare_icase_backing());
        gix_dir::walk(
            &workdir.join(prefix),
            workdir,
            gix_dir::walk::Context {
                git_dir_realpath: git_dir_realpath.as_ref(),
                current_dir: self.current_dir(),
                index,
                ignore_case_index_lookup: accelerate_lookup.as_ref(),
                pathspec: &mut pathspec,
                pathspec_attributes: &mut |relative_path, case, is_dir, out| {
                    let stack = maybe_attributes
                        .as_mut()
                        .expect("can only be called if attributes are used in patterns");
                    stack
                        .set_case(case)
                        .at_entry(relative_path, Some(is_dir), &self.objects)
                        .map_or(false, |platform| platform.matching_attributes(out))
                },
                excludes: Some(&mut excludes),
                objects: &self.objects,
            },
            options.into(),
            delegate,
        )
        .map_err(Into::into)
    }
}
