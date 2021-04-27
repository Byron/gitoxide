// TODO: add deny(missing_docs)
#![forbid(unsafe_code, rust_2018_idioms)]

///
pub mod tree {
    use git_hash::{oid, ObjectId};
    use git_object::immutable;
    use quick_error::quick_error;

    const EMPTY_TREE: immutable::Tree<'static> = immutable::Tree::empty();

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            NotFound(oid: ObjectId) {
                display("The object {} referenced by the tree was not found in the database", oid)
            }
            Cancelled {
                display("The delegate cancelled the operation")
            }
        }
    }

    #[derive(Default, Clone)]
    pub struct State {
        buf1: Vec<u8>,
        buf2: Vec<u8>,
    }

    pub struct VisitChanges<'a>(Option<&'a immutable::Tree<'a>>);

    impl<'a, T> From<T> for VisitChanges<'a>
    where
        T: Into<Option<&'a immutable::Tree<'a>>>,
    {
        fn from(v: T) -> Self {
            VisitChanges(v.into())
        }
    }

    impl<'a> VisitChanges<'a> {
        /// Returns the changes that need to be applied to `self` to get `other`.
        pub fn to_obtain<LocateFn>(
            &self,
            _other: &git_object::immutable::Tree<'_>,
            _state: &mut State,
            _locate: LocateFn,
            _delegate: &mut impl Delegate,
        ) -> Result<(), Error>
        where
            LocateFn: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<immutable::Object<'b>>,
        {
            let _this = *self.0.as_ref().unwrap_or(&&EMPTY_TREE);
            todo!("changes tree to tree")
        }
    }

    pub mod delegate {
        use git_hash::ObjectId;
        use git_object::{bstr::BStr, tree};

        pub enum Change {
            Addition { mode: tree::Mode, oid: ObjectId },
            Copy,
            Deletion,
            Modification,
            Renaming,
            Type,
        }

        #[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
        pub enum PathComponentMode {
            Replace,
            Push,
        }

        #[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
        pub struct PathComponent<'a> {
            pub component: &'a BStr,
            /// An ID referring uniquely to the path built thus far. Used to keep track of source paths
            /// in case of [renames][Change::Rename] and [copies][Change::Copy].
            pub id: usize,
        }

        #[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
        pub enum Action {
            Continue,
            Cancel,
        }

        pub trait Delegate {
            fn update_path_component(&mut self, component: PathComponent<'_>, mode: PathComponentMode);
            fn pop_path_component(&mut self);
            fn record(change: Change) -> Action;
        }

        #[derive(Clone, Default)]
        pub struct Recorder;

        impl Delegate for Recorder {
            fn update_path_component(&mut self, _component: PathComponent<'_>, _mode: PathComponentMode) {
                todo!()
            }

            fn pop_path_component(&mut self) {
                todo!()
            }

            fn record(_change: Change) -> Action {
                todo!()
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn size_of_change() {
                assert_eq!(
                    std::mem::size_of::<Change>(),
                    22,
                    "this type shouldn't grow without us knowing"
                )
            }
        }
    }
    pub use delegate::Delegate;
}
