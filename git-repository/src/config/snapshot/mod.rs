mod access;

///
pub mod apply_cli_overrides;

///
pub mod credential_helpers;

mod _impls {
    use crate::config::{Snapshot, SnapshotMut};
    use std::fmt::{Debug, Formatter};
    use std::ops::{Deref, DerefMut};

    impl Debug for Snapshot<'_> {
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
            self.repo.config.resolved = std::mem::take(&mut self.config).into();
        }
    }

    impl Deref for SnapshotMut<'_> {
        type Target = git_config::File<'static>;

        fn deref(&self) -> &Self::Target {
            &self.config
        }
    }

    impl DerefMut for SnapshotMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.config
        }
    }
}
