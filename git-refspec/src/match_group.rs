use crate::parse::Operation;
use crate::types::{MatchGroup, Mode};
use crate::RefSpecRef;
use bstr::BStr;
use git_hash::oid;
use git_hash::ObjectId;
use std::borrow::Cow;

/// An item to match, input to various matching operations.
#[derive(Debug, Copy, Clone)]
pub struct Item<'a> {
    /// The full name of the references, like `refs/heads/main`
    pub full_ref_name: &'a BStr,
    /// The peeled id it points to that we should match against.
    pub target: &'a oid,
    /// The tag object's id if this is a tag
    pub tag: Option<&'a oid>,
}

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
    /// Match all `items` against all fetch specs present in this group.
    /// Note that this method only makes sense if the specs are indeed fetch specs and may panic otherwise.
    ///
    /// Note that negative matches are not part of the return value, so they are not observable.
    pub fn match_remotes<'item>(&self, items: impl Iterator<Item = Item<'item>> + Clone) -> Vec<Mapping<'item, 'a>> {
        let mut matchers: Vec<Matcher<'_>> = self.specs.iter().copied().map(Into::into).collect();
        let mut out = Vec::new();
        for (spec_index, (spec, matcher)) in self.specs.iter().zip(matchers.iter_mut()).enumerate() {
            for (item_index, item) in items.clone().enumerate() {
                if spec.mode == Mode::Negative {
                    continue;
                }
                let (matched, rhs) = matcher.matches_lhs(item);
                if matched {
                    out.push(Mapping {
                        item_index,
                        lhs: item.full_ref_name,
                        rhs,
                        spec_index,
                    })
                }
            }
        }
        // TODO: negation subtracts from the entire set, order doesn't matter.
        out
    }

    /// Return the spec that produced the given `mapping`.
    pub fn spec_by_mapping(&self, mapping: &Mapping<'_, '_>) -> RefSpecRef<'a> {
        self.specs[mapping.spec_index]
    }
}

/// A mapping from a remote to a local refs for fetches or local to remote refs for pushes.
///
/// Mappings are like edges in a graph, initially without any constraints.
#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct Mapping<'a, 'b> {
    /// The index into the initial `items` list.
    pub item_index: usize,
    /// The name of the remote side for fetches or the local one for pushes that matched.
    pub lhs: &'a BStr,
    /// The name of the local side for fetches or the remote one for pushes that corresponds to `lhs`, if available.
    pub rhs: Option<Cow<'b, BStr>>,
    /// The index of the matched ref-spec as seen from the match group.
    spec_index: usize,
}

/// A type keeping enough information about a ref-spec to be able to efficiently match it against multiple matcher items.
#[allow(dead_code)]
struct Matcher<'a> {
    lhs: Option<Needle<'a>>,
    rhs: Option<Needle<'a>>,
}

impl<'a> Matcher<'a> {
    /// Match `item` against this spec and return `(true, Some<rhs>)` to gain the other side of the match as configured, or `(true, None)`
    /// if there was no `rhs`.
    ///
    /// This may involve resolving a glob with an allocation, as the destination is built using the matching portion of a glob.
    #[allow(dead_code)]
    pub fn matches_lhs(&self, _item: Item<'_>) -> (bool, Option<Cow<'a, BStr>>) {
        todo!()
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub(crate) enum Needle<'a> {
    FullName(&'a BStr),
    PartialName(&'a BStr),
    Glob { glob: &'a BStr, asterisk_pos: usize },
    Object(ObjectId),
}

impl<'a> From<&'a BStr> for Needle<'a> {
    fn from(v: &'a BStr) -> Self {
        if v.starts_with(b"refs/") {
            Needle::FullName(v)
        } else {
            todo!()
        }
    }
}

impl<'a> From<RefSpecRef<'a>> for Matcher<'a> {
    fn from(v: RefSpecRef<'a>) -> Self {
        Matcher {
            lhs: v.src.map(Into::into),
            rhs: v.dst.map(Into::into),
        }
    }
}
