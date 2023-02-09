#![no_main]

use git_date;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    git_date::parse(data, None).ok();
});
