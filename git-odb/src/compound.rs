use crate::{loose, pack};

pub struct Db {
    pub loose: loose::Db,
    pub packs: Vec<pack::Bundle>,
}

mod locate {
    use crate::{compound, loose, pack};
    use git_object::borrowed;
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Loose(err: loose::db::locate::Error) {
                display("An error occurred while obtaining an object from the loose object store")
                source(err)
                from()
            }
            Pack(err: pack::bundle::locate::Error) {
                display("An error occurred while obtaining an object from the packed object store")
                source(err)
                from()
            }
        }
    }

    pub enum Object<'a> {
        Loose(loose::Object),
        Borrowed(crate::borrowed::Object<'a>),
    }

    impl compound::Db {
        pub fn locate<'a>(&self, _id: borrowed::Id, _buffer: &'a mut Vec<u8>) -> Option<Result<Object<'a>, Error>> {
            unimplemented!("object location")
        }
    }
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
            let packs = if let Ok(entries) = std::fs::read_dir(loose_objects.join("packs")) {
                let mut packs_and_sizes = entries
                    .filter_map(Result::ok)
                    .filter_map(|e| e.metadata().map(|md| (e.path().to_owned(), md)).ok())
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
}
