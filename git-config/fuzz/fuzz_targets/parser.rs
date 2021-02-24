#![no_main]

use git_config::parser::Parser;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    Parser::from_bytes(data);
});
