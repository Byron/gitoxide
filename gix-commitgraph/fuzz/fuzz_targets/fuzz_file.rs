#![no_main]

use anyhow::Result;
use arbitrary::Arbitrary;
use gix_commitgraph::File;
use libfuzzer_sys::fuzz_target;
use std::fs;
use std::hint::black_box;
use tempfile::NamedTempFile;

fn fuzz(data: &[u8]) -> Result<()> {
    let named_temp_file = NamedTempFile::new()?;
    fs::write(named_temp_file.path(), data).expect("Unable to write fuzzed file");
    let file = File::try_from(named_temp_file.path())?;

    _ = black_box(file.iter_base_graph_ids().count());
    _ = black_box(file.iter_commits().count());
    _ = black_box(file.iter_ids().count());

    let _ = black_box(file.checksum());
    let _ = black_box(file.verify_checksum());
    let _ = black_box(file.object_hash());

    Ok(())
}

fuzz_target!(|data: &[u8]| {
    _ = black_box(fuzz(data));
});
