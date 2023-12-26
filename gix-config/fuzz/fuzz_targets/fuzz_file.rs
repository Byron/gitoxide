#![no_main]

use anyhow::Result;
use arbitrary::Arbitrary;
use bstr::BStr;
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
    section_subsection_key_triples: Vec<(&'a str, Option<&'a [u8]>, &'a str)>,
}

const DEFAULT_TRIPLE: (&str, Option<&'static [u8]>, &str) = ("section", Some(b"subsection"), "key");

fn fuzz(ctx: Ctx) -> Result<()> {
    let meta = Metadata::default();
    let options = Options::default();
    let file = File::from_bytes_no_includes(&ctx.input, meta.clone(), options.clone())?;

    let mut triples = ctx.section_subsection_key_triples.iter();

    let (section_name, subsection_name, key) = triples.next().unwrap_or(&DEFAULT_TRIPLE);
    _ = black_box(file.string(section_name, subsection_name.map(|x| BStr::new(x)), key));
    _ = black_box(file.string_by_key(BStr::new(key)));
    _ = black_box(file.string_filter(section_name, subsection_name.map(|x| BStr::new(x)), key, &mut |_| false));
    _ = black_box(file.string_filter_by_key(BStr::new(key), &mut |_| false));

    let (section_name, subsection_name, key) = triples.next().unwrap_or(&DEFAULT_TRIPLE);
    _ = black_box(file.path(section_name, subsection_name.map(|x| BStr::new(x)), key));
    _ = black_box(file.path_by_key(BStr::new(key)));
    _ = black_box(file.path_filter(section_name, subsection_name.map(|x| BStr::new(x)), key, &mut |_| false));
    _ = black_box(file.path_filter_by_key(BStr::new(key), &mut |_| false));

    let (section_name, subsection_name, key) = triples.next().unwrap_or(&DEFAULT_TRIPLE);
    _ = black_box(file.boolean(section_name, subsection_name.map(|x| BStr::new(x)), key));
    _ = black_box(file.boolean_by_key(BStr::new(key)));
    _ = black_box(file.boolean_filter(section_name, subsection_name.map(|x| BStr::new(x)), key, &mut |_| false));
    _ = black_box(file.boolean_filter_by_key(BStr::new(key), &mut |_| false));

    let (section_name, subsection_name, key) = triples.next().unwrap_or(&DEFAULT_TRIPLE);
    _ = black_box(file.integer(section_name, subsection_name.map(|x| BStr::new(x)), key));
    _ = black_box(file.integer_by_key(BStr::new(key)));
    _ = black_box(file.integer_filter(section_name, subsection_name.map(|x| BStr::new(x)), key, &mut |_| false));
    _ = black_box(file.integer_filter_by_key(BStr::new(key), &mut |_| false));

    let (section_name, subsection_name, key) = triples.next().unwrap_or(&DEFAULT_TRIPLE);
    _ = black_box(file.strings(section_name, subsection_name.map(|x| BStr::new(x)), key));
    _ = black_box(file.strings_by_key(BStr::new(key)));
    _ = black_box(file.strings_filter(section_name, subsection_name.map(|x| BStr::new(x)), key, &mut |_| false));
    _ = black_box(file.strings_filter_by_key(BStr::new(key), &mut |_| false));

    let (section_name, subsection_name, key) = triples.next().unwrap_or(&DEFAULT_TRIPLE);
    _ = black_box(file.integers(section_name, subsection_name.map(|x| BStr::new(x)), key));
    _ = black_box(file.integers_by_key(BStr::new(key)));
    _ = black_box(file.integers_filter(section_name, subsection_name.map(|x| BStr::new(x)), key, &mut |_| false));
    _ = black_box(file.integers_filter_by_key(BStr::new(key), &mut |_| false));

    let (section_name, subsection_name, key) = triples.next().unwrap_or(&DEFAULT_TRIPLE);
    _ = black_box(file.integers(section_name, subsection_name.map(|x| BStr::new(x)), key));
    _ = black_box(file.integers_by_key(BStr::new(key)));
    _ = black_box(file.integers_filter(section_name, subsection_name.map(|x| BStr::new(x)), key, &mut |_| false));
    _ = black_box(file.integers_filter_by_key(BStr::new(key), &mut |_| false));

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
    _ = black_box(File::from_bytes_no_includes(
        &roundtrip_as_string,
        meta.clone(),
        options.clone(),
    ))?;
    Ok(())
}

fuzz_target!(|ctx: Ctx| {
    _ = black_box(fuzz(ctx));
});
