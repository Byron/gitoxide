use gix_odb::FindExt;

use crate::Tree;

/// Traversal
impl<'repo> Tree<'repo> {
    /// Obtain a platform for initiating a variety of traversals.
    pub fn traverse(&self) -> Platform<'_, 'repo> {
        Platform {
            root: self,
            breadthfirst: BreadthFirstPresets { root: self },
        }
    }
}

/// An intermediate object to start traversing the parent tree from.
pub struct Platform<'a, 'repo> {
    root: &'a Tree<'repo>,
    /// Provides easy access to presets for common breadth-first traversal.
    pub breadthfirst: BreadthFirstPresets<'a, 'repo>,
}

/// Presets for common choices in breadth-first traversal.
#[derive(Copy, Clone)]
pub struct BreadthFirstPresets<'a, 'repo> {
    root: &'a Tree<'repo>,
}

impl<'a, 'repo> BreadthFirstPresets<'a, 'repo> {
    /// Returns all entries and their file paths, recursively, as reachable from this tree.
    pub fn files(&self) -> Result<Vec<gix_traverse::tree::recorder::Entry>, gix_traverse::tree::breadthfirst::Error> {
        let mut recorder = gix_traverse::tree::Recorder::default();
        Platform {
            root: self.root,
            breadthfirst: *self,
        }
        .breadthfirst(&mut recorder)?;
        Ok(recorder.records)
    }
}

impl<'a, 'repo> Platform<'a, 'repo> {
    /// Start a breadth-first, recursive traversal using `delegate`, for which a [`Recorder`][gix_traverse::tree::Recorder] can be used to get started.
    ///
    /// # Note
    ///
    /// - Results are returned in sort order according to tree-entry sorting rules, one level at a time.
    /// - for obtaining the direct children of the tree, use [.iter()][crate::Tree::iter()] instead.
    pub fn breadthfirst<V>(&self, delegate: &mut V) -> Result<(), gix_traverse::tree::breadthfirst::Error>
    where
        V: gix_traverse::tree::Visit,
    {
        let root = gix_object::TreeRefIter::from_bytes(&self.root.data);
        let state = gix_traverse::tree::breadthfirst::State::default();
        gix_traverse::tree::breadthfirst(
            root,
            state,
            |oid, buf| self.root.repo.objects.find_tree_iter(oid, buf).ok(),
            delegate,
        )
    }
}
