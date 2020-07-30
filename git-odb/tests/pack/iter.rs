use crate::{fixture_path, pack::SMALL_PACK};
use git_odb::pack;
use std::fs;

#[test]
fn new_from_header() -> Result<(), Box<dyn std::error::Error>> {
    for should_verify in &[false, true] {
        let iter = pack::data::Iter::new_from_header(
            std::io::BufReader::new(fs::File::open(fixture_path(SMALL_PACK))?),
            *should_verify,
        )?;

        let num_objects = iter.len();
        assert_eq!(iter.kind(), pack::data::Kind::V2);
        assert_eq!(iter.len(), 42);
        assert_eq!(iter.count(), num_objects);
    }
    Ok(())
}
