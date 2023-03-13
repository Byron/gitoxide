use std::{borrow::Cow, path::PathBuf};

use crate::{
    bstr::ByteSlice,
    config::tree::{gitoxide, Key},
    Repository,
};

impl Repository {
    /// Return `true` if the repository is a shallow clone, i.e. contains history only up to a certain depth.
    pub fn is_shallow(&self) -> bool {
        self.shallow_file()
            .metadata()
            .map_or(false, |m| m.is_file() && m.len() > 0)
    }

    /// Return a shared list of shallow commits which is updated automatically if the in-memory snapshot has become stale
    /// as the underlying file on disk has changed.
    ///
    /// The list of shallow commits represents the shallow boundary, beyond which we are lacking all (parent) commits.
    /// Note that the list is never empty, as `Ok(None)` is returned in that case indicating the repository
    /// isn't a shallow clone.
    ///
    /// The shared list is shared across all clones of this repository.
    pub fn shallow_commits(&self) -> Result<Option<crate::shallow::Commits>, crate::shallow::open::Error> {
        self.shallow_commits.recent_snapshot(
            || self.shallow_file().metadata().ok().and_then(|m| m.modified().ok()),
            || {
                let buf = match std::fs::read(self.shallow_file()) {
                    Ok(buf) => buf,
                    Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
                    Err(err) => return Err(err.into()),
                };

                let mut commits = buf
                    .lines()
                    .map(gix_hash::ObjectId::from_hex)
                    .collect::<Result<Vec<_>, _>>()?;

                commits.sort();
                if commits.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(commits))
                }
            },
        )
    }

    /// Return the path to the `shallow` file which contains hashes, one per line, that describe commits that don't have their
    /// parents within this repository.
    ///
    /// Note that it may not exist if the repository isn't actually shallow.
    pub fn shallow_file(&self) -> PathBuf {
        let shallow_name = self
            .config
            .resolved
            .string_filter_by_key(
                gitoxide::Core::SHALLOW_FILE.logical_name().as_str(),
                &mut self.filter_config_section(),
            )
            .unwrap_or_else(|| Cow::Borrowed("shallow".into()));
        self.common_dir().join(gix_path::from_bstr(shallow_name))
    }
}
