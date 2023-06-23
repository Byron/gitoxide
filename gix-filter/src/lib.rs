//! A library for implementing everything needed to deal with git filter pipelines.
//!
//! Generally, multiple filters are applied in a row forming a pipeline, with each filter being a stage in that pipeline.
//! This pipeline is pre-determined with each stage being configurable.
//!
//! The transformation on an input buffer goes in two ways: either a filter is applied, or its effects are undone. Differentiating
//! between these states is important to avoid comparing unfiltered buffers with filtered ones, for example.
//!
//! This crate implements the building blocks in terms of applying and undoing filters, along with logic to decide whether
//! or not to apply such a filter.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

///
pub mod ident {
    use bstr::{BStr, ByteSlice, ByteVec};
    use std::borrow::Cow;
    use std::ops::Range;

    /// Undo identifiers like `$Id:<hexsha>$` to `$Id$`. Newlines between dollars are ignored.
    pub fn undo(mut input: Cow<'_, BStr>) -> Cow<'_, BStr> {
        fn find_range(input: &[u8]) -> Option<Range<usize>> {
            let mut ofs = 0;
            loop {
                let mut cursor = input.get(ofs..)?;
                let start = cursor.find(b"$Id:")?;
                cursor = cursor.get((start + 4)..)?;
                let maybe_end = cursor.find_byteset(b"$\n")?;
                if cursor[maybe_end] == b'\n' {
                    ofs += start + 4 + maybe_end + 1;
                    continue;
                } else {
                    return Some((ofs + start)..(ofs + start + 4 + maybe_end + 1));
                }
            }
        }

        let mut ofs = 0;
        while let Some(range) = find_range(&input[ofs..]) {
            input
                .to_mut()
                .replace_range((range.start + ofs)..(range.end + ofs), b"$Id$");
            ofs += range.start + 4;
        }
        input
    }
}
