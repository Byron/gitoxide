use crate::{helper, Helper, Program};
use std::process::{Command, Stdio};

impl Helper for Program {
    type Send = std::process::ChildStdin;
    type Receive = std::process::ChildStdout;

    fn start(&mut self, action: &helper::Action<'_>) -> std::io::Result<(Self::Send, Option<Self::Receive>)> {
        assert!(self.child.is_none(), "BUG: cannot call this twice");
        let (mut cmd, is_custom) = match self.kind {
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

        self.child = Some(child);
        Ok((stdin, stdout))
    }

    fn finish(self) -> std::io::Result<()> {
        self.child.expect("start() called").wait().and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Credentials helper program failed with status code {:?}", status.code()),
                ))
            }
        })
    }
}
