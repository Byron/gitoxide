use crate::State;
use std::cmp::Ordering;

pub mod entries {
    use bstr::BString;
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            OutOfOrder { current_index: usize, current_path: BString, current_stage: u8, previous_path: BString, previous_stage: u8 } {
                display("todo")
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
}
