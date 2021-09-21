pub mod changes {
    use git_diff::tree::{
        visit::{Action, Change},
        Visit,
    };
    use git_hash::ObjectId;
    use git_object::bstr::BStr;

    use crate::data::output::count::objects_impl::util::InsertImmutable;

    pub struct AllNew<'a, H> {
        pub objects: Vec<ObjectId>,
        all_seen: &'a H,
    }

    impl<'a, H> AllNew<'a, H>
    where
        H: InsertImmutable<ObjectId>,
    {
        pub fn new(all_seen: &'a H) -> Self {
            AllNew {
                objects: Default::default(),
                all_seen,
            }
        }
        pub fn clear(&mut self) {
            self.objects.clear();
        }
    }

    impl<'a, H> Visit for AllNew<'a, H>
    where
        H: InsertImmutable<ObjectId>,
    {
        fn pop_front_tracked_path_and_set_current(&mut self) {}

        fn push_back_tracked_path_component(&mut self, _component: &BStr) {}

        fn push_path_component(&mut self, _component: &BStr) {}

        fn pop_path_component(&mut self) {}

        fn visit(&mut self, change: Change) -> Action {
            match change {
                Change::Addition { oid, .. } | Change::Modification { oid, .. } => {
                    let inserted = self.all_seen.insert(oid);
                    if inserted {
                        self.objects.push(oid);
                    }
                }
                Change::Deletion { .. } => {}
            };
            Action::Continue
        }
    }
}

pub mod traverse {
    use git_hash::ObjectId;
    use git_object::{bstr::BStr, tree::EntryRef};
    use git_traverse::tree::{visit::Action, Visit};

    use crate::data::output::count::objects_impl::util::InsertImmutable;

    pub struct AllUnseen<'a, H> {
        pub non_trees: Vec<ObjectId>,
        all_seen: &'a H,
    }

    impl<'a, H> AllUnseen<'a, H>
    where
        H: InsertImmutable<ObjectId>,
    {
        pub fn new(all_seen: &'a H) -> Self {
            AllUnseen {
                non_trees: Default::default(),
                all_seen,
            }
        }
        pub fn clear(&mut self) {
            self.non_trees.clear();
        }
    }

    impl<'a, H> Visit for AllUnseen<'a, H>
    where
        H: InsertImmutable<ObjectId>,
    {
        fn pop_front_tracked_path_and_set_current(&mut self) {}

        fn push_back_tracked_path_component(&mut self, _component: &BStr) {}

        fn push_path_component(&mut self, _component: &BStr) {}

        fn pop_path_component(&mut self) {}

        fn visit_tree(&mut self, entry: &EntryRef<'_>) -> Action {
            let inserted = self.all_seen.insert(entry.oid.to_owned());
            if inserted {
                Action::Continue
            } else {
                Action::Skip
            }
        }

        fn visit_nontree(&mut self, entry: &EntryRef<'_>) -> Action {
            let inserted = self.all_seen.insert(entry.oid.to_owned());
            if inserted {
                self.non_trees.push(entry.oid.to_owned());
            }
            Action::Continue
        }
    }
}
