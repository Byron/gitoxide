mod impls {
    use std::ops::{Deref, DerefMut};

    use crate::{File, State};

    impl Deref for File {
        type Target = State;

        fn deref(&self) -> &Self::Target {
            &self.state
        }
    }

    impl DerefMut for File {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.state
        }
    }
}

mod impl_ {
    use std::fmt::Formatter;

    impl std::fmt::Debug for crate::File {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("File")
                .field("path", &self.path.display())
                .field("checksum", &self.checksum)
                .finish_non_exhaustive()
        }
    }
}

mod access {
    use crate::File;

    /// Consumption
    impl File {
        /// Take the state and discard the rest.
        pub fn into_state(self) -> crate::State {
            self.state
        }

        /// Take all non-copy parts of the index.
        pub fn into_parts(self) -> (crate::State, std::path::PathBuf) {
            (self.state, self.path)
        }
    }

    /// Access
    impl File {
        /// The path from which the index was read or to which it is supposed to be written when used with [`File::from_state()`].
        pub fn path(&self) -> &std::path::Path {
            &self.path
        }

        /// The checksum over the file that was read or written to disk, or `None` if the state in memory was never serialized.
        ///
        /// Note that even if `Some`, it will only represent the state in memory right after reading or [writing][File::write()].
        pub fn checksum(&self) -> Option<git_hash::ObjectId> {
            (!self.checksum.is_null()).then(|| self.checksum)
        }
    }
}

///
pub mod init;
///
pub mod verify;
///
pub mod write;
