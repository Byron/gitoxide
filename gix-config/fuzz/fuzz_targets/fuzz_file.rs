#![no_main]

use arbitrary::Arbitrary;
use gix_config::{
    file::{init::Options, Metadata},
    File,
};
use libfuzzer_sys::fuzz_target;
use std::hint::black_box;

#[derive(Arbitrary, Debug)]
struct Ctx<'a> {
    input: &'a [u8],
    sections_by_name: &'a str,
}

macro_rules! unwrap_or_return {
    ($e:expr) => {
        match $e {
            Ok(val) => val,
            Err(_) => return,
        }
    };
}

fuzz_target!(|ctx: Ctx| {
    let meta = Metadata::default();
    let options = Options::default();
    let file = unwrap_or_return!(File::from_bytes_no_includes(&ctx.input, meta.clone(), options.clone()));
    _ = black_box(file.sections().count());
    _ = black_box(file.sections_and_ids().count());
    _ = black_box(file.sections_and_postmatter().count());
    _ = black_box(file.sections_by_name(ctx.sections_by_name).map(|x| x.count()));
    _ = black_box(file.frontmatter());

    for section in file.sections() {
        for key in section.keys() {
            _ = black_box(
                section
                    .value_implicit(key.as_ref())
                    .expect("The key exists, so should the value."),
            );
        }
    }

    let roundtrip_as_string: Vec<u8> = file.to_bstring().into();
    _ = unwrap_or_return!(black_box(File::from_bytes_no_includes(
        &roundtrip_as_string,
        meta.clone(),
        options.clone(),
    )));
});
