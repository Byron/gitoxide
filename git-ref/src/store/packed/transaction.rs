#![allow(missing_docs)]

use crate::{
    mutable::Target,
    store::packed,
    transaction::{Change, RefEdit, RefEditsExt},
};
use std::io::Write;

/// Access and instantiation
impl packed::Transaction {
    /// Create an entirely new packfile using the given `lock` representing the resource to write.
    /// Note that it's up to the caller to assure a race cannot occur.
    pub(crate) fn new_empty(lock: git_lock::File) -> Self {
        packed::Transaction {
            buffer: None,
            edits: None,
            lock: Some(lock),
            closed_lock: None,
        }
    }

    pub(crate) fn new_from_pack_and_lock(buffer: packed::Buffer, lock: git_lock::File) -> Self {
        packed::Transaction {
            buffer: Some(buffer),
            edits: None,
            lock: Some(lock),
            closed_lock: None,
        }
    }
}

/// Access
impl packed::Transaction {
    /// Returns our packed buffer
    pub fn buffer(&self) -> Option<&packed::Buffer> {
        self.buffer.as_ref()
    }
}

/// Lifecycle
impl packed::Transaction {
    /// Prepare the transaction by checking all edits for applicability.
    pub fn prepare(mut self, edits: impl IntoIterator<Item = RefEdit>) -> Result<Self, prepare::Error> {
        match self.edits {
            None => {
                let mut edits: Vec<RefEdit> = edits.into_iter().collect();
                edits
                    .pre_process(
                        |name| {
                            self.buffer
                                .as_ref()
                                .and_then(|b| b.find_existing(name).map(|r| Target::Peeled(r.target())).ok())
                        },
                        |_idx, update| update,
                    )
                    .map_err(prepare::Error::PreprocessingFailed)?;

                // Remove all edits which are deletions that aren't here in the first place
                let buffer = &self.buffer;
                edits.retain(|edit| {
                    if let Change::Delete { .. } = edit.change {
                        buffer
                            .as_ref()
                            .map_or(true, |b| b.find_existing(edit.name.borrow()).is_ok())
                    } else {
                        true
                    }
                });
                if edits.is_empty() {
                    self.closed_lock = self
                        .lock
                        .take()
                        .map(|l| l.close())
                        .transpose()
                        .map_err(prepare::Error::CloseLock)?;
                } else {
                    for edit in edits.iter_mut() {
                        let existing = self
                            .buffer
                            .as_ref()
                            .and_then(|p| p.find_existing(edit.name.to_partial()).ok());
                        match (existing, &mut edit.change) {
                            (
                                Some(existing),
                                Change::Delete {
                                    previous: Some(Target::Peeled(desired_oid)),
                                    ..
                                },
                            ) => {
                                if !desired_oid.is_null() && *desired_oid != existing.target() {
                                    todo!("delete existing mismatch")
                                }
                                *desired_oid = existing.target();
                            }
                            _ => todo!("all other cases"),
                        }
                    }
                }
                self.edits = Some(edits);
            }
            Some(_) => {
                panic!("BUG: cannot call prepare(…) more than once")
            }
        }
        Ok(self)
    }

