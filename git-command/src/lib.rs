//! Launch commands very similarly to `Command`, but with `git` specific capabilities and adjustments.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

use std::ffi::OsString;

/// A structure to keep settings to use when invoking a command via [`spawn()`][Prepare::spawn()], after creating it with [`prepare()`].
pub struct Prepare {
    command: OsString,
    stdin: std::process::Stdio,
    stdout: std::process::Stdio,
    stderr: std::process::Stdio,
    args: Vec<OsString>,
    use_shell: bool,
}

mod prepare {
    use crate::Prepare;
    use std::process::{Command, Stdio};

    /// Builder
    impl Prepare {
        /// If called, the command will not be executed directly, but with `sh`.
        ///
        /// This also allows to pass shell scripts as command, or use commands that contain arguments which are subsequently
        /// parsed by `sh`.
        pub fn with_shell(mut self) -> Self {
            self.use_shell = true;
            self
        }

        /// Configure the process to use `stdio` for _stdin.
        pub fn stdin(mut self, stdio: Stdio) -> Self {
            self.stdin = stdio;
            self
        }
        /// Configure the process to use `stdio` for _stdout_.
        pub fn stdout(mut self, stdio: Stdio) -> Self {
            self.stdout = stdio;
            self
        }
        /// Configure the process to use `stdio` for _stderr.
        pub fn stderr(mut self, stdio: Stdio) -> Self {
            self.stderr = stdio;
            self
        }

        /// Add `arg` to the list of arguments to call the command with.
        pub fn arg(mut self, arg: impl Into<std::ffi::OsString>) -> Self {
            self.args.push(arg.into());
            self
        }
    }

    /// Finalization
    impl Prepare {
        /// Spawn the command as configured.
        pub fn spawn(self) -> std::io::Result<std::process::Child> {
            let mut cmd: Command = self.into();
            cmd.spawn()
        }
    }

    impl Into<Command> for Prepare {
        fn into(mut self) -> Command {
            let mut cmd = if self.use_shell {
                let mut cmd = Command::new(if cfg!(windows) { "sh" } else { "/bin/sh" });
                cmd.arg("-c");
                if !self.args.is_empty() {
                    self.command.push(" \"$@\"")
                }
                cmd.arg(self.command);
                cmd.arg("--");
                cmd
            } else {
                Command::new(self.command)
            };
            cmd.stdin(self.stdin)
                .stdout(self.stdout)
                .stderr(self.stderr)
                .args(self.args);
            cmd
        }
    }
}

/// Prepare `cmd` for [spawning][Process::spawn()] by configuring it with various builder methods.
///
/// Note that the default IO is configured for typical API usage, that is
///
/// - `stdin` is null
/// - `stdout` is captured.
/// - `stderr` is null
pub fn prepare(cmd: impl Into<OsString>) -> Prepare {
    Prepare {
        command: cmd.into(),
        stdin: std::process::Stdio::null(),
        stdout: std::process::Stdio::piped(),
        stderr: std::process::Stdio::null(),
        args: Vec::new(),
        use_shell: false,
    }
}
