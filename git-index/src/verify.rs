use std::cmp::Ordering;

use crate::State;

pub mod entries {
    use bstr::BString;
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            OutOfOrder { current_index: usize, current_path: BString, current_stage: u8, previous_path: BString, previous_stage: u8 } {
                display("Entry '{}' (stage = {}) at index {} should order after prior entry '{}' (stage = {})", current_path, current_stage, current_index, previous_path, previous_stage)
            }
        }
    }
}

pub mod extensions {
    use quick_error::quick_error;

    use crate::extension;

    pub fn no_find<'a>(_: &git_hash::oid, _: &'a mut Vec<u8>) -> Option<git_object::TreeRefIter<'a>> {
        None
    }

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Tree(err: extension::tree::verify::Error) {
                display("The tree extension wasn't valid")
                source(err)
                from()
            }
        }
    }
}

impl State {
    pub fn verify_entries(&self) -> Result<(), entries::Error> {
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
        F: for<'a> FnMut(&git_hash::oid, &'a mut Vec<u8>) -> Option<git_object::TreeRefIter<'a>>,
    {
        self.tree().map(|t| t.verify(use_find, find)).transpose()?;
        // TODO: verify links by running the whole set of tests on the index
        //       - do that once we load it as well, or maybe that's lazy loaded? Too many questions for now.
        Ok(())
    }
}
