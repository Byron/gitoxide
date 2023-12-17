#![no_main]
use libfuzzer_sys::fuzz_target;
use std::hint::black_box;

fuzz_target!(|commit: &[u8]| {
    _ = black_box(gix_object::CommitRef::from_bytes(commit));
    _ = black_box(gix_object::CommitRefIter::from_bytes(commit)).count();
});
