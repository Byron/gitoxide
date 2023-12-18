#![no_main]

use anyhow::Result;
use libfuzzer_sys::fuzz_target;

use std::hint::black_box;

use arbitrary::{Arbitrary, Unstructured};
use gix_attributes::{
    search::{MetadataCollection, Outcome},
    Search,
};
use gix_glob::pattern::Case;

fn arbitrary_case(u: &mut Unstructured) -> arbitrary::Result<Case> {
    Ok(*u.choose(&[Case::Sensitive, Case::Fold])?)
}

#[derive(Debug, Arbitrary)]
struct Ctx<'a> {
    pattern: &'a str,
    #[arbitrary(with = arbitrary_case)]
    case: Case,
}

fn fuzz(Ctx { pattern, case }: Ctx) -> Result<()> {
    let mut search = Search::default();
    let mut collection = MetadataCollection::default();
    search.add_patterns_buffer(
        format!("{pattern} attr").as_bytes(),
        Default::default(),
        None,
        &mut collection,
        true,
    );
    let mut out = Outcome::default();
    out.initialize(&collection);
    _ = black_box(search.pattern_matching_relative_path("relative/path".into(), case, None, &mut out));
    Ok(())
}

fuzz_target!(|ctx: Ctx| {
    _ = black_box(fuzz(ctx));
});
