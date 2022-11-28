pub trait InsertImmutable<Item: Eq + std::hash::Hash> {
    fn insert(&self, item: Item) -> bool;
}

mod trait_impls {
    use std::{cell::RefCell, hash::Hash};

    use dashmap::DashSet;
    use git_hashtable::HashSet;

    use super::InsertImmutable;

    impl<T: Eq + Hash> InsertImmutable<T> for DashSet<T, git_hashtable::hash::Builder> {
        fn insert(&self, item: T) -> bool {
            self.insert(item)
        }
    }

    impl<T: Eq + Hash> InsertImmutable<T> for RefCell<HashSet<T>> {
        fn insert(&self, item: T) -> bool {
            self.borrow_mut().insert(item)
        }
    }
}
