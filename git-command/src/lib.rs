#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]
#![allow(dead_code)]

use std::ffi::OsString;

pub struct Prepare {
    command: OsString,
    use_shell: bool,
}

mod prepare {
    use crate::Prepare;

    /// Builder
    impl Prepare {
        pub fn with_shell(mut self) -> Self {
            self.use_shell = true;
            self
        }
    }

    /// Finalization
    impl Prepare {
        pub fn spawn(self) -> std::io::Result<std::process::Child> {
            todo!("create command and spawn that")
        }
    }
}

pub fn prepare(cmd: impl Into<OsString>) -> Prepare {
    Prepare {
        command: cmd.into(),
        use_shell: false,
    }
}
