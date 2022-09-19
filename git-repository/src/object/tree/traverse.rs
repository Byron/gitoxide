use crate::Tree;
use git_odb::FindExt;

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
    #[allow(missing_docs)] // TODO
    pub breadthfirst: BreadthFirstPresets<'a, 'repo>,
}

#[allow(missing_docs)] // TODO
#[derive(Copy, Clone)]
pub struct BreadthFirstPresets<'a, 'repo> {
    root: &'a Tree<'repo>,
}

impl<'a, 'repo> BreadthFirstPresets<'a, 'repo> {
    /// Returns all entries and their file paths, recursively, as reachable from this tree.
    pub fn files(&self) -> Result<Vec<git_traverse::tree::recorder::Entry>, git_traverse::tree::breadthfirst::Error> {
        let mut recorder = git_traverse::tree::Recorder::default();
        Platform {
            root: self.root,
            breadthfirst: *self,
        }
        .breadthfirst(&mut recorder)?;
        Ok(recorder.records)
    }
}

impl<'a, 'repo> Platform<'a, 'repo> {
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
            |oid, buf| self.root.repo.objects.find_tree_iter(oid, buf).ok(),
            delegate,
        )
    }
}
