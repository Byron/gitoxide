use std::{io::Read, ops::Deref};

use git_hash::ObjectId;
use git_object::Kind;

use crate::store;

mod error {
    use crate::{loose, store};

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        LoadIndex(#[from] store::load_index::Error),
        #[error(transparent)]
        LooseWrite(#[from] loose::write::Error),
        #[error(transparent)]
        Io(#[from] std::io::Error),
    }
}
use crate::store_impls::dynamic;
pub use error::Error;

impl<S> crate::Write for store::Handle<S>
where
    S: Deref<Target = dynamic::Store> + Clone,
{
    type Error = Error;

    fn write_stream(
        &self,
        kind: Kind,
        size: u64,
        from: impl Read,
        hash: git_hash::Kind,
    ) -> Result<ObjectId, Self::Error> {
        let mut snapshot = self.snapshot.borrow_mut();
        Ok(match snapshot.loose_dbs.get(0) {
            Some(ldb) => ldb.write_stream(kind, size, from, hash)?,
            None => {
                let new_snapshot = self
                    .store
                    .load_one_index(self.refresh_mode, snapshot.marker)?
                    .expect("there is always at least one ODB, and this code runs only once for initialization");
                *snapshot = new_snapshot;
                snapshot.loose_dbs[0].write_stream(kind, size, from, hash)?
            }
        })
    }
}
