use std::{io::Read, ops::Deref};

use gix_hash::ObjectId;
use gix_object::Kind;

use crate::store;

mod error {
    use crate::{loose, store};

    /// The error returned by the [dynamic Store's][crate::Store] [`Write`][crate::Write] implementation.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        LoadIndex(#[from] store::load_index::Error),
        #[error(transparent)]
        LooseWrite(#[from] loose::write::Error),
        #[error(transparent)]
        Io(#[from] std::io::Error),
    }
}
pub use error::Error;

use crate::store_impls::dynamic;

impl<S> crate::Write for store::Handle<S>
where
    S: Deref<Target = dynamic::Store> + Clone,
{
    fn write_stream(&self, kind: Kind, size: u64, from: &mut dyn Read) -> Result<ObjectId, crate::write::Error> {
        let mut snapshot = self.snapshot.borrow_mut();
        Ok(match snapshot.loose_dbs.first() {
            Some(ldb) => ldb.write_stream(kind, size, from)?,
            None => {
                let new_snapshot = self
                    .store
                    .load_one_index(self.refresh, snapshot.marker)
                    .map_err(Box::new)?
                    .expect("there is always at least one ODB, and this code runs only once for initialization");
                *snapshot = new_snapshot;
                snapshot.loose_dbs[0].write_stream(kind, size, from)?
            }
        })
    }
}
