use crate::fixture_path;
use git_odb::pack;
use std::convert::TryFrom;

fn new_pack(at: &str) -> pack::File {
    pack::File::try_from(fixture_path(at).as_path()).unwrap()
}

mod decode_entry {
    use crate::pack::file::new_pack;
    use crate::pack::SMALL_PACK;
    use bstr::ByteSlice;

    #[test]
    fn decode_commit() {
        let p = new_pack(SMALL_PACK);
        let entry = p.entry(1968);
        let mut buf = Vec::with_capacity(entry.size as usize + 20); // simulate slightly bigger buffers, for fun
        buf.resize(entry.size as usize + 20, 0);
        p.decode_entry(&entry, &mut buf);
        assert_eq!(buf[..entry.size as usize].as_bstr(), b"tree e90926b07092bccb7bf7da445fae6ffdfacf3eae\nauthor Sebastian Thiel <byronimo@gmail.com> 1286529993 +0200\ncommitter Sebastian Thiel <byronimo@gmail.com> 1286529993 +0200\n\nInitial commit\n".as_bstr());
    }
}
