use crate::fixture_path;
use git_odb::pack;
use std::convert::TryFrom;

fn new_pack(at: &str) -> pack::File {
    pack::File::try_from(fixture_path(at).as_path()).unwrap()
}

/// All hardcoded offsets are obtained via `git verify-pack --verbose  tests/fixtures/packs/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.idx`
mod decode_entry {
    use crate::{pack::file::new_pack, pack::SMALL_PACK};
    use bstr::ByteSlice;
    use git_odb::pack::ResolvedBase;

    #[test]
    fn commit() {
        let buf = decompress_entry_at_offset(1968);
        assert_eq!(buf.as_bstr(), b"tree e90926b07092bccb7bf7da445fae6ffdfacf3eae\nauthor Sebastian Thiel <byronimo@gmail.com> 1286529993 +0200\ncommitter Sebastian Thiel <byronimo@gmail.com> 1286529993 +0200\n\nInitial commit\n".as_bstr());
        assert_eq!(buf.len(), 187)
    }

    #[test]
    fn blob() {
        let buf = decompress_entry_at_offset(2142);
        assert_eq!(buf.as_bstr(), b"GitPython is a python library used to interact with Git repositories.\n\nHi there\n\nHello Other\n".as_bstr());
        assert_eq!(buf.len(), 93)
    }

    #[test]
    fn tree() {
        let buf = decompress_entry_at_offset(2097);
        assert_eq!(buf[..13].as_bstr(), b"100644 README".as_bstr());
        assert_eq!(buf.len(), 34)
    }

    fn resolve_with_panic(_oid: &git_object::Id, _out: &mut Vec<u8>) -> ResolvedBase {
        panic!("should not want to resolve an id here")
    }

    #[test]
    fn blob_ofs_delta_two_links() {
        let p = new_pack(SMALL_PACK);
        let entry = p.entry(3033);
        let mut buf = Vec::new();
        p.decode_entry(entry, &mut buf, resolve_with_panic).unwrap();
        assert_eq!(buf.len(), 173, "buffer length is the acutal object size");
        assert_eq!(buf.capacity(), 2381, "capacity is much higher as we allocate everything into a single, bigger, reusable buffer, which depends on base sizes");
        assert_eq!(buf.as_bstr(), b"GitPython is a python library used to interact with Git repositories.\n\nGitPython is a port of the grit library in Ruby created by \nTom Preston-Werner and Chris Wanstrath.\n\n\n".as_bstr());
    }

    #[test]
    fn blob_ofs_delta_single_link() {
        let p = new_pack(SMALL_PACK);
        let entry = p.entry(3569);
        let mut buf = Vec::new();
        p.decode_entry(entry, &mut buf, resolve_with_panic).unwrap();
        assert_eq!(buf.len(), 1163, "buffer length is the acutal object size");
        assert_eq!(buf.capacity(),2398, "capacity is much higher as we allocate everything into a single, bigger, reusable buffer, which depends on base sizes");
        assert_eq!(buf[..102].as_bstr(), b"==========\nGitPython\n==========\n\nGitPython is a python library used to interact with Git repositories.".as_bstr());
    }

    fn decompress_entry_at_offset(offset: u64) -> Vec<u8> {
        let p = new_pack(SMALL_PACK);
        let entry = p.entry(offset);

        let mut buf = Vec::with_capacity(entry.size as usize);
        buf.resize(entry.size as usize, 0);

        p.decompress_entry(&entry, &mut buf).unwrap();

        buf.resize(entry.size as usize, 0);
        buf
    }
}
