use crate::{fixture_path, pack::SMALL_PACK};
use git_odb::pack;
use std::fs;

#[test]
fn new_from_header() {
    let (kind, num_objects, iter) =
        pack::data::Iter::new_from_header(fs::File::open(fixture_path(SMALL_PACK)).unwrap())
            .unwrap()
            .unwrap();
    assert_eq!(kind, pack::data::Kind::V2);
    assert_eq!(num_objects, 42);
    assert_eq!(num_objects as usize, iter.count());
}
