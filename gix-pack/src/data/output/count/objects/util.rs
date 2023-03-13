pub trait InsertImmutable {
    fn insert(&self, id: gix_hash::ObjectId) -> bool;
}

mod trait_impls {
    use std::cell::RefCell;

    use gix_hash::ObjectId;
    use gix_hashtable::HashSet;

    use super::InsertImmutable;

    impl InsertImmutable for gix_hashtable::sync::ObjectIdMap<()> {
        fn insert(&self, id: ObjectId) -> bool {
            self.insert(id, ()).is_none()
        }
    }

    impl InsertImmutable for RefCell<HashSet<ObjectId>> {
        fn insert(&self, item: ObjectId) -> bool {
            self.borrow_mut().insert(item)
        }
    }
}
