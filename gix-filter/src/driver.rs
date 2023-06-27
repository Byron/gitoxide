use crate::Driver;
use bstr::{BStr, BString, ByteSlice, ByteVec};
use std::collections::HashMap;
use std::ffi::OsString;
use std::process::Stdio;

///
pub mod apply {
    use crate::driver::Operation;
    use bstr::BString;

    /// The error returned by [State::apply()][super::State::apply()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Failed to spawn driver: {command:?}")]
        SpawnCommand {
            source: std::io::Error,
            command: std::process::Command,
        },
        #[error("A {operation:?} program is not configured for the driver named '{driver}'")]
        MissingCommand { driver: BString, operation: Operation },
        #[error("Could not write entire object to driver")]
        WriteSource(#[from] std::io::Error),
    }
}

/// The kind of operation to apply using a driver
#[derive(Debug, Copy, Clone)]
pub enum Operation {
    /// Turn worktree content into content suitable for storage in `git`.
    Clean,
    /// Turn content stored in `git` to content suitable for the working tree.
    Smudge,
}

/// State required to handle `process` filters, which are running until all their work is done.
///
/// These can be significantly faster on some platforms as they are launched only once, while supporting asynchronous processing.
#[derive(Default)]
pub struct State {
    /// The list of currently running processes. These are preferred over simple clean-and-smudge programs.
    ///
    /// Note that these processes are expected to shut-down once their stdin/stdout are dropped, so nothing else
    /// needs to be done to clean them up after drop.
    _running: HashMap<OsString, std::process::Child>,
}

impl State {
    /// Apply `operation` of `driver` to the bytes read from `src` and return a reader to immediately consume the output
    /// produced by the filter. `rela_path` is the repo-relative path of the entry to handle.
    ///
    /// Each call to this method will cause the corresponding filter to be invoked unless `driver` indicates a `process` filter,
    /// which is only launched once.
    pub fn apply(
        &mut self,
        driver: &Driver,
        mut src: impl std::io::Read,
        operation: Operation,
        rela_path: &BStr,
    ) -> Result<impl std::io::Read, apply::Error> {
        match driver.process.as_ref() {
            Some(_process) => todo!("find existing or launch process"),
            None => {
                let cmd = match operation {
                    Operation::Clean => driver
                        .clean
                        .as_ref()
                        .map(|cmd| substitute_f_parameter(cmd.as_ref(), rela_path)),

                    Operation::Smudge => driver
                        .smudge
                        .as_ref()
                        .map(|cmd| substitute_f_parameter(cmd.as_ref(), rela_path)),
                };

                let cmd = cmd.ok_or_else(|| apply::Error::MissingCommand {
                    operation,
                    driver: driver.name.clone(),
                })?;

                let mut cmd: std::process::Command = gix_command::prepare(gix_path::from_bstr(cmd).into_owned())
                    .with_shell()
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit())
                    .into();
                let mut child = match cmd.spawn() {
                    Ok(child) => child,
                    Err(err) => {
                        return Err(apply::Error::SpawnCommand {
                            source: err,
                            command: cmd,
                        })
                    }
                };

                std::io::copy(&mut src, &mut child.stdin.take().expect("configured"))?;
                Ok(StdoutErrCheck {
                    inner: child.stdout.take(),
                    child: driver.required.then_some((child, cmd)),
                })
            }
        }
    }
}

struct StdoutErrCheck {
    inner: Option<std::process::ChildStdout>,
    /// The child is present if we need its exit code to be positive.
    child: Option<(std::process::Child, std::process::Command)>,
}

impl std::io::Read for StdoutErrCheck {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.inner.as_mut() {
            Some(inner) => {
                let num_read = inner.read(buf)?;
                if num_read == 0 {
                    self.inner.take();
                    if let Some((mut child, cmd)) = self.child.take() {
                        let status = child.wait()?;
                        if !status.success() {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("Driver process {cmd:?} failed"),
                            ));
                        }
                    }
                }
                Ok(num_read)
            }
            None => Ok(0),
        }
    }
}

/// Substitute `path` as shell-save version into `cmd` which could be something like `cmd something %f`.
fn substitute_f_parameter(cmd: &BStr, path: &BStr) -> BString {
    let mut buf: BString = Vec::with_capacity(cmd.len()).into();

    let mut ofs = 0;
    while let Some(pos) = cmd[ofs..].find(b"%f") {
        buf.push_str(&cmd[..ofs + pos]);
        buf.extend_from_slice(&gix_quote::single(path));
        ofs += pos + 2;
    }
    buf.push_str(&cmd[ofs..]);
    buf
}
