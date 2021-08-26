use std::io::Write;

use crate::{
    store::{file::transaction::FindObjectFn, packed, packed::Edit},
    transaction::{Change, RefEdit},
    Target,
};

pub(crate) const HEADER_LINE: &[u8] = b"# pack-refs with: peeled fully-peeled sorted \n";

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
    pub fn prepare(
        mut self,
        edits: impl IntoIterator<Item = RefEdit>,
        find: &mut FindObjectFn,
    ) -> Result<Self, prepare::Error> {
        assert!(self.edits.is_none(), "BUG: cannot call prepare(…) more than once");
        let buffer = &self.buffer;
        // Remove all edits which are deletions that aren't here in the first place
        let mut edits: Vec<Edit> = edits
            .into_iter()
            .filter(|edit| {
                if let Change::Delete { .. } = edit.change {
                    buffer.as_ref().map_or(true, |b| b.find(edit.name.to_ref()).is_ok())
                } else {
                    true
                }
            })
            .map(|change| Edit {
                inner: change,
                peeled: None,
            })
            .collect();

        let mut buf = Vec::new();
        for edit in edits.iter_mut() {
            if let Change::Update {
                new: Target::Peeled(new),
                ..
            } = edit.inner.change
            {
                let mut next_id = new;
                edit.peeled = loop {
                    let kind = find(next_id, &mut buf)?;
                    match kind {
                        Some(kind) if kind == git_object::Kind::Tag => {
                            next_id = git_object::TagRefIter::from_bytes(&buf).target_id().ok_or_else(|| {
                                prepare::Error::Resolve(
                                    format!("Couldn't get target object id from tag {}", next_id).into(),
                                )
                            })?;
                        }
                        Some(_) => {
                            break if next_id == new { None } else { Some(next_id) };
                        }
                        None => {
                            return Err(prepare::Error::Resolve(
                                format!("Couldn't find object with id {}", next_id).into(),
                            ))
                        }
                    }
                };
            }
        }

        if edits.is_empty() {
            self.closed_lock = self
                .lock
                .take()
                .map(|l| l.close())
                .transpose()
                .map_err(prepare::Error::CloseLock)?;
        } else {
            // NOTE that we don't do any additional checks here but apply all edits unconditionally.
            // This is because this transaction system is internal and will be used correctly from the
            // loose ref store transactions, which do the necessary checking.
        }
        self.edits = Some(edits);
        Ok(self)
    }

    /// Commit the prepared transaction.
    ///
    /// Please note that actual edits invalidated existing packed buffers.
    /// Note: There is the potential to write changes into memory and return such a packed-refs buffer for reuse.
    pub fn commit(self) -> Result<(), commit::Error> {
        let mut edits = self.edits.expect("BUG: cannot call commit() before prepare(…)");
        if edits.is_empty() {
            return Ok(());
        }

        let mut file = self.lock.expect("a write lock for applying changes");
        let refs_sorted: Box<dyn Iterator<Item = Result<packed::Reference<'_>, packed::iter::Error>>> =
            match self.buffer.as_ref() {
                Some(buffer) => Box::new(buffer.iter()?),
                None => Box::new(std::iter::empty()),
            };

        let mut refs_sorted = refs_sorted.peekable();

        edits.sort_by(|l, r| l.inner.name.as_bstr().cmp(r.inner.name.as_bstr()));
        let mut peekable_sorted_edits = edits.iter().peekable();

        file.with_mut(|f| f.write_all(HEADER_LINE))?;

        let mut num_written_lines = 0;
        loop {
            match (refs_sorted.peek(), peekable_sorted_edits.peek()) {
                (Some(Err(_)), _) => {
                    let err = refs_sorted.next().expect("next").expect_err("err");
                    return Err(commit::Error::Iteration(err));
                }
                (None, None) => {
                    break;
                }
                (Some(Ok(_)), None) => {
                    let pref = refs_sorted.next().expect("next").expect("no err");
                    num_written_lines += 1;
                    write_packed_ref(&mut file, pref)?;
                }
                (Some(Ok(pref)), Some(edit)) => {
                    use std::cmp::Ordering::*;
                    match pref.name.as_bstr().cmp(edit.inner.name.as_bstr()) {
                        Less => {
                            let pref = refs_sorted.next().expect("next").expect("valid");
                            num_written_lines += 1;
                            write_packed_ref(&mut file, pref)?;
                        }
                        Greater => {
                            let edit = peekable_sorted_edits.next().expect("next");
                            write_edit(&mut file, edit, &mut num_written_lines)?;
                        }
                        Equal => {
                            let _pref = refs_sorted.next().expect("next").expect("valid");
                            let edit = peekable_sorted_edits.next().expect("next");
                            write_edit(&mut file, edit, &mut num_written_lines)?;
                        }
                    }
                }
                (None, Some(_)) => {
                    let edit = peekable_sorted_edits.next().expect("next");
                    write_edit(&mut file, edit, &mut num_written_lines)?;
                }
            }
        }

        if num_written_lines == 0 {
            std::fs::remove_file(file.resource_path())?;
        } else {
            file.commit()?;
        }
        drop(refs_sorted);
        Ok(())
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

fn write_edit(file: &mut git_lock::File, edit: &Edit, lines_written: &mut i32) -> std::io::Result<()> {
    match edit.inner.change {
        Change::Delete { .. } => {}
        Change::Update {
            new: Target::Peeled(target_oid),
            ..
        } => {
            file.with_mut(|out| {
                write!(out, "{} ", target_oid)?;
                out.write_all(edit.inner.name.as_bstr())?;
                out.write_all(b"\n")?;
                if let Some(object) = edit.peeled {
                    writeln!(out, "^{}", object)?;
                }
                Ok(())
            })?;
            *lines_written += 1;
        }
        Change::Update {
            new: Target::Symbolic(_),
            ..
        } => unreachable!("BUG: packed refs cannot contain symbolic refs, catch that in prepare(…)"),
    }
    Ok(())
}

impl packed::Buffer {
    /// Convert this buffer to be used as the basis for a transaction.
    pub(crate) fn into_transaction(
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
        /// The error used in [`Transaction::prepare(…)`][super::packed::Transaction::prepare()].
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            CloseLock(err: std::io::Error) {
                display("Could not close a lock which won't ever be committed")
                source(err)
            }
            Resolve(err: Box<dyn std::error::Error + Send + Sync + 'static>) {
                display("The lookup of an object failed while peeling it")
                from()
                source(&**err)
            }
        }
    }
}

///
pub mod commit {
    use quick_error::quick_error;

    use crate::store::packed;

    quick_error! {
        /// The error used in [`Transaction::commit(…)`][super::packed::Transaction::commit()].
        #[derive(Debug)]
        #[allow(missing_docs)]
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
