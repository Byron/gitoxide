use std::{borrow::Cow, collections::HashMap, convert::TryFrom, path::Path};

use smallvec::SmallVec;

use super::GitConfig;
use crate::{
    file::LookupTreeNode,
    parser::{Key, ParserOrIoError, SectionHeaderName},
};

enum ResolvedTreeNode<'event> {
    Terminal(HashMap<Key<'event>, Cow<'event, [u8]>>),
    NonTerminal(),
}

/// A `git-config` that resolves entries on creation, providing a
/// [`HashMap`]-like interface for users. However, this does not provide the
/// same guarantees as [`GitConfig`]; namely, it does not remember comments nor
/// whitespace. Additionally, values are normalized upon creation, so it's not
/// possible to retrieve the original value.
struct ResolvedGitConfig<'data>(HashMap<(SectionHeaderName<'data>, Option<Cow<'data, [u8]>>), Cow<'data, [u8]>>);

impl ResolvedGitConfig<'static> {
    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ParserOrIoError<'static>> {
        GitConfig::open(path.as_ref()).map(Self::from)
    }
}

impl<'data> ResolvedGitConfig<'data> {
    #[inline]
    pub fn from_config(config: GitConfig) -> Self {
        // Map a <SectionId, SectionBody> into <SectionId, HashMap<Key, Cow<[u8]>>>.
        let sections: HashMap<_, _> = config
            .sections
            .into_iter()
            .map(|(key, section_body)| {
                let mut mapping: HashMap<Key, Cow<[u8]>> = HashMap::new();
                for (key, value) in section_body.into_iter() {
                    mapping.insert(key, value);
                }
                (key, mapping)
            })
            .collect();

        let section_name_to_node = config.section_lookup_tree.into_iter().map(|(section_name, vec)| {
            let node = vec.into_iter().map(|node| match node {
                LookupTreeNode::Terminal(items) => {
                    let items: HashMap<Key, Cow<[u8]>> = items
                        .into_iter()
                        .flat_map(|section_id| {
                            sections
                                .get(&section_id)
                                .expect("GitConfig invariant failed")
                                .into_iter()
                        })
                        // Copy the Cow struct, not the underlying slice.
                        .map(|(key, value)| (key.clone(), Cow::clone(value)))
                        .collect();
                    ResolvedTreeNode::Terminal(items)
                }
                LookupTreeNode::NonTerminal(mapping) => todo!(),
            });

            (section_name, node)
        });

        let mut resolved: HashMap<(SectionHeaderName, Option<Cow<[u8]>>), SmallVec<[Cow<[u8]>; 1]>> = HashMap::new();

        for (section_name, node) in section_name_to_node {
            for node in node {
                // let (subsection_name, entry) = match node {
                //     ResolvedTreeNode::Terminal(mapping) => (None, mapping),
                //     ResolvedTreeNode::NonTerminal() => todo!(),
                // };
                todo!()
            }
            // let entry = resolved.entry((section_name, subsection_name)).or_default();
        }

        todo!()
    }
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
    fn from(config: GitConfig) -> Self {
        Self::from_config(config)
    }
}
