use crate::{compound, linked, store::alternate};
use std::path::PathBuf;

/// The error returned by [`linked::Db::at()`]
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    CompoundDbInit(#[from] compound::init::Error),
    #[error(transparent)]
    AlternateResolve(#[from] alternate::Error),
}

impl linked::Db {
    /// Instantiate an instance at the given `objects_directory`, commonly `.git/objects`.
    ///
    /// _git alternate_ files will be traversed to build a chain of [`compound::Db`] instances.
    pub fn at(objects_directory: impl Into<PathBuf>) -> Result<Self, Error> {
        let mut dbs = vec![compound::Db::at(objects_directory.into())?];
        for object_path in alternate::resolve(dbs[0].loose.path.clone())?.into_iter() {
            dbs.push(compound::Db::at(object_path)?);
        }
        assert!(
            !dbs.is_empty(),
            "we can rely on at least one compound database to be present"
        );
        Ok(linked::Db { dbs })
    }
}

impl std::convert::TryFrom<PathBuf> for linked::Db {
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        linked::Db::at(value)
    }
}
