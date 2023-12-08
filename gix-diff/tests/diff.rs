use gix_testtools::Result;

fn hex_to_id(hex: &str) -> gix_hash::ObjectId {
    gix_hash::ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

mod blob;
mod rewrites;
mod tree;

mod util {
    use std::collections::HashMap;

    use gix_hash::oid;
    use gix_object::{bstr::BString, find::Error};

    #[derive(Default)]
    pub struct ObjectDb {
        data_by_id: HashMap<gix_hash::ObjectId, BString>,
    }

    impl gix_object::FindHeader for ObjectDb {
        fn try_header(&self, id: &oid) -> Result<Option<gix_object::Header>, Error> {
            match self.data_by_id.get(&id.to_owned()) {
                Some(data) => Ok(Some(gix_object::Header {
                    kind: gix_object::Kind::Blob,
                    size: data.len() as u64,
                })),
                None => Ok(None),
            }
        }
    }

    impl gix_object::Find for ObjectDb {
        fn try_find<'a>(&self, id: &oid, buffer: &'a mut Vec<u8>) -> Result<Option<gix_object::Data<'a>>, Error> {
            match self.data_by_id.get(&id.to_owned()) {
                Some(data) => {
                    buffer.resize(data.len(), 0);
                    buffer.copy_from_slice(data);
                    Ok(Some(gix_object::Data {
                        kind: gix_object::Kind::Blob,
                        data: buffer.as_slice(),
                    }))
                }
                None => Ok(None),
            }
        }
    }

    impl ObjectDb {
        /// Insert `data` and return its hash. That can be used to find it again.
        pub fn insert(&mut self, data: &str) -> gix_hash::ObjectId {
            let id = gix_object::compute_hash(gix_hash::Kind::Sha1, gix_object::Kind::Blob, data.as_bytes());
            self.data_by_id.insert(id, data.into());
            id
        }
    }
}
