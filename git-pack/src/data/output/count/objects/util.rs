pub trait InsertImmutable<Item: Eq + std::hash::Hash> {
    fn insert(&self, item: Item) -> bool;
}

mod trait_impls {
    use std::{cell::RefCell, hash::Hash};

    use dashmap::DashSet;
    use git_shamap::{ShaHashBuilder, ShaHashSet};

    use super::InsertImmutable;

    impl<T: Eq + Hash> InsertImmutable<T> for DashSet<T, ShaHashBuilder> {
        fn insert(&self, item: T) -> bool {
            self.insert(item)
        }
    }

    impl<T: Eq + Hash> InsertImmutable<T> for RefCell<ShaHashSet<T>> {
        fn insert(&self, item: T) -> bool {
            self.borrow_mut().insert(item)
        }
    }
}
