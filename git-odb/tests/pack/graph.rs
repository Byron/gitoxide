use git_odb::pack::graph;

mod from_sorted_offsets {
    use crate::pack::SMALL_PACK_INDEX;
    use crate::{fixture_path, pack::SMALL_PACK};
    use git_odb::pack;
    use std::{
        convert::TryInto,
        fs,
        io::{BufRead, BufReader},
    };

    #[test]
    fn pack_v2() {
        let mut r = BufReader::new(fs::File::open(fixture_path(SMALL_PACK)).unwrap());
        {
            let buf = r.fill_buf().unwrap();
            pack::data::parse::header(&buf[..12].try_into().unwrap()).unwrap();
            r.consume(12);
        }
        let idx = pack::index::File::at(fixture_path(SMALL_PACK_INDEX)).unwrap();
        let ofs = idx.sorted_offsets();
        pack::graph::DeltaTree::from_sorted_offsets(ofs.into_iter(), r, git_features::progress::Discard);
    }
}

#[test]
fn size() {
    assert_eq!(
        std::mem::size_of::<graph::PackEntryKind>(),
        16,
        "PackEntryKinds must remain small as these trees are up to 10mio objects"
    )
}
