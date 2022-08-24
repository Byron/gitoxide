use crate::{helper, Helper, Program};
use std::process::{Command, Stdio};

impl Program {
    /// Create a new program of the given `kind`.
    pub fn from_kind(kind: helper::Kind) -> Self {
        Program::Ready(kind)
    }
}

impl Helper for Program {
    type Send = std::process::ChildStdin;
    type Receive = std::process::ChildStdout;

    fn start(&mut self, action: &helper::Action) -> std::io::Result<(Self::Send, Option<Self::Receive>)> {
        match self {
            Program::Ready(kind) => {
                let (mut cmd, is_custom) = match kind {
                    helper::Kind::GitCredential => {
                        let mut cmd = Command::new(cfg!(windows).then(|| "git.exe").unwrap_or("git"));
                        cmd.arg("credential");
                        (cmd, false)
                    }
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
