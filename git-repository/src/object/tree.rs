use git_hash::ObjectId;
use git_object::{bstr::BStr, TreeRefIter};
use git_odb::FindExt;

use crate::object::find;
use crate::Tree;

impl<'repo> Tree<'repo> {
    /// Obtain a tree instance by handing in all components that it is made up of.
    pub fn from_data(id: impl Into<ObjectId>, data: Vec<u8>, handle: &'repo crate::Repository) -> Self {
        Tree {
            id: id.into(),
            data,
            handle,
        }
    }
    // TODO: move implementation to git-object, tests.
    /// Follow a sequence of `path` components starting from this instance, and look them up one by one until the last component
    /// is looked up and its tree entry is returned.
    ///
    /// # Performance Notes
    ///
    /// Searching tree entries is currently done in sequence, which allows to the search to be allocation free. It would be possible
    /// to re-use a vector and use a binary search instead, which might be able to improve performance over all.
    /// However, a benchmark should be created first to have some data and see which trade-off to choose here.
    pub fn lookup_path<I, P>(mut self, path: I) -> Result<Option<git_object::tree::Entry>, find::existing::OdbError>
    where
        I: IntoIterator<Item = P>,
        P: PartialEq<BStr>,
    {
        // let mut out = None;
        let mut path = path.into_iter().peekable();
        while let Some(component) = path.next() {
            match TreeRefIter::from_bytes(&self.data)
                .filter_map(Result::ok)
                .find(|entry| component.eq(entry.filename))
            {
                Some(entry) => {
                    if path.peek().is_none() {
                        return Ok(Some(entry.into()));
                    } else {
                        let next_id = entry.oid.to_owned();
                        let handle = self.handle;
                        drop(entry);
                        drop(self);
                        self = match handle.find_object(next_id)?.try_into_tree() {
                            Ok(tree) => tree,
                            Err(_) => return Ok(None),
                        };
                    }
                }
                None => return Ok(None),
            }
        }
        Ok(None)
    }

    /// Obtain a platform for initiating a variety of traversals.
    pub fn traverse(&self) -> Traversal<'_, 'repo> {
        Traversal {
            root: self,
            breadthfirst: BreadthFirstTraversalPresets { root: self },
        }
    }
}

/// An intermediate object to start traversing the parent tree from.
pub struct Traversal<'a, 'repo> {
    root: &'a Tree<'repo>,
    /// TODO: EXPLAIN
    pub breadthfirst: BreadthFirstTraversalPresets<'a, 'repo>,
}

/// TODO: explain THIS!
#[derive(Copy, Clone)]
pub struct BreadthFirstTraversalPresets<'a, 'repo> {
    root: &'a Tree<'repo>,
}

impl<'a, 'repo> BreadthFirstTraversalPresets<'a, 'repo> {
    /// Returns all entries and their file paths, recursively, as reachable from this tree.
    pub fn files(&self) -> Result<Vec<git_traverse::tree::recorder::Entry>, git_traverse::tree::breadthfirst::Error> {
        let mut recorder = git_traverse::tree::Recorder::default();
        Traversal {
            root: self.root,
            breadthfirst: *self,
        }
        .breadthfirst(&mut recorder)?;
        Ok(recorder.records)
    }
}

impl<'a, 'repo> Traversal<'a, 'repo> {
    /// Start a breadth-first traversal with a delegate, note that it's not sorted.
    /// TODO: more docs or links to git-traverse
    pub fn breadthfirst<V>(&self, delegate: &mut V) -> Result<(), git_traverse::tree::breadthfirst::Error>
    where
        V: git_traverse::tree::Visit,
    {
        let root = git_object::TreeRefIter::from_bytes(&self.root.data);
        let state = git_traverse::tree::breadthfirst::State::default();
        git_traverse::tree::breadthfirst(
            root,
            state,
            |oid, buf| self.root.handle.objects.find_tree_iter(oid, buf).ok(),
            delegate,
        )
    }
}
