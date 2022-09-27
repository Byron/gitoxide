use git_features::threading::OwnShared;
use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};

use crate::config::{CommitAndRollback, Snapshot, SnapshotMut};

impl Debug for Snapshot<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repo.config.resolved.to_string())
    }
}

impl Debug for CommitAndRollback<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repo.config.resolved.to_string())
    }
}

impl Debug for SnapshotMut<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.config.to_string())
    }
}

impl Drop for SnapshotMut<'_> {
    fn drop(&mut self) {
        if let Some(repo) = self.repo.take() {
            repo.config.resolved = std::mem::take(&mut self.config).into();
            repo.config.reread_values_and_clear_caches().ok();
        };
    }
}

impl Drop for CommitAndRollback<'_> {
    fn drop(&mut self) {
        self.repo.config.resolved = OwnShared::clone(&self.prev_config);
        self.repo.config.reread_values_and_clear_caches().ok();
    }
}

impl Deref for SnapshotMut<'_> {
    type Target = git_config::File<'static>;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl Deref for CommitAndRollback<'_> {
    type Target = crate::Repository;

    fn deref(&self) -> &Self::Target {
        self.repo
    }
}

impl DerefMut for SnapshotMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}
