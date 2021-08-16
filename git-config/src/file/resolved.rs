use std::{borrow::Cow, collections::HashMap, convert::TryFrom, path::Path};

use super::{GitConfig, SectionId};
use crate::{
    file::LookupTreeNode,
    parser::{Key, ParserOrIoError, SectionHeaderName},
};

enum ResolvedTreeNode<'event> {
    Terminal(HashMap<Key<'event>, Cow<'event, [u8]>>),
    NonTerminal(HashMap<Cow<'event, str>, HashMap<Key<'event>, Cow<'event, [u8]>>>),
}

/// A `git-config` that resolves entries on creation, providing a
/// [`HashMap`]-like interface for users.
///
/// This does not provide the same guarantees as [`GitConfig`]; namely, it does
/// not remember comments nor whitespace. Additionally, values are normalized
/// upon creation, so it's not possible to retrieve the original value.
#[allow(clippy::module_name_repetitions)]
pub struct ResolvedGitConfig<'data>(HashMap<SectionLookupTuple<'data>, HashMap<Key<'data>, Cow<'data, [u8]>>>);

type SectionLookupTuple<'data> = (SectionHeaderName<'data>, Option<Cow<'data, str>>);

impl ResolvedGitConfig<'static> {
    /// Opens a `git-config` file from the given path.
    ///
    /// # Errors
    ///
    /// This returns an error if an IO error occurs, or if the file is not a
    /// valid `git-config` file.
    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ParserOrIoError<'static>> {
        GitConfig::open(path.as_ref()).map(Self::from)
    }
}

impl<'data> ResolvedGitConfig<'data> {
    /// Resolves a given [`GitConfig`].
    #[must_use]
    pub fn from_config(config: GitConfig<'data>) -> Self {
        // Map a <SectionId, SectionBody> into <SectionId, HashMap<Key, Cow<[u8]>>>.
        let sections: HashMap<_, _> = config
            .sections
            .into_iter()
            .map(|(key, section_body)| {
                let mut mapping: HashMap<Key, Cow<[u8]>> = HashMap::new();
                for (key, value) in section_body {
                    mapping.insert(key, value);
                }
                (key, mapping)
            })
            .collect();

        let section_name_to_node = config.section_lookup_tree.into_iter().map(|(section_name, vec)| {
            let node = vec.into_iter().map(|node| match node {
                LookupTreeNode::Terminal(items) => ResolvedTreeNode::Terminal(resolve_sections(&sections, items)),
                LookupTreeNode::NonTerminal(mapping) => {
                    let items = mapping
                        .into_iter()
                        .map(|(key, items)| (key, resolve_sections(&sections, items)))
                        .collect();
                    ResolvedTreeNode::NonTerminal(items)
                }
            });

            (section_name, node)
        });

        let mut resolved: HashMap<_, HashMap<Key, Cow<[u8]>>> = HashMap::new();

        for (section_name, node) in section_name_to_node {
            for node in node {
                match node {
                    ResolvedTreeNode::Terminal(mapping) => {
                        let entry = resolved.entry((section_name.clone(), None)).or_default();
                        entry.extend(mapping);
                    }
                    ResolvedTreeNode::NonTerminal(mapping) => {
                        for (subsection, mapping) in mapping {
                            let entry = resolved.entry((section_name.clone(), Some(subsection))).or_default();
                            entry.extend(mapping);
                        }
                    }
                };
            }
        }

        Self(resolved)
    }
}

fn resolve_sections<'key, 'data>(
    mapping: &HashMap<SectionId, HashMap<Key<'key>, Cow<'data, [u8]>>>,
    sections: Vec<SectionId>,
) -> HashMap<Key<'key>, Cow<'data, [u8]>> {
    sections
        .into_iter()
        .flat_map(|section_id| mapping.get(&section_id).expect("GitConfig invariant failed").iter())
        // Copy the Cow struct, not the underlying slice.
        .map(|(key, value)| (Key::clone(key), Cow::clone(value)))
        .collect()
}

impl TryFrom<&Path> for ResolvedGitConfig<'static> {
    type Error = ParserOrIoError<'static>;

    #[inline]
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Self::open(path)
    }
}

impl<'data> From<GitConfig<'data>> for ResolvedGitConfig<'data> {
    #[inline]
    fn from(config: GitConfig<'data>) -> Self {
        Self::from_config(config)
    }
}
