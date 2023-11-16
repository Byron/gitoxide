//! Launch commands very similarly to `Command`, but with `git` specific capabilities and adjustments.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

use std::ffi::OsString;

/// A structure to keep settings to use when invoking a command via [`spawn()`][Prepare::spawn()], after creating it with [`prepare()`].
pub struct Prepare {
    /// The command to invoke (either with or without shell depending on `use_shell`.
    pub command: OsString,
    /// The way standard input is configured.
    pub stdin: std::process::Stdio,
    /// The way standard output is configured.
    pub stdout: std::process::Stdio,
    /// The way standard error is configured.
    pub stderr: std::process::Stdio,
    /// The arguments to pass to the spawned process.
    pub args: Vec<OsString>,
    /// environment variables to set in the spawned process.
    pub env: Vec<(OsString, OsString)>,
    /// If `true`, we will use `sh` to execute the `command`.
    pub use_shell: bool,
    /// If `true` (default `true` on windows and `false` everywhere else)
    /// we will see if it's safe to manually invoke `command` after splitting
    /// its arguments as a shell would do.
    /// Note that outside of windows, it's generally not advisable as this
    /// removes support for literal shell scripts with shell-builtins.
    ///
    /// This mimics the behaviour we see with `git` on windows, which also
    /// won't invoke the shell there at all.
    ///
    /// Only effective if `use_shell` is `true` as well, as the shell will
    /// be used as a fallback if it's not possible to split arguments as
    /// the command-line contains 'scripting'.
    pub allow_manual_arg_splitting: bool,
}

mod prepare {
    use std::{
        ffi::OsString,
        process::{Command, Stdio},
    };

    use bstr::ByteSlice;

    use crate::Prepare;

    /// Builder
    impl Prepare {
        /// If called, the command will not be executed directly, but with `sh`, but ony if the
        /// command passed to [`prepare`](super::prepare()) requires this.
        ///
        /// This also allows to pass shell scripts as command, or use commands that contain arguments which are subsequently
        /// parsed by `sh`.
        pub fn with_shell(mut self) -> Self {
            self.use_shell = self.command.to_str().map_or(true, |cmd| {
                cmd.as_bytes().find_byteset(b"|&;<>()$`\\\"' \t\n*?[#~=%").is_some()
            });
            self
        }

        /// Unconditionally turn off using the shell when spawning the command.
        /// Note that not using the shell is the default so an effective use of this method
        /// is some time after [`with_shell()`][Prepare::with_shell()] was called.
        pub fn without_shell(mut self) -> Self {
            self.use_shell = false;
            self
        }

        /// Use a shell, but try to split arguments by hand if this be safely done without a shell.
        ///
        /// If that's not the case, use a shell instead.
        pub fn with_shell_allow_argument_splitting(mut self) -> Self {
            self.allow_manual_arg_splitting = true;
            self.with_shell()
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
        pub fn arg(mut self, arg: impl Into<OsString>) -> Self {
            self.args.push(arg.into());
            self
        }

        /// Add `args` to the list of arguments to call the command with.
        pub fn args(mut self, args: impl IntoIterator<Item = impl Into<OsString>>) -> Self {
            self.args
                .append(&mut args.into_iter().map(Into::into).collect::<Vec<_>>());
            self
        }

        /// Add `key` with `value` to the environment of the spawned command.
        pub fn env(mut self, key: impl Into<OsString>, value: impl Into<OsString>) -> Self {
            self.env.push((key.into(), value.into()));
            self
        }
    }

    /// Finalization
    impl Prepare {
        /// Spawn the command as configured.
        pub fn spawn(self) -> std::io::Result<std::process::Child> {
            let mut cmd = Command::from(self);
            gix_trace::debug!(cmd = ?cmd);
            cmd.spawn()
        }
    }

    impl From<Prepare> for Command {
        fn from(mut prep: Prepare) -> Command {
            let mut cmd = if prep.use_shell {
                let split_args = prep
                    .allow_manual_arg_splitting
                    .then(|| {
                        if gix_path::into_bstr(std::borrow::Cow::Borrowed(prep.command.as_ref()))
                            .find_byteset(b"\\|&;<>()$`\n*?[#~%")
                            .is_none()
                        {
                            prep.command
                                .to_str()
                                .and_then(|args| shell_words::split(args).ok().map(Vec::into_iter))
                        } else {
                            None
                        }
                    })
                    .flatten();
                match split_args {
                    Some(mut args) => {
                        let mut cmd = Command::new(args.next().expect("non-empty input"));
                        cmd.args(args);
                        cmd
                    }
                    None => {
                        let mut cmd = Command::new(if cfg!(windows) { "sh" } else { "/bin/sh" });
                        cmd.arg("-c");
                        if !prep.args.is_empty() {
                            prep.command.push(" \"$@\"")
                        }
                        cmd.arg(prep.command);
                        cmd.arg("--");
                        cmd
                    }
                }
            } else {
                Command::new(prep.command)
            };
            cmd.stdin(prep.stdin)
                .stdout(prep.stdout)
                .stderr(prep.stderr)
                .envs(prep.env)
                .args(prep.args);
            cmd
        }
    }
}

/// Prepare `cmd` for [spawning][std::process::Command::spawn()] by configuring it with various builder methods.
///
/// Note that the default IO is configured for typical API usage, that is
///
/// - `stdin` is null to prevent blocking unexpectedly on consumption of stdin
/// - `stdout` is captured for consumption by the caller
/// - `stderr` is inherited to allow the command to provide context to the user
pub fn prepare(cmd: impl Into<OsString>) -> Prepare {
    Prepare {
        command: cmd.into(),
        stdin: std::process::Stdio::null(),
        stdout: std::process::Stdio::piped(),
        stderr: std::process::Stdio::inherit(),
        args: Vec::new(),
        env: Vec::new(),
        use_shell: false,
        allow_manual_arg_splitting: cfg!(windows),
    }
}
