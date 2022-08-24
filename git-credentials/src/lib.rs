//! Interact with git credentials in various ways and launch helper programs.
//!
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

/// A utility trait to launch a credentials helper, as well as stop them gracefully.
pub trait Helper {
    /// A way to send data to the helper.
    type Send: std::io::Write;
    /// A way to receive data from the helper.
    type Receive: std::io::Read;

    /// Start the helper and provide handles to send and receive from it.
    fn start(&mut self, action: &helper::Action<'_>) -> std::io::Result<(Self::Send, Option<Self::Receive>)>;
    /// Stop the helper and provide a way to determine it's successful.
    fn finish(self) -> std::io::Result<()>;
}

/// A program implementing a credentials helper.
pub struct Program {
    /// The kind of helper program.
    pub kind: helper::Kind,
    child: Option<std::process::Child>,
}

mod program {
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
}

///
pub mod helper;
pub use helper::function::helper;
