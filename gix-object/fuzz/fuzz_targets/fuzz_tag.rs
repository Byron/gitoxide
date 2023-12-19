#![no_main]

use libfuzzer_sys::fuzz_target;
use std::hint::black_box;

fuzz_target!(|tag: &[u8]| {
    _ = black_box(gix_object::TagRef::from_bytes(tag));
    _ = black_box(gix_object::TagRefIter::from_bytes(tag).count());
});
