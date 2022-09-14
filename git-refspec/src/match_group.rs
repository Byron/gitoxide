use crate::parse::Operation;
use crate::types::{MatchGroup, Mode};
use crate::RefSpecRef;
use bstr::{BStr, BString, ByteSlice, ByteVec};
use git_hash::oid;
use git_hash::ObjectId;
use std::borrow::Cow;
use std::ops::Range;

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
        let mut out = Vec::new();
        let mut matchers: Vec<Option<Matcher<'_>>> = self
            .specs
            .iter()
            .copied()
            .map(Matcher::from)
            .enumerate()
            .map(|(idx, m)| match m.lhs {
                Some(Needle::Object(id)) => {
                    out.push(Mapping {
                        item_index: None,
                        lhs: Source::ObjectId(id),
                        rhs: m.rhs.map(|n| n.to_bstr()),
                        spec_index: idx,
                    });
                    None
                }
                _ => Some(m),
            })
            .collect();

        for (spec_index, (spec, matcher)) in self.specs.iter().zip(matchers.iter_mut()).enumerate() {
            for (item_index, item) in items.clone().enumerate() {
                if spec.mode == Mode::Negative {
                    continue;
                }
                if let Some(matcher) = matcher {
                    let (matched, rhs) = matcher.matches_lhs(item);
                    if matched {
                        out.push(Mapping {
                            item_index: Some(item_index),
                            lhs: Source::FullName(item.full_ref_name),
                            rhs,
                            spec_index,
                        })
                    }
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// The source (or left-hand) side of a mapping.
pub enum Source<'a> {
    /// A full reference name, which is expected to be valid.
    ///
    /// Validity, however, is not enforced here.
    FullName(&'a BStr),
    /// The name of an object that is expected to exist on the remote side.
    /// Note that it might not be advertised by the remote but part of the object graph,
    /// and thus gets sent in the pack. The server is expected to fail unless the desired
    /// object is present but at some time it is merely a request by the user.
    ObjectId(git_hash::ObjectId),
}

/// A mapping from a remote to a local refs for fetches or local to remote refs for pushes.
///
/// Mappings are like edges in a graph, initially without any constraints.
#[derive(Debug, Clone)]
pub struct Mapping<'a, 'b> {
    /// The index into the initial `items` list that matched against a spec.
    pub item_index: Option<usize>,
    /// The name of the remote side for fetches or the local one for pushes that matched.
    pub lhs: Source<'a>,
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
    pub fn matches_lhs(&self, item: Item<'_>) -> (bool, Option<Cow<'a, BStr>>) {
        match (self.lhs, self.rhs) {
            (Some(lhs), None) => (lhs.matches(item).is_some(), None),
            (Some(lhs), Some(rhs)) => {
                let m = lhs.matches(item);
                match m {
                    Some(m) => (true, Some(rhs.to_bstr_replace(m.glob_range.map(|r| (r, item))))),
                    None => (false, None),
                }
            }
            _ => todo!(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub(crate) enum Needle<'a> {
    FullName(&'a BStr),
    PartialName(&'a BStr),
    Glob { name: &'a BStr, asterisk_pos: usize },
    Object(ObjectId),
}

#[derive(Default)]
struct Match {
    /// The range of text to copy from the originating item name
    glob_range: Option<Range<usize>>,
}

impl<'a> Needle<'a> {
    #[inline]
    fn matches(&self, item: Item<'_>) -> Option<Match> {
        match self {
            Needle::FullName(name) => (*name == item.full_ref_name).then(Match::default),
            Needle::PartialName(name) => {
                let mut buf = BString::from(Vec::with_capacity(128));
                for (base, append_head) in [
                    ("refs/", false),
                    ("refs/tags/", false),
                    ("refs/heads/", false),
                    ("refs/remotes/", false),
                    ("refs/remotes/", true),
                ] {
                    buf.clear();
                    buf.push_str(base);
                    buf.push_str(name);
                    if append_head {
                        buf.push_str("/HEAD");
                    }
                    if buf == item.full_ref_name {
                        return Some(Match::default());
                    }
                }
                None
            }
            Needle::Glob { name, asterisk_pos } => {
                if name[..*asterisk_pos] != item.full_ref_name.get(..*asterisk_pos)? {
                    return None;
                }
                let end = item.full_ref_name[*asterisk_pos..]
                    .find_byte(b'/')
                    .unwrap_or(item.full_ref_name.len());
                Some(Match {
                    glob_range: Some(*asterisk_pos..end),
                })
            }
            Needle::Object(id) => {
                if *id == item.target {
                    return Some(Match::default());
                }
                (*id == item.tag?).then(Match::default)
            }
        }
    }

    fn to_bstr_replace(self, range: Option<(Range<usize>, Item<'_>)>) -> Cow<'a, BStr> {
        match (self, range) {
            (Needle::FullName(name), None) => Cow::Borrowed(name),
            (Needle::PartialName(name), None) => Cow::Owned({
                let mut base: BString = "refs/".into();
                if !(name.starts_with(b"tags/") || name.starts_with(b"remotes/")) {
                    base.push_str("heads/");
                }
                base.push_str(name);
                base
            }),
            (Needle::Glob { name, asterisk_pos }, Some((range, item))) => {
                let mut buf = Vec::with_capacity(name.len() + range.len() - 1);
                buf.push_str(&name[..asterisk_pos]);
                buf.push_str(&item.full_ref_name[range]);
                buf.push_str(&name[asterisk_pos + 1..]);
                Cow::Owned(buf.into())
            }
            (Needle::Object(id), None) => {
                let mut name = id.to_string();
                name.insert_str(0, "refs/heads/");
                Cow::Owned(name.into())
            }
            (Needle::Glob { .. }, None) => unreachable!("BUG: no range provided for glob pattern"),
            (_, Some(_)) => unreachable!("BUG: range provided even though needle wasn't a glob. Globs are symmetric."),
        }
    }

    fn to_bstr(self) -> Cow<'a, BStr> {
        self.to_bstr_replace(None)
    }
}

impl<'a> From<&'a BStr> for Needle<'a> {
    fn from(v: &'a BStr) -> Self {
        if let Some(pos) = v.find_byte(b'*') {
            Needle::Glob {
                name: v,
                asterisk_pos: pos,
            }
        } else if v.starts_with(b"refs/") {
            Needle::FullName(v)
        } else if let Ok(id) = git_hash::ObjectId::from_hex(v) {
            Needle::Object(id)
        } else {
            Needle::PartialName(v)
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
