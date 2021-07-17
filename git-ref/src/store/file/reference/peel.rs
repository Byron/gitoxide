use crate::file::{self, find, reference::State, Reference};
use bstr::ByteSlice;
use quick_error::quick_error;

quick_error! {
    /// The error returned by [`Reference::peel_one_level()`].
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        FindExisting(err: find::existing::Error) {
            display("Could not resolve symbolic reference name that is expected to exist")
            source(err)
        }
        Decode(err: file::reference::decode::Error) {
            display("The reference could not be decoded.")
            source(err)
        }
    }
}

impl<'a> Reference<'a> {
    /// Follow this symbolic reference one level and return the ref it refers to.
    ///
    /// Returns `None` if this is not a symbolic reference, hence the leaf of the chain.
    pub fn peel_one_level(&self) -> Option<Result<Reference<'a>, Error>> {
        match &self.state {
            State::Id(_) => None,
            State::ValidatedPath(relative_path) => {
                let path = relative_path.to_path_lossy();
                match self.parent.find_one_with_verified_input(path.as_ref()) {
                    Ok(Some(next)) => Some(Ok(next)),
                    Ok(None) => Some(Err(Error::FindExisting(find::existing::Error::NotFound(
                        path.into_owned(),
                    )))),
                    Err(err) => Some(Err(Error::FindExisting(find::existing::Error::Find(err)))),
                }
            }
        }
    }
}

///
pub mod to_id {
    use crate::file::{reference, Reference};
    use git_hash::oid;
    use quick_error::quick_error;
    use std::{collections::BTreeSet, path::PathBuf};

    quick_error! {
        /// The error returned by [`Reference::peel_to_id_in_place()`].
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            PeelOne(err: reference::peel::Error) {
                display("Could not peel a single level of a reference")
                from()
                source(err)
            }
            Cycle(start_absolute: PathBuf){
                display("Aborting due to reference cycle with first seen path being '{}'", start_absolute.display())
            }
            DepthLimitExceeded{  max_depth: usize  } {
                display("Refusing to follow more than {} levels of indirection", max_depth)
            }
        }
    }

    impl<'a> Reference<'a> {
        /// Peel this symbolic reference until the end of the chain is reached and an object ID is available.
        ///
        /// If an error occurs this reference remains unchanged.
        pub fn peel_to_id_in_place(&mut self) -> Result<&oid, Error> {
            let mut count = 0;
            let mut seen = BTreeSet::new();
            let mut storage;
            let mut cursor = &mut *self;
            while let Some(next) = cursor.peel_one_level() {
                let next_ref = next?;
                if let crate::Kind::Peeled = next_ref.kind() {
                    *self = next_ref;
                    return Ok(self.state.as_id().expect("it to be present"));
                }
                storage = next_ref;
                cursor = &mut storage;
                if seen.contains(&cursor.relative_path) {
                    return Err(Error::Cycle(cursor.parent.base.join(&cursor.relative_path)));
                }
                seen.insert(cursor.relative_path.clone());
                count += 1;
                const MAX_REF_DEPTH: usize = 5;
                if count == MAX_REF_DEPTH {
                    return Err(Error::DepthLimitExceeded {
                        max_depth: MAX_REF_DEPTH,
                    });
                }
            }
            Ok(self.state.as_id().expect("to be peeled"))
        }
    }
}
