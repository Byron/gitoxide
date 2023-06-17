use std::cmp::Ordering;

use crate::State;

///
pub mod entries {
    use bstr::BString;

    /// The error returned by [`State::verify_entries()`][crate::State::verify_entries()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Entry '{current_path}' (stage = {current_stage}) at index {current_index} should order after prior entry '{previous_path}' (stage = {previous_stage})")]
        OutOfOrder {
            current_index: usize,
            current_path: BString,
            current_stage: u8,
            previous_path: BString,
            previous_stage: u8,
        },
    }
}

///
pub mod extensions {
    use crate::extension;

    /// An implementation of a `find` function that never finds or returns any object, a no-op.
    pub fn no_find<'a>(_: &gix_hash::oid, _: &'a mut Vec<u8>) -> Option<gix_object::TreeRefIter<'a>> {
        None
    }

    /// The error returned by [`State::verify_extensions()`][crate::State::verify_extensions()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Tree(#[from] extension::tree::verify::Error),
    }
}

impl State {
    /// Assure our entries are consistent.
    pub fn verify_entries(&self) -> Result<(), entries::Error> {
        let _span = gix_features::trace::coarse!("gix_index::File::verify_entries()");
        let mut previous = None::<&crate::Entry>;
        for (idx, entry) in self.entries.iter().enumerate() {
            if let Some(prev) = previous {
                if prev.cmp(entry, self) != Ordering::Less {
                    return Err(entries::Error::OutOfOrder {
                        current_index: idx,
                        current_path: entry.path(self).into(),
                        current_stage: entry.flags.stage() as u8,
                        previous_path: prev.path(self).into(),
                        previous_stage: prev.flags.stage() as u8,
                    });
                }
            }
            previous = Some(entry);
        }
        Ok(())
    }

    /// Note: `find` cannot be `Option<F>` as we can't call it with a closure then due to the indirection through `Some`.
    pub fn verify_extensions<F>(&self, use_find: bool, find: F) -> Result<(), extensions::Error>
    where
        F: for<'a> FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Option<gix_object::TreeRefIter<'a>>,
    {
        self.tree().map(|t| t.verify(use_find, find)).transpose()?;
        // TODO: verify links by running the whole set of tests on the index
        //       - do that once we load it as well, or maybe that's lazy loaded? Too many questions for now.
        Ok(())
    }
}
