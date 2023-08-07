use std::collections::HashMap;

use bstr::{BStr, BString, ByteSlice};
use gix_attributes::State;
use gix_pathspec::{MagicSignature, MatchMode, Pattern};
use once_cell::sync::Lazy;

#[test]
fn baseline() {
    for (pattern, exit_code) in BASELINE.iter() {
        let res = gix_pathspec::parse(pattern);
        assert_eq!(
            res.is_ok(),
            *exit_code == 0,
            "{pattern:?} disagrees with baseline: {res:?}"
        )
    }
}

mod invalid;
mod valid;

/// A way to specify expectations more easily by simplifying assignments.
#[derive(Debug, Clone, PartialEq, Eq)]
struct NormalizedPattern {
    path: BString,
    signature: MagicSignature,
    search_mode: MatchMode,
    attributes: Vec<(BString, State)>,
}

impl From<Pattern> for NormalizedPattern {
    fn from(p: Pattern) -> Self {
        NormalizedPattern {
            path: p.path,
            signature: p.signature,
            search_mode: p.search_mode,
            attributes: p
                .attributes
                .into_iter()
                .map(|attr| (attr.name.as_str().into(), attr.state))
                .collect(),
        }
    }
}

static BASELINE: Lazy<HashMap<BString, usize>> = Lazy::new(|| {
    let base = gix_testtools::scripted_fixture_read_only("parse_baseline.sh").unwrap();

    (|| -> crate::Result<_> {
        let mut map = HashMap::new();
        let baseline = std::fs::read(base.join("baseline.git"))?;
        let mut lines = baseline.lines();
        while let Some(spec) = lines.next() {
            let exit_code = lines.next().expect("two lines per baseline").to_str()?.parse()?;
            map.insert(spec.into(), exit_code);
        }
        Ok(map)
    })()
    .unwrap()
});

fn check_valid_inputs<'a>(inputs: impl IntoIterator<Item = (&'a str, NormalizedPattern)>) {
    for (input, expected) in inputs.into_iter() {
        assert!(
            check_against_baseline(input),
            "This pathspec is invalid in git: {input}"
        );

        let pattern: NormalizedPattern = gix_pathspec::parse(input.as_bytes())
            .unwrap_or_else(|_| panic!("parsing should not fail with pathspec {input}"))
            .into();
        assert_eq!(pattern, expected, "while checking input: \"{input}\"");
    }
}

fn check_against_baseline(pathspec: &str) -> bool {
    let key: &BStr = pathspec.into();
    let base = BASELINE
        .get(key)
        .unwrap_or_else(|| panic!("missing baseline for pathspec: {pathspec:?}"));
    *base == 0
}
