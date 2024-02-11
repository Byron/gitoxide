use crate::walk_utils::{collect, entry, fixture, options};
use gix_dir::entry::Kind::File;
use gix_dir::entry::Status::Untracked;
use gix_dir::walk;
use std::path::Path;

pub mod walk_utils;

#[test]
fn prefixes_work_as_expected() -> gix_testtools::Result {
    let root = fixture("only-untracked");
    std::env::set_current_dir(root.join("d"))?;
    let (out, entries) = collect(&root, |keep, ctx| {
        walk(&Path::new("..").join("d"), Path::new(".."), ctx, options(), keep)
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 2,
            returned_entries: entries.len(),
            seen_entries: 3,
        }
    );
    assert_eq!(
        &entries,
        &[
            entry("d/a", Untracked, File),
            entry("d/b", Untracked, File),
            entry("d/d/a", Untracked, File),
        ]
    );
    Ok(())
}