    /// Commit the prepare transaction
    pub fn commit(self) -> Result<Vec<RefEdit>, commit::Error> {
        match self.edits {
            Some(mut edits) => {
                if edits.is_empty() {
                    Ok(edits)
                } else {
                    let mut file = self.lock.expect("a write lock for applying changes");
                    let refs_sorted: Box<dyn Iterator<Item = Result<packed::Reference<'_>, packed::iter::Error>>> =
                        match self.buffer.as_ref() {
                            Some(buffer) => Box::new(buffer.iter()?),
                            None => Box::new(std::iter::empty()),
                        };

                    let mut refs_sorted = refs_sorted.peekable();

                    edits.sort_by(|l, r| l.name.as_bstr().cmp(r.name.as_bstr()));
                    let mut peekable_sorted_edits = edits.iter().peekable();

                    let header_line = b"# pack-refs with: peeled fully-peeled sorted \n";
                    file.with_mut(|f| f.write_all(header_line))?;

                    let mut num_written_lines = 0;
                    loop {
                        // TODO: a way to resolve/peel target objects
                        num_written_lines += 1;
                        match (refs_sorted.peek(), peekable_sorted_edits.peek()) {
                            (Some(Err(_)), _) => {
                                let err = refs_sorted.next().expect("next").expect_err("err");
                                return Err(commit::Error::Iteration(err));
                            }
                            (None, None) => {
                                num_written_lines -= 1;
                                break;
                            }
                            (Some(Ok(_)), None) => {
                                let pref = refs_sorted.next().expect("next").expect("no err");
                                write_packed_ref(&mut file, pref)?;
                            }
                            (Some(Ok(pref)), Some(edit)) => {
                                use std::cmp::Ordering::*;
                                match pref.name.as_bstr().cmp(edit.name.as_bstr()) {
                                    Less => {
                                        let pref = refs_sorted.next().expect("next").expect("valid");
                                        write_packed_ref(&mut file, pref)?;
                                    }
                                    Greater => {
                                        let edit = peekable_sorted_edits.next().expect("next");
                                        write_edit(&mut file, edit)?;
                                    }
                                    Equal => {
                                        let _pref = refs_sorted.next().expect("next").expect("valid");
                                        let edit = peekable_sorted_edits.next().expect("next");
                                        write_edit(&mut file, edit)?;
                                    }
                                }
                            }
                            (None, Some(_)) => {
                                write_edit(&mut file, peekable_sorted_edits.next().expect("next"))?;
                            }
                        }
                    }

                    if num_written_lines == 0 {
                        todo!("delete packed refs file, don't commit lock")
                    } else {
                        file.commit()?;
                        drop(refs_sorted);
                        Ok(edits)
                    }
                }
            }
            None => panic!("BUG: cannot call commit() before prepare(…)"),
        }
    }
}

fn write_packed_ref(file: &mut git_lock::File, pref: packed::Reference<'_>) -> std::io::Result<()> {
    file.with_mut(|out| {
        write!(out, "{} ", pref.target)?;
        out.write_all(pref.name.as_bstr())?;
        out.write_all(b"\n")?;
        if let Some(object) = pref.object {
            writeln!(out, "^{}", object)?;
        }
        Ok(())
    })
}

fn write_edit(file: &mut git_lock::File, edit: &RefEdit) -> std::io::Result<()> {
    match edit.change {
        Change::Delete { .. } => {}
        Change::Update {
            new: Target::Peeled(target_oid),
            ..
        } => file.with_mut(|out| {
            write!(out, "{} ", target_oid)?;
            out.write_all(edit.name.as_bstr())?;
            out.write_all(b"\n")
            // TODO: write peeled
        })?,
        Change::Update {
            new: Target::Symbolic(_),
            ..
        } => unreachable!("BUG: packed refs cannot contain symbolic refs, catch that in prepare(…)"),
    }
    Ok(())
}

impl packed::Buffer {
    /// Convert this buffer to be used as the basis for a transaction.
    pub fn into_transaction(
        self,
        lock_mode: git_lock::acquire::Fail,
    ) -> Result<packed::Transaction, git_lock::acquire::Error> {
        let lock = git_lock::File::acquire_to_update_resource(&self.path, lock_mode, None)?;
        Ok(packed::Transaction {
            buffer: Some(self),
            lock: Some(lock),
            closed_lock: None,
            edits: None,
        })
    }
}

///
pub mod prepare {
    use quick_error::quick_error;
    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            CloseLock(err: std::io::Error) {
                display("Could not close a lock which won't ever be committed")
                source(err)
            }
            PreprocessingFailed(err: std::io::Error) {
                display("Edit preprocessing failed with error: {}", err.to_string())
                source(err)
            }
        }
    }
}

///
pub mod commit {
    use crate::store::packed;
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Commit(err: git_lock::commit::Error<git_lock::File>) {
                display("Changes to the resource could not be comitted")
                from()
                source(err)
            }
            Iteration(err: packed::iter::Error) {
                display("Some references in the packed refs buffer could not be parsed")
                from()
                source(err)
            }
            Io(err: std::io::Error) {
                display("Failed to write a ref line to the packed ref file")
                from()
                source(err)
            }
        }
    }
}
