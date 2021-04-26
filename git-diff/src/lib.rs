// TODO: add deny(missing_docs)
#![forbid(unsafe_code, rust_2018_idioms)]

pub struct Tree<'a>(pub &'a git_object::immutable::Tree<'a>);

const EMPTY_TREE: git_object::immutable::Tree<'static> = git_object::immutable::Tree::empty();

impl<'a> Tree<'a> {
    /// Returns the changes that need to be applied to `other` to get `self`.
    pub fn changes_from(&self, other: Option<&git_object::immutable::Tree<'_>>) {
        let other = other.unwrap_or(&EMPTY_TREE);
        todo!("changes tree to tree")
    }
}
