#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    drop(gix_refspec::parse(data.into(), gix_refspec::parse::Operation::Push));
    drop(gix_refspec::parse(data.into(), gix_refspec::parse::Operation::Fetch));
});
