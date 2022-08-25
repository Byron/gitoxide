use crate::{helper, Helper, Program};
use bstr::{BString, ByteSlice, ByteVec};
use std::process::{Command, Stdio};

/// The kind of helper program to use.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Kind {
    /// The built-in `git credential` helper program, part of any git distribution.
    Builtin,
    /// A custom credentials helper, as identified just by the name with optional arguments
    CustomName {
        /// The name like `foo` along with optional args, like `foo --arg --bar="a b"`, with arguments using `sh` shell quoting rules.
        /// The program executed will be `git-credential-foo` if `name_and_args` starts with `foo`.
        name_and_args: BString,
    },
    /// A custom credentials helper, as identified just by the absolute path to the program and optional arguments. The program is executed through a shell.
    CustomPath {
        /// The absolute path to the executable, like `/path/to/exe` along with optional args, like `/path/to/exe --arg --bar="a b"`, with arguments using `sh`
        /// shell quoting rules.
        path_and_args: BString,
    },
    /// A script to execute with `sh`.
    CustomScript(BString),
}

impl Program {
    /// Create a new program of the given `kind`.
    pub fn from_kind(kind: Kind) -> Self {
        Program::Ready(kind)
    }

    /// Parse the given input as per the custom helper definition, supporting `!<script>`, `name` and `/absolute/name`, the latter two
    /// also support arguments which are ignored here.
    pub fn from_custom_definition(input: impl Into<BString>) -> Self {
        let mut input = input.into();
        Program::Ready(if input.starts_with(b"!") {
            input.remove(0);
            Kind::CustomScript(input)
        } else {
            let path = git_path::from_bstr(
                input
                    .find_byte(b' ')
                    .map_or(input.as_slice(), |pos| &input[..pos])
                    .as_bstr(),
            );
            if path.is_absolute() || input.first() == Some(&b'/') {
                Kind::CustomPath { path_and_args: input }
            } else {
                input.insert_str(0, "git credential-");
                Kind::CustomName { name_and_args: input }
            }
        })
    }
}

impl Helper for Program {
    type Send = std::process::ChildStdin;
    type Receive = std::process::ChildStdout;

    fn start(&mut self, action: &helper::invoke::Action) -> std::io::Result<(Self::Send, Option<Self::Receive>)> {
        match self {
            Program::Ready(kind) => {
                let (mut cmd, is_custom) = match kind {
                    Kind::Builtin => {
                        let mut cmd = Command::new(cfg!(windows).then(|| "git.exe").unwrap_or("git"));
                        cmd.arg("credential");
                        (cmd, false)
                    }
                    Kind::CustomScript(for_shell)
                    | Kind::CustomName {
                        name_and_args: for_shell,
                    }
                    | Kind::CustomPath {
                        path_and_args: for_shell,
                    } => todo!("name and args: {for_shell:?}"),
                };
                cmd.arg(action.as_helper_arg(is_custom))
                    .stdin(Stdio::piped())
                    .stdout(if action.expects_output() {
                        Stdio::piped()
                    } else {
                        Stdio::null()
                    });
                let mut child = cmd.spawn()?;
                let stdin = child.stdin.take().expect("stdin to be configured");
                let stdout = child.stdout.take();

                *self = Program::Started(child);
                Ok((stdin, stdout))
            }
            Program::Started(_) => panic!("BUG: must not call `start()` twice"),
        }
    }

    fn finish(self) -> std::io::Result<()> {
        match self {
            Program::Started(mut child) => child.wait().and_then(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Credentials helper program failed with status code {:?}", status.code()),
                    ))
                }
            }),
            Program::Ready(_) => panic!("Call `start()` before calling finish()"),
        }
    }
}
