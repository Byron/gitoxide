#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    drop(git_refspec::parse(data.into(), git_refspec::parse::Operation::Push));
    drop(git_refspec::parse(data.into(), git_refspec::parse::Operation::Fetch));
});
