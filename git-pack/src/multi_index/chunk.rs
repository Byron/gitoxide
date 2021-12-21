pub mod index_names {
    use git_object::bstr::{BString, ByteSlice};
    use os_str_bytes::OsStrBytes;
    use std::path::{Path, PathBuf};

    pub const ID: git_chunk::Id = *b"PNAM";

    pub mod from_slice {
        use git_object::bstr::BString;

        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error("The pack names were not ordered alphabetically.")]
            NotOrderedAlphabetically,
            #[error("Each pack path name must be terminated with a null byte")]
            MissingNullByte,
            #[error("Couldn't turn path '{path}' into OS path due to encoding issues")]
            PathEncoding { path: BString },
        }
    }

    pub fn from_slice(mut chunk: &[u8], num_packs: u32) -> Result<Vec<PathBuf>, from_slice::Error> {
        let mut out = Vec::new();
        for _ in 0..num_packs {
            let null_byte_pos = chunk.find_byte(b'\0').ok_or(from_slice::Error::MissingNullByte)?;

            let path = &chunk[..null_byte_pos];
            let path = Path::from_raw_bytes(path)
                .map_err(|_| from_slice::Error::PathEncoding {
                    path: BString::from(path),
                })?
                .into_owned();

            if let Some(previous) = out.last() {
                if previous >= &path {
                    return Err(from_slice::Error::NotOrderedAlphabetically);
                }
            }
            out.push(path);

            chunk = &chunk[null_byte_pos + 1..];
        }

        // NOTE: git writes garbage into this chunk, usually extra \0 bytes, which we simply ignore. If we were strict
        // about it we couldn't read this chunk data at all.
        Ok(out)
    }
}

pub mod fanout {
    use std::convert::TryInto;

    pub const ID: git_chunk::Id = *b"OIDF";

    pub fn from_slice(chunk: &[u8]) -> Option<[u32; 256]> {
        if chunk.len() != 4 * 256 {
            return None;
        }
        let mut out = [0; 256];
        for (c, f) in chunk.chunks(4).zip(out.iter_mut()) {
            *f = u32::from_be_bytes(c.try_into().unwrap());
        }
        out.into()
    }
}

pub mod lookup {
    use git_chunk::file::Offset;
    use git_hash::Kind;
    use std::ops::Range;

    pub const ID: git_chunk::Id = *b"OIDL";

    pub fn is_valid(offset: &Range<usize>, hash: git_hash::Kind, num_objects: u32) -> bool {
        (offset.end - offset.start) / hash.len_in_bytes() == num_objects as usize
    }
}

pub mod offsets {
    use git_chunk::file::Offset;
    use std::ops::Range;

    pub const ID: git_chunk::Id = *b"OOFF";

    pub fn is_valid(offset: &Range<usize>, num_objects: u32) -> bool {
        let entry_size = 4 /* pack-id */ + 4 /* pack-offset */;
        ((offset.end - offset.start) / num_objects as usize) == entry_size
    }
}

pub mod large_offsets {
    use git_chunk::file::Offset;
    use std::ops::Range;

    pub const ID: git_chunk::Id = *b"LOFF";
    pub fn is_valid(offset: &Range<usize>) -> bool {
        (offset.end - offset.start) % 8 == 0
    }
}
