use crate::{compound, loose, pack};
use std::path::PathBuf;

/// Returned by [`compound::Db::at()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("The objects directory at '{0}' is not an accessible directory")]
    Inaccessible(PathBuf),
    #[error(transparent)]
    Pack(#[from] pack::bundle::Error),
    #[error(transparent)]
    Alternate(#[from] Box<crate::alternate::Error>),
}

/// Instantiation
impl compound::Db {
    /// Returns a compound database as initialized from the given git `objects_directory`.
    pub fn at(objects_directory: impl Into<PathBuf>) -> Result<compound::Db, Error> {
        let loose_objects = objects_directory.into();
        if !loose_objects.is_dir() {
            return Err(Error::Inaccessible(loose_objects));
        }
        let packs = match std::fs::read_dir(loose_objects.join("pack")) {
            Ok(entries) => {
                let mut packs_and_sizes = entries
                    .filter_map(Result::ok)
                    .filter_map(|e| e.metadata().map(|md| (e.path(), md)).ok())
                    .filter(|(_, md)| md.file_type().is_file())
                    .filter(|(p, _)| {
                        p.extension().unwrap_or_default() == "idx"
                            && p.file_name().unwrap_or_default().to_string_lossy().starts_with("pack-")
                    })
                    .map(|(p, md)| pack::Bundle::at(p).map(|b| (b, md.len())))
                    .collect::<Result<Vec<_>, _>>()?;
                packs_and_sizes.sort_by_key(|e| e.1);
                packs_and_sizes.into_iter().rev().map(|(b, _)| b).collect()
            }
            Err(_) => Vec::new(),
        };

        Ok(compound::Db {
            loose: loose::Db::at(loose_objects.clone()),
            packs,
            alternates: crate::alternate::resolve(loose_objects).map_err(Box::new)?,
        })
    }
}
