use crate::{fixture_path, pack::SMALL_PACK};
use git_odb::pack;
use git_odb::pack::data::iter::TrailerMode;
use std::fs;

#[test]
fn size_of_entry() {
    assert_eq!(
        std::mem::size_of::<pack::data::iter::Entry>(),
        104,
        "let's keep the size in check as we have many of them"
    );
}

#[test]
fn new_from_header() -> Result<(), Box<dyn std::error::Error>> {
    for trailer_mode in &[TrailerMode::AsIs, TrailerMode::Verify, TrailerMode::Restore] {
        let mut iter = pack::data::Iter::new_from_header(
            std::io::BufReader::new(fs::File::open(fixture_path(SMALL_PACK))?),
            *trailer_mode,
        )?;

        let num_objects = iter.len();
        assert_eq!(iter.kind(), pack::data::Kind::V2);
        assert_eq!(num_objects, 42);
        assert_eq!(iter.by_ref().take(42 - 1).count(), num_objects - 1);
        assert_eq!(iter.len(), 1);
        assert_eq!(
            iter.next().expect("last object")?.trailer.expect("trailer id"),
            pack::data::File::at(fixture_path(SMALL_PACK))?.checksum(),
            "last object contains the trailer - a hash over all bytes in the pack"
        );
        assert_eq!(iter.len(), 0);
    }
    Ok(())
}
