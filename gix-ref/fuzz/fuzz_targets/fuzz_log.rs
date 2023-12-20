#![no_main]

use anyhow::Result;
use gix_ref::file::log;
use libfuzzer_sys::fuzz_target;
use std::hint::black_box;

fn fuzz(line: &[u8]) -> Result<()> {
    let line = log::LineRef::from_bytes(line)?;
    _ = black_box(line.previous_oid());
    _ = black_box(line.new_oid());
    Ok(())
}

fuzz_target!(|ctx: &[u8]| {
    _ = black_box(fuzz(ctx));
});
