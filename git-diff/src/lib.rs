// TODO: add deny(missing_docs)
#![forbid(unsafe_code, rust_2018_idioms)]

///
pub mod tree {
    use git_object::immutable;
    const EMPTY_TREE: immutable::Tree<'static> = immutable::Tree::empty();

    pub struct Changes<'a>(Option<&'a immutable::Tree<'a>>);

    impl<'a, T> From<T> for Changes<'a>
    where
        T: Into<Option<&'a immutable::Tree<'a>>>,
    {
        fn from(v: T) -> Self {
            Changes(v.into())
        }
    }

    impl<'a> Changes<'a> {
        /// Returns the changes that need to be applied to `self` to get `other`.
        pub fn to_obtain(&self, _other: &git_object::immutable::Tree<'_>) {
            let _this = *self.0.as_ref().unwrap_or(&&EMPTY_TREE);
            todo!("changes tree to tree")
        }
    }
}
