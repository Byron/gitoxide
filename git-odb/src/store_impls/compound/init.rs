use std::path::PathBuf;

use crate::{
    pack,
    store_impls::{compound, loose},
};

/// Returned by [`compound::Store::at()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("The objects directory at '{0}' is not an accessible directory")]
    Inaccessible(PathBuf),
    #[error(transparent)]
    Pack(#[from] pack::bundle::init::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Alternate(#[from] Box<crate::alternate::Error>),
}

/// Instantiation
impl compound::Store {
    /// Returns a compound database as initialized from the given git `objects_directory`, commonly `.git/objects`.
    ///
    /// Only loose and packed objects will be considered. See the [linked Db][crate::linked::Store] for a database with
    /// support for _git alternates_, i.e. linking to other repositories.
    ///
    /// `pack_id_offset` is used to allow multiple compound databases to be used for lookups without their pack-ids clashing.
    pub fn at(objects_directory: impl Into<PathBuf>, pack_id_offset: u32) -> Result<compound::Store, Error> {
        let loose_objects = objects_directory.into();
        if !loose_objects.is_dir() {
            return Err(Error::Inaccessible(loose_objects));
        }
        let packs = match std::fs::read_dir(loose_objects.join("pack")) {
            Ok(entries) => {
                let mut packs_and_modification_time = entries
                    .filter_map(Result::ok)
                    .filter_map(|e| e.metadata().map(|md| (e.path(), md)).ok())
                    .filter(|(_, md)| md.file_type().is_file())
                    .filter(|(p, _)| p.extension().unwrap_or_default() == "idx")
                    .enumerate()
                    .map(|(idx, (p, md))| {
                        pack::Bundle::at(p).map_err(Error::from).and_then(|mut b| {
                            md.modified().map_err(Into::into).map(|mod_time| {
                                (
                                    {
                                        // don't rely on crc32 for producing non-clashing ids. It's the kind of bug we don't want
                                        b.pack.id = idx as u32 + pack_id_offset;
                                        b
                                    },
                                    mod_time,
                                )
                            })
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                // Like libgit2, sort by modification date, newest first
                packs_and_modification_time.sort_by(|l, r| l.1.cmp(&r.1).reverse());
                packs_and_modification_time.into_iter().rev().map(|(b, _)| b).collect()
            }
            Err(_) => Vec::new(),
        };

        Ok(compound::Store {
            loose: loose::Store::at(loose_objects, git_hash::Kind::Sha1),
            bundles: packs,
        })
    }
}
