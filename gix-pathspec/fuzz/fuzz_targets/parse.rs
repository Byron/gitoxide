#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _a = gix_pathspec::parse(data, Default::default());
});
