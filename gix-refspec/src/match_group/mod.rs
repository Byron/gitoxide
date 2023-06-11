use std::collections::BTreeSet;

use crate::{parse::Operation, types::Mode, MatchGroup, RefSpecRef};

pub(crate) mod types;
pub use types::{Item, Mapping, Outcome, Source, SourceRef};

///
pub mod validate;

/// Initialization
impl<'a> MatchGroup<'a> {
    /// Take all the fetch ref specs from `specs` get a match group ready.
    pub fn from_fetch_specs(specs: impl IntoIterator<Item = RefSpecRef<'a>>) -> Self {
        MatchGroup {
            specs: specs.into_iter().filter(|s| s.op == Operation::Fetch).collect(),
        }
    }
}

/// Matching
impl<'a> MatchGroup<'a> {
    /// Match all `items` against all fetch specs present in this group, returning deduplicated mappings from source to destination.
    /// Note that this method only makes sense if the specs are indeed fetch specs and may panic otherwise.
    ///
    /// Note that negative matches are not part of the return value, so they are not observable but will be used to remove mappings.
    pub fn match_remotes<'item>(self, mut items: impl Iterator<Item = Item<'item>> + Clone) -> Outcome<'a, 'item> {
        let mut out = Vec::new();
        let mut seen = BTreeSet::default();
        let mut push_unique = |mapping| {
            if seen.insert(calculate_hash(&mapping)) {
                out.push(mapping);
            }
        };
        let mut matchers: Vec<Option<Matcher<'_>>> = self
            .specs
            .iter()
            .copied()
            .map(Matcher::from)
            .enumerate()
            .map(|(idx, m)| match m.lhs {
                Some(Needle::Object(id)) => {
                    push_unique(Mapping {
                        item_index: None,
                        lhs: SourceRef::ObjectId(id),
                        rhs: m.rhs.map(Needle::to_bstr),
                        spec_index: idx,
                    });
                    None
                }
                _ => Some(m),
            })
            .collect();

        let mut has_negation = false;
        for (spec_index, (spec, matcher)) in self.specs.iter().zip(matchers.iter_mut()).enumerate() {
            for (item_index, item) in items.clone().enumerate() {
                if spec.mode == Mode::Negative {
                    has_negation = true;
                    continue;
                }
                if let Some(matcher) = matcher {
                    let (matched, rhs) = matcher.matches_lhs(item);
                    if matched {
                        push_unique(Mapping {
                            item_index: Some(item_index),
                            lhs: SourceRef::FullName(item.full_ref_name),
                            rhs,
                            spec_index,
                        })
                    }
                }
            }
        }

        if let Some(id) = has_negation.then(|| items.next().map(|i| i.target)).flatten() {
            let null_id = gix_hash::ObjectId::null(id.kind());
            for matcher in matchers
                .into_iter()
                .zip(self.specs.iter())
                .filter_map(|(m, spec)| m.and_then(|m| (spec.mode == Mode::Negative).then_some(m)))
            {
                out.retain(|m| match m.lhs {
                    SourceRef::ObjectId(_) => true,
                    SourceRef::FullName(name) => {
                        !matcher
                            .matches_lhs(Item {
                                full_ref_name: name,
                                target: &null_id,
                                object: None,
                            })
                            .0
                    }
                });
            }
        }
        Outcome {
            group: self,
            mappings: out,
        }
    }
}

fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    use std::hash::Hasher;
    let mut s = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

mod util;
use util::{Matcher, Needle};
