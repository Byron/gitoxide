// TODO: add deny(missing_docs)
#![forbid(unsafe_code, rust_2018_idioms)]
#![allow(dead_code, unused_variables)]

///
pub mod tree {
    use git_hash::{oid, ObjectId};
    use git_object::{
        bstr::{BStr, BString},
        immutable, tree,
    };

    const EMPTY_TREE: immutable::Tree<'static> = immutable::Tree::empty();

    type PathNodeId = usize;

    /// A tree of paths to make storage and allocation of paths more efficient.
    #[derive(Default)]
    pub struct PathTree {
        graph: petgraph::graph::DiGraph<BString, ()>,
    }

    impl PathTree {
        /// Find the path with the given `id` and place all of its elements in `out_elements`.
        /// Returns the amount of elements the path contains, like `&["a", "b", "c"]` would be `Some(3)`.
        ///
        /// The output vector will be cleared beforehand.
        pub fn elements(&self, id: PathNodeId, out_elements: &mut Vec<&BStr>) -> Option<usize> {
            todo!("fetch elements")
        }
    }

    pub enum Change {
        Addition {
            mode: tree::Mode,
            oid: ObjectId,
            path: PathNodeId,
        },
        Copy,
        Deletion,
        Modification,
        Renaming,
        Type,
    }

    pub struct Changes<'a>(Option<&'a immutable::Tree<'a>>);

    impl<'a, T> From<T> for Changes<'a>
    where
        T: Into<Option<&'a immutable::Tree<'a>>>,
    {
        fn from(v: T) -> Self {
            Changes(v.into())
        }
    }

    // Possible things to detect (from git diff --help)
    // o   A: addition of a file
    // o   C: copy of a file into a new one
    // o   D: deletion of a file
    // o   M: modification of the contents or mode of a file
    // o   R: renaming of a file
    // o   T: change in the type of the file
    // o   U: file is unmerged (you must complete the merge before it can be committed)
    // o   X: "unknown" change type (most probably a bug, please report it)
    impl<'a> Changes<'a> {
        /// Returns the changes that need to be applied to `self` to get `other`.
        pub fn to_obtain<LocateFn>(
            &self,
            _other: &git_object::immutable::Tree<'_>,
            buf: (&mut Vec<u8>, &mut Vec<u8>),
            locate: LocateFn,
            out_paths: &mut PathTree,
            out_changes: &mut Vec<Change>,
        ) where
            LocateFn: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<immutable::Tree<'b>>,
        {
            out_paths.graph.clear();
            out_changes.clear();

            let _this = *self.0.as_ref().unwrap_or(&&EMPTY_TREE);
            todo!("changes tree to tree")
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn size_of_change() {
            assert_eq!(
                std::mem::size_of::<Change>(),
                32,
                "this type shouldn't grow without us knowing"
            )
        }
    }
}
