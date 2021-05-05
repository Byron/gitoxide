///
pub mod visit {
    use git_hash::bstr::BStr;
    use git_object::immutable;

    /// What to do after an entry was [recorded][Visit::visit()].
    #[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
    pub enum Action {
        /// Continue the traversal of entries.
        Continue,
        /// Stop the traversal of entries, making this te last call to [visit(…)][Visit::visit()].
        Cancel,
        /// Don't dive into the entry, skipping children effectively.
        Skip,
    }

    impl Action {
        /// Returns true if this action means to stop the traversal.
        pub fn cancelled(&self) -> bool {
            matches!(self, Action::Cancel)
        }
    }

    /// A trait to allow responding to a traversal designed to observe all entries in a tree, recursively while keeping track of
    /// paths if desired.
    pub trait Visit {
        /// A type capable of uniquely identifying paths in a tree.
        type PathId: Clone + Default;

        /// Sets the path associated with the given `id` so future calls to push and pop components affect it instead.
        fn set_current_path(&mut self, id: Self::PathId);
        /// Append a `component` to the end of a path, which may be empty, and associate it with the returned path id.
        fn push_tracked_path_component(&mut self, component: &BStr) -> Self::PathId;
        /// Append a `component` to the end of a path, which may be empty.
        fn push_path_component(&mut self, component: &BStr);
        /// Removes the last component from the path, which may leave it empty.
        fn pop_path_component(&mut self);

        /// Observe a tree entry that is a tree and return an instruction whether to continue or not.
        /// [`Action::Skip`] can be used to prevent traversing it, for example if it's known to the caller already.
        ///
        /// The implementation may use the current path to learn where in the tree the change is located.
        fn visit_tree(&mut self, entry: &immutable::tree::Entry<'_>) -> Action;

        /// Observe a tree entry that is NO tree and return an instruction whether to continue or not.
        /// [`Action::Skip`] has no effect here.
        ///
        /// The implementation may use the current path to learn where in the tree the change is located.
        fn visit_nontree(&mut self, entry: &immutable::tree::Entry<'_>) -> Action;
    }
}

///
pub mod recorder {
    use crate::tree::visit;
    use crate::tree::visit::Action;
    use git_hash::ObjectId;
    use git_object::{
        bstr::{BStr, BString, ByteSlice, ByteVec},
        immutable, tree,
    };
    use std::collections::BTreeMap;

    /// An owned entry as observed by a call to [`visit_(tree|nontree)(…)`][visit::Visit::visit_tree()], enhanced with the full path to it.
    /// Otherwise similar to [`immutable::tree::Entry`][git_object::immutable::tree::Entry].
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Entry {
        /// The kind of entry, similar to entries in a unix directory tree.
        pub mode: tree::EntryMode,
        /// The full path to the entry. A root entry would be `d`, and a file `a` within the directory would be `d/a`.
        ///
        /// This is independent of the platform and the path separators actually used there.
        pub filepath: BString,
        /// The id of the entry which can be used to locate it in an object database.
        pub oid: ObjectId,
    }

    impl Entry {
        fn new(entry: &immutable::tree::Entry<'_>, filepath: BString) -> Self {
            Entry {
                filepath,
                oid: entry.oid.to_owned(),
                mode: entry.mode,
            }
        }
    }

    /// A [Visit][visit::Visit] implementation to record every observed change and keep track of the changed paths.
    #[derive(Clone, Debug, Default)]
    pub struct Recorder {
        path_count: usize,
        path_map: BTreeMap<usize, BString>,
        path: BString,
        /// The observed entries.
        pub records: Vec<Entry>,
    }

    impl Recorder {
        fn pop_element(&mut self) {
            if let Some(pos) = self.path.rfind_byte(b'/') {
                self.path.resize(pos, 0);
            } else {
                self.path.clear();
            }
        }

        fn push_element(&mut self, name: &BStr) {
            if !self.path.is_empty() {
                self.path.push(b'/');
            }
            self.path.push_str(name);
        }

        fn path_clone(&self) -> BString {
            self.path.clone()
        }
    }

    impl visit::Visit for Recorder {
        type PathId = usize;

        fn set_current_path(&mut self, path: Self::PathId) {
            self.path = self.path_map.remove(&path).expect("every parent is set only once");
        }

