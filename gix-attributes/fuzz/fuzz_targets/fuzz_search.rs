#![no_main]

use anyhow::Result;
use libfuzzer_sys::fuzz_target;

use std::hint::black_box;
use std::path::Path;

use arbitrary::{Arbitrary, Unstructured};
use gix_attributes::{
    search::{MetadataCollection, Outcome},
    Search,
};
use gix_glob::pattern::Case;

fn arbitrary_case(u: &mut Unstructured) -> arbitrary::Result<Case> {
    Ok(*u.choose(&[Case::Sensitive, Case::Fold])?)
}

fn arbitrary_relative_path<'a>(u: &mut Unstructured<'a>) -> arbitrary::Result<&'a str> {
    let mut path = <&str>::arbitrary(u)?;
    if path.starts_with("/") {
        path = path.trim_start_matches("/");
    }
    Ok(path)
}

#[derive(Debug, Arbitrary)]
struct Ctx<'a> {
    pattern: &'a str,
    #[arbitrary(with = arbitrary_relative_path)]
    path: &'a str,
    relative_containing_dir: Option<&'a str>,
    #[arbitrary(with = arbitrary_case)]
    case: Case,
}

fn fuzz(ctx: Ctx) -> Result<()> {
    let Ctx {
        pattern,
        path,
        relative_containing_dir,
        case,
    } = ctx;

    let mut search = Search::default();
    let mut collection = MetadataCollection::default();
    search.add_patterns_buffer(
        format!("{pattern} test").as_bytes(),
        relative_containing_dir.map_or_else(|| Path::new("<memory>").into(), |d| Path::new(d).join("filename")),
        relative_containing_dir.map(|_| Path::new("")),
        &mut collection,
        true,
    );
    let mut out = Outcome::default();
    out.initialize(&collection);
    _ = black_box(search.pattern_matching_relative_path(path.into(), case, None, &mut out));
    Ok(())
}

fuzz_target!(|ctx: Ctx| {
    _ = black_box(fuzz(ctx));
});
