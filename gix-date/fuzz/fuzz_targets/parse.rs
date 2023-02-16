#![no_main]

use gix_date;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    gix_date::parse(data, None).ok();
});
