#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Don't name this _; Rust may optimize it out.
    let _a = gix_config::parse::from_bytes(data, &mut |_e| ());
});
