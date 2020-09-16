use crate::{compound, loose, pack};
use quick_error::quick_error;
use std::path::PathBuf;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Pack(err: pack::bundle::Error) {
            display("Failed to instantiate a pack bundle")
            source(err)
            from()
        }
    }
}

/// Instantiation
impl compound::Db {
    pub fn at(objects_directory: impl Into<PathBuf>) -> Result<compound::Db, Error> {
        let loose_objects = objects_directory.into();
        let packs = if let Ok(entries) = std::fs::read_dir(loose_objects.join("packs")) {
            let mut packs_and_sizes = entries
                .filter_map(Result::ok)
                .filter_map(|e| e.metadata().map(|md| (e.path(), md)).ok())
                .filter(|(_, md)| md.file_type().is_file())
                .filter(|(p, _)| p.extension().unwrap_or_default() == "idx" && p.starts_with("pack-"))
                .map(|(p, md)| pack::Bundle::at(p).map(|b| (b, md.len())))
                .collect::<Result<Vec<_>, _>>()?;
            packs_and_sizes.sort_by_key(|e| e.1);
            packs_and_sizes.into_iter().rev().map(|(b, _)| b).collect()
        } else {
            Vec::new()
        };

        Ok(compound::Db {
            loose: loose::Db::at(loose_objects),
            packs,
        })
    }
}
