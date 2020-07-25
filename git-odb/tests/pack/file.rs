use crate::fixture_path;
use git_odb::pack;
use std::convert::TryFrom;

fn pack_at(at: &str) -> pack::data::File {
    pack::data::File::try_from(fixture_path(at).as_path()).unwrap()
}

mod method {
    use crate::pack::file::pack_at;
    use crate::pack::SMALL_PACK;

    #[test]
    fn checksum() {
        let p = pack_at(SMALL_PACK);
        assert_eq!(
            hex::encode(p.checksum().sha1()),
            "0f3ea84cd1bba10c2a03d736a460635082833e59"
        );
    }

    #[test]
    fn verify_checksum() {
        let p = pack_at(SMALL_PACK);
        assert_eq!(p.verify_checksum().unwrap(), p.checksum());
    }
}

/// All hardcoded offsets are obtained via `git pack-verify --verbose  tests/fixtures/packs/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.idx`
mod decode_entry {
    use crate::{fixture_path, fixup, pack::file::pack_at, pack::SMALL_PACK};
    use bstr::ByteSlice;
    use git_object::borrowed;
    use git_odb::pack::{cache, data::decode::ResolvedBase};

    fn content_of(path: &str) -> Vec<u8> {
        fixup(std::fs::read(fixture_path(path)).unwrap())
    }

    #[test]
    fn commit() {
        let buf = decode_entry_at_offset(1968);
        assert_eq!(buf.len(), 187);
        assert_eq!(
            buf.capacity(),
            187,
            "for undeltified objects, there is no change in allocation or resizing"
        );
    }

    #[test]
    fn blob_ofs_delta_two_links() {
        let buf = decode_entry_at_offset(3033);
        assert_eq!(buf.len(), 173, "buffer length is the acutal object size");
        assert_eq!(buf.capacity(), 2381, "capacity is much higher as we allocate everything into a single, bigger, reusable buffer, which depends on base sizes");
        assert_eq!(
            buf.as_bstr(),
            content_of("objects/b8aa61be84b78d7fcff788e8d844406cc97132bf.txt").as_bstr()
        );
    }

    #[test]
    fn blob_ofs_delta_single_link() {
        let buf = decode_entry_at_offset(3569);
        assert_eq!(buf.len(), 1163, "buffer length is the acutal object size");
        assert_eq!(buf.capacity(),2398, "capacity is much higher as we allocate everything into a single, bigger, reusable buffer, which depends on base sizes");
        assert_eq!(
            buf.as_bstr(),
            content_of("objects/f139391424a8c623adadf2388caec73e5e90865b.txt").as_bstr()
        );
    }

    fn decode_entry_at_offset(offset: u64) -> Vec<u8> {
        fn resolve_with_panic(_oid: borrowed::Id, _out: &mut Vec<u8>) -> Option<ResolvedBase> {
            panic!("should not want to resolve an id here")
        }

        let p = pack_at(SMALL_PACK);
        let entry = p.entry(offset);
        let mut buf = Vec::new();
        p.decode_entry(entry, &mut buf, resolve_with_panic, &mut cache::DecodeEntryNoop)
            .unwrap();
        buf
    }
}

mod decompress_entry {
    use crate::{pack::file::pack_at, pack::SMALL_PACK};
    use git_object::bstr::ByteSlice;

    #[test]
    fn commit() {
        let buf = decompress_entry_at_offset(1968);
        assert_eq!(buf.as_bstr(), b"tree e90926b07092bccb7bf7da445fae6ffdfacf3eae\nauthor Sebastian Thiel <byronimo@gmail.com> 1286529993 +0200\ncommitter Sebastian Thiel <byronimo@gmail.com> 1286529993 +0200\n\nInitial commit\n".as_bstr());
        assert_eq!(buf.len(), 187)
    }

    #[test]
    fn blob() {
        let buf = decompress_entry_at_offset(2142);
        assert_eq!(
            buf.as_bstr(),
            b"GitPython is a python library used to interact with Git repositories.\n\nHi there\n\nHello Other\n"
                .as_bstr()
        );
        assert_eq!(buf.len(), 93)
    }

    #[test]
    fn blob_with_two_chain_links() {
        let buf = decompress_entry_at_offset(3033);
        assert_eq!(buf.len(), 6, "it decompresses delta objects, but won't resolve them")
    }

    #[test]
    fn tree() {
        let buf = decompress_entry_at_offset(2097);
        assert_eq!(buf[..13].as_bstr(), b"100644 README".as_bstr());
        assert_eq!(buf.len(), 34);
        assert_eq!(
            buf.capacity(),
            34,
            "capacity must be controlled by the caller to be big enough"
        );
    }

    fn decompress_entry_at_offset(offset: u64) -> Vec<u8> {
        let p = pack_at(SMALL_PACK);
        let entry = p.entry(offset);

        let size = entry.decompressed_size as usize;
        let mut buf = Vec::with_capacity(size);
        buf.resize(size, 0);

        p.decompress_entry(&entry, &mut buf).unwrap();

        buf.resize(entry.decompressed_size as usize, 0);
        buf
    }
}
