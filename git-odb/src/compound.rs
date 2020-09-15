use crate::{loose, pack};

pub struct Db {
    pub loose: loose::Db,
    pub packs: Vec<pack::Bundle>,
}

mod write {
    use crate::{compound, loose};
    use git_object::{owned, HashKind, Kind};
    use std::io::Read;

    impl crate::Write for compound::Db {
        type Error = loose::db::write::Error;

        fn write(&self, object: &owned::Object, hash: HashKind) -> Result<owned::Id, Self::Error> {
            self.loose.write(object, hash)
        }

        fn write_buf(&self, object: Kind, from: &[u8], hash: HashKind) -> Result<owned::Id, Self::Error> {
            self.loose.write_buf(object, from, hash)
        }

        fn write_stream(
            &self,
            kind: Kind,
            size: u64,
            from: impl Read,
            hash: HashKind,
        ) -> Result<owned::Id, Self::Error> {
            self.loose.write_stream(kind, size, from, hash)
        }
    }
}

mod init {
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
            let packs = std::fs::read_dir(loose_objects.join("packs"))
                .map(|entries| {
                    entries
                        .filter_map(Result::ok)
                        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
                        .map(|e| e.path().to_owned())
                        .filter(|p| p.extension().unwrap_or_default() == "idx" && p.starts_with("pack-"))
                        .map(pack::Bundle::at)
                        .collect::<Result<Vec<_>, _>>()
                })
                .unwrap_or_else(|_read_dir_io_err| Ok(Vec::new()))?;

            Ok(compound::Db {
                loose: loose::Db::at(loose_objects),
                packs,
            })
        }
    }
}
