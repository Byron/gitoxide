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
pub mod init;
pub mod verify;
