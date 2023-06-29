use crate::driver::{process, Operation, Process, State};
use crate::{driver, Driver};
use bstr::BStr;

/// What to do if delay is supported by a process filter.
#[derive(Debug, Copy, Clone)]
pub enum Delay {
    /// Use delayed processing for this entry.
    ///
    /// Note that it's up to the filter to determine whether or not the processing should be delayed.
    Allow,
    /// Do not delay the processing, and force it to happen immediately. In this case, no delayed processing will occur
    /// even if the filter supports it.
    Forbid,
}

/// The error returned by [State::apply()][super::State::apply()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Init(#[from] driver::init::Error),
    #[error("Could not write entire object to driver")]
    WriteSource(#[from] std::io::Error),
    #[error("Failed to invoke '{command}' command")]
    ProcessInvoke {
        source: process::client::invoke::Error,
        command: String,
    },
    #[error("The invoked command '{command}' in process indicated an error: '{error}'")]
    ProcessStatus { error: String, command: String },
}

/// Additional information for use in the [`State::apply()`] method.
#[derive(Debug, Copy, Clone)]
pub struct Context<'a> {
    /// The repo-relative using slashes as separator of the entry currently being processed.
    pub rela_path: &'a BStr,
    /// The name of the reference that `HEAD` is pointing to. It's passed to `process` filters if present.
    pub ref_name: Option<&'a BStr>,
    /// The root-level tree that contains the current entry directly or indirectly, or the commit owning the tree (if available).
    ///
    /// This is passed to `process` filters if present.
    pub treeish: Option<gix_hash::ObjectId>,
    /// The actual blob-hash of the data we are processing. It's passed to `process` filters if present.
    ///
    /// Note that this hash might be different from the `$Id$` of the respective `ident` filter, as the latter generates the hash itself.
    pub blob: Option<gix_hash::ObjectId>,
}

impl State {
    /// Apply `operation` of `driver` to the bytes read from `src` and return a reader to immediately consume the output
    /// produced by the filter. `rela_path` is the repo-relative path of the entry to handle.
    ///
    /// Each call to this method will cause the corresponding filter to be invoked unless `driver` indicates a `process` filter,
    /// which is only launched once and maintained using this state.
    ///
    /// Note that it's not an error if there is no filter process for `operation` or if a long-running process doesn't supported
    /// the desired capability.
    pub fn apply<'a>(
        &'a mut self,
        driver: &Driver,
        src: impl std::io::Read,
        operation: Operation,
        ctx: Context<'_>,
    ) -> Result<Option<Box<dyn std::io::Read + 'a>>, Error> {
        match self.apply_delayed(driver, src, operation, Delay::Forbid, ctx)? {
            Some(MaybeDelayed::Delayed(_)) => {
                unreachable!("we forbid delaying the entry")
            }
            Some(MaybeDelayed::Immediate(read)) => Ok(Some(read)),
            None => Ok(None),
        }
    }

    /// Like [`apply()]`[Self::apply()], but use `delay` to determine if the filter result may be delayed or not.
    pub fn apply_delayed<'a>(
        &'a mut self,
        driver: &Driver,
        mut src: impl std::io::Read,
        operation: Operation,
        delay: Delay,
        ctx: Context<'_>,
    ) -> Result<Option<MaybeDelayed<'a>>, Error> {
        match self.process(driver, operation, ctx.rela_path)? {
            Some(Process::SingleFile { mut child, command }) => {
                std::io::copy(&mut src, &mut child.stdin.take().expect("configured"))?;
                Ok(Some(MaybeDelayed::Immediate(Box::new(ReadFilterOutput {
                    inner: child.stdout.take(),
                    child: driver.required.then_some((child, command)),
                }))))
            }
            Some(Process::MultiFile { client, key }) => {
                let command = match operation {
                    Operation::Clean => "clean",
                    Operation::Smudge => "smudge",
                };

                if !client.capabilities().contains(command) {
                    return Ok(None);
                }

                let status = client
                    .invoke(
                        command,
                        [
                            ("pathname", Some(ctx.rela_path.to_owned())),
                            ("ref", ctx.ref_name.map(ToOwned::to_owned)),
                            ("treeish", ctx.treeish.map(|id| id.to_hex().to_string().into())),
                            ("blob", ctx.blob.map(|id| id.to_hex().to_string().into())),
                            (
                                "can-delay",
                                match delay {
                                    Delay::Allow if client.capabilities().contains("delay") => Some("1".into()),
                                    Delay::Forbid | Delay::Allow => None,
                                },
                            ),
                        ]
                        .into_iter()
                        .filter_map(|(key, value)| value.map(|v| (key, v))),
                        src,
                    )
                    .map_err(|err| Error::ProcessInvoke {
                        command: command.into(),
                        source: err,
                    })?;
                if matches!(delay, Delay::Allow) && status.is_delayed() {
                    Ok(Some(MaybeDelayed::Delayed(key)))
                } else if status.is_success() {
                    Ok(Some(MaybeDelayed::Immediate(Box::new(client.as_read()))))
                } else {
                    // TODO: handle "error" and "abort", with abort removing the capability so we ignore it.
                    Err(Error::ProcessStatus {
                        command: command.into(),
                        error: match status {
                            process::Status::Named(error) => error,
                            process::Status::Previous => {
                                unreachable!("at this point a single status must be given")
                            }
                        },
                    })
                }
            }
            None => Ok(None),
        }
    }
}

/// A utility type to represent delayed or immediate apply-filter results.
pub enum MaybeDelayed<'a> {
    /// Using the delayed protocol, this entry has been sent to a long-running process and needs to be
    /// checked for again, later, using the [`driver::Key`] to refer to the filter who owes a response.
    ///
    /// Note that the path to the entry is also needed to obtain the filtered result later.
    Delayed(driver::Key),
    /// The filtered result can be read from the contained reader right away.
    ///
    /// Note that it must be consumed in full or till a read error occurs.
    Immediate(Box<dyn std::io::Read + 'a>),
}

/// A utility type to facilitate streaming the output of a filter process.
struct ReadFilterOutput {
    inner: Option<std::process::ChildStdout>,
    /// The child is present if we need its exit code to be positive.
    child: Option<(std::process::Child, std::process::Command)>,
}

impl std::io::Read for ReadFilterOutput {
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
