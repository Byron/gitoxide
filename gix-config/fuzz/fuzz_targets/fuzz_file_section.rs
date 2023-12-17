#![no_main]

use arbitrary::Arbitrary;
use bstr::BStr;
use gix_config::file::{Metadata, Section};
use libfuzzer_sys::fuzz_target;
use std::borrow::Cow;
use std::hint::black_box;

#[derive(Arbitrary, Debug)]
struct Ctx<'a> {
    name: Cow<'a, str>,
    subsection: Option<&'a [u8]>,
    #[arbitrary(default)]
    meta: Metadata,
    value_key: &'a str,
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
    let section = unwrap_or_return!(Section::new(
        ctx.name.clone(),
        ctx.subsection.map(|x| Cow::from(BStr::new(x))),
        ctx.meta.clone(),
    ));
    _ = black_box(section.values(ctx.value_key));
    for key in section.keys() {
        _ = black_box(section.value(key).expect("The key exists, so should the value."));
    }
    _ = black_box(section.to_bstring());
});