        fn push_tracked_path_component(&mut self, component: &BStr) -> Self::PathId {
            self.push_element(component);
            self.path_map.insert(self.path_count, self.path_clone());
            let res = self.path_count;
            self.path_count += 1;
            res
        }

        fn push_path_component(&mut self, component: &BStr) {
            self.push_element(component);
        }

        fn pop_path_component(&mut self) {
            self.pop_element();
        }

        fn visit_tree(&mut self, entry: &immutable::tree::Entry<'_>) -> Action {
            self.records.push(Entry::new(entry, self.path_clone()));
            Action::Continue
        }

        fn visit_nontree(&mut self, entry: &immutable::tree::Entry<'_>) -> Action {
            self.records.push(Entry::new(entry, self.path_clone()));
            Action::Continue
        }
    }
}
#[doc(inline)]
pub use recorder::Recorder;

///
pub mod breadthfirst {
    use crate::tree::visit::Visit;
    use git_hash::{oid, ObjectId};
    use git_object::{immutable, tree};
    use quick_error::quick_error;
    use std::{borrow::BorrowMut, collections::VecDeque};

    quick_error! {
        /// The error is part of the item returned by the [breadthfirst] function.
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            NotFound{oid: ObjectId} {
                display("The object {} could not be found", oid)
            }
            Cancelled {
                display("The delegate cancelled the operation")
            }
            ObjectDecode(err: immutable::object::decode::Error) {
                display("An object could not be decoded")
                source(err)
                from()
            }
        }
    }

    /// The state used and potentially shared by multiple tree traversals.
    #[derive(Default, Clone)]
    pub struct State<PathId: Clone> {
        next: VecDeque<(Option<PathId>, ObjectId)>,
        buf: Vec<u8>,
    }

    impl<PathId: Clone> State<PathId> {
        fn clear(&mut self) {
            self.next.clear();
            self.buf.clear();
        }
    }

    /// Create a new instance.
    ///
    /// * `find` - a way to lookup new object data during traversal by their ObjectId, writing their data into buffer and returning
    ///    an iterator over entries if the object is present and is a tree. Caching should be implemented within this function
    ///    as needed. The return value is `Option<TreeIter>` which degenerates all error information. Not finding a commit should also
    ///    be considered an errors as all objects in the tree DAG should be present in the database. Hence [`Error::NotFound`] should
    ///    be escalated into a more specific error if its encountered by the caller.
    /// * `state` - all state used for the iteration. If multiple iterations are performed, allocations can be minimized by reusing
    ///   this state.
    /// * `root`
    ///   * the starting points of the iteration
    ///   * each commit they lead to will only be returned once, including the tip that started it
    pub fn traverse<StateMut, Find, V>(
        root: impl Into<ObjectId>,
        mut state: StateMut,
        mut find: Find,
        delegate: &mut V,
    ) -> Result<(), Error>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::TreeIter<'a>>,
        StateMut: BorrowMut<State<V::PathId>>,
        V: Visit,
    {
        let state = state.borrow_mut();
        state.clear();
        state.next.push_back((None, root.into()));
        while let Some((path_id, oid)) = state.next.pop_front() {
            if let Some(path_id) = path_id {
                delegate.set_current_path(path_id);
            }
            match find(&oid, &mut state.buf) {
                Some(tree_iter) => {
                    for entry in tree_iter {
                        let entry = entry?;
                        match entry.mode {
                            tree::EntryMode::Tree => {
                                use super::visit::Action::*;
                                let path_id = delegate.push_tracked_path_component(entry.filename);
                                let action = delegate.visit_tree(&entry);
                                match action {
                                    Skip => {}
                                    Continue => state.next.push_back((Some(path_id), entry.oid.to_owned())),
                                    Cancel => {
                                        return Err(Error::Cancelled);
                                    }
                                }
                            }
                            _non_tree => {
                                delegate.push_path_component(entry.filename);
                                if delegate.visit_nontree(&entry).cancelled() {
                                    return Err(Error::Cancelled);
                                }
                            }
                        }
                        delegate.pop_path_component();
                    }
                }
                None => return Err(Error::NotFound { oid: oid.to_owned() }),
            }
        }
        Ok(())
    }
}
