type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

mod file;
mod transaction {
    mod refedit_ext {
        use git_ref::transaction::{Change, RefEdit, RefEditsExt, RefLogMode};
        use std::convert::TryInto;

        fn named_edit(name: &str) -> RefEdit {
            RefEdit {
                change: Change::Delete {
                    previous: None,
                    mode: RefLogMode::RefAndRefLog,
                },
                name: name.try_into().expect("valid name"),
                deref: false,
            }
        }

        #[test]
        fn reject_duplicates() {
            assert!(
                vec![named_edit("HEAD")].assure_one_name_has_one_edit().is_ok(),
                "there are no duplicates"
            );
            assert!(
                vec![named_edit("refs/foo"), named_edit("HEAD")]
                    .assure_one_name_has_one_edit()
                    .is_ok(),
                "there are no duplicates"
            );
            assert_eq!(
                vec![named_edit("HEAD"), named_edit("refs/heads/main"), named_edit("HEAD")]
                    .assure_one_name_has_one_edit()
                    .expect_err("duplicate"),
                "HEAD",
                "a correctly named duplicate"
            );
        }

        mod splitting {
            use bstr::{BString, ByteSlice};
            use git_hash::ObjectId;
            use git_ref::{
                mutable::Target,
                transaction::{Change, RefEdit, RefEditsExt, RefLogMode},
                FullName, PartialName, RefStore,
            };
            use git_testtools::hex_to_id;
            use std::cell::Cell;
            use std::{cell::RefCell, collections::BTreeMap, convert::TryInto};

            #[derive(Default)]
            struct MockStore {
                targets: RefCell<BTreeMap<BString, Target>>,
            }

            impl MockStore {
                fn assert_empty(self) {
                    assert_eq!(self.targets.borrow().len(), 0, "all targets should be used");
                }
                fn with(edits: impl IntoIterator<Item = (&'static str, Target)>) -> Self {
                    MockStore {
                        targets: {
                            let mut h = BTreeMap::new();
                            h.extend(edits.into_iter().map(|(k, v)| (k.as_bytes().as_bstr().to_owned(), v)));
                            RefCell::new(h)
                        },
                    }
                }
            }

            impl RefStore for MockStore {
                type FindOneExistingError = std::io::Error;

                fn find_one_existing(&self, name: PartialName<'_>) -> Result<Target, Self::FindOneExistingError> {
                    self.targets
                        .borrow_mut()
                        .remove(name.as_bstr())
                        .ok_or(std::io::ErrorKind::NotFound.into())
                }
            }

            fn find<'a>(edits: &'a [RefEdit], name: &str) -> &'a RefEdit {
                let name: FullName = name.try_into().unwrap();
                edits
                    .iter()
                    .find(|e| e.name.as_ref() == name.as_bstr())
                    .expect("always available")
            }

            #[test]
            fn non_symbolic_refs_are_ignored_or_if_the_deref_flag_is_not_set() -> crate::Result {
                let store = MockStore::with(Some((
                    "refs/heads/anything-but-not-symbolic",
                    Target::Peeled(ObjectId::null_sha1()),
                )));
                let mut edits = vec![
                    RefEdit {
                        change: Change::Delete {
                            previous: None,
                            mode: RefLogMode::RefAndRefLog,
                        },
                        name: "SYMBOLIC_PROBABLY_BUT_DEREF_IS_FALSE_SO_IGNORED".try_into()?,
                        deref: false,
                    },
                    RefEdit {
                        change: Change::Delete {
                            previous: None,
                            mode: RefLogMode::RefAndRefLog,
                        },
                        name: "refs/heads/anything-but-not-symbolic".try_into()?,
                        deref: true,
                    },
                    RefEdit {
                        change: Change::Delete {
                            previous: None,
                            mode: RefLogMode::RefAndRefLog,
                        },
                        name: "refs/heads/does-not-exist-and-deref-is-ignored".try_into()?,
                        deref: true,
                    },
                ];

                edits.extend_with_splits_of_symbolic_refs(&store, |_, _| panic!("should not be called"))?;
                assert_eq!(edits.len(), 3, "no edit was added");
                assert!(
                    !find(&edits, "refs/heads/anything-but-not-symbolic").deref,
                    "the algorithm corrects these flags"
                );
                assert!(
                    find(&edits, "refs/heads/does-not-exist-and-deref-is-ignored").deref,
                    "non-existing refs won't change the flag"
                );
                store.assert_empty();
                Ok(())
            }
            #[test]
            fn empty_inputs_are_ok() -> crate::Result {
                Vec::<RefEdit>::new()
                    .extend_with_splits_of_symbolic_refs(&MockStore::default(), |_, e| e)
                    .map_err(Into::into)
            }

            #[test]
            #[ignore]
            fn symbolic_refs_cycles_are_handled_gracefully() {
                #[derive(Default)]
                struct Cycler {
                    next_item: Cell<bool>,
                }
                impl RefStore for Cycler {
                    type FindOneExistingError = std::convert::Infallible;

                    fn find_one_existing(&self, _name: PartialName<'_>) -> Result<Target, Self::FindOneExistingError> {
                        let item: bool = self.next_item.get();
                        self.next_item.set(!item);
                        Ok(Target::Symbolic(
                            if item { "heads/refs/next" } else { "heads/refs/previous" }
                                .try_into()
                                .unwrap(),
                        ))
                    }
                }

                let mut edits = vec![
                    RefEdit {
                        change: Change::Delete {
                            previous: None,
                            mode: RefLogMode::RefAndRefLog,
                        },
                        name: "refs/heads/delete-symbolic-1".try_into().unwrap(),
                        deref: true,
                    },
                    RefEdit {
                        change: Change::Update {
                            previous: None,
                            mode: RefLogMode::RefAndRefLog,
                            force_create_reflog: true,
                            new: Target::Peeled(ObjectId::null_sha1()),
                        },
                        name: "refs/heads/update-symbolic-1".try_into().unwrap(),
                        deref: true,
                    },
                ];

                let err = edits
                    .extend_with_splits_of_symbolic_refs(&Cycler::default(), |_, e| e)
                    .expect_err("cycle detected");
                assert_eq!(
                    err.to_string(),
                    "Could not follow all splits after 5 rounds, assuming reference cycle"
                );
            }

            #[test]
            fn symbolic_refs_are_split_into_referents_handling_the_reflog_recursively() {
                let store = MockStore::with(vec![
                    (
                        "refs/heads/delete-symbolic-1",
                        Target::Symbolic("refs/heads/delete-symbolic-2".try_into().unwrap()),
                    ),
                    (
                        "refs/heads/delete-symbolic-2",
                        Target::Symbolic("refs/heads/delete-symbolic-3".try_into().unwrap()),
                    ),
                    (
                        "refs/heads/delete-symbolic-3",
                        Target::Peeled(hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")),
                    ),
                    (
                        "refs/heads/update-symbolic-1",
                        Target::Symbolic("refs/heads/update-symbolic-2".try_into().unwrap()),
                    ),
                    (
                        "refs/heads/update-symbolic-2",
                        Target::Symbolic("refs/heads/update-symbolic-3".try_into().unwrap()),
                    ),
                    (
                        "refs/heads/update-symbolic-3",
                        Target::Peeled(hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")),
                    ),
                ]);
                let mut edits = vec![
                    RefEdit {
                        change: Change::Delete {
                            previous: None,
                            mode: RefLogMode::RefAndRefLog,
                        },
                        name: "refs/heads/delete-symbolic-1".try_into().unwrap(),
                        deref: true,
                    },
                    RefEdit {
                        change: Change::Update {
                            previous: None,
                            mode: RefLogMode::RefAndRefLog,
                            force_create_reflog: true,
                            new: Target::Peeled(ObjectId::null_sha1()),
                        },
                        name: "refs/heads/update-symbolic-1".try_into().unwrap(),
                        deref: true,
                    },
                ];

                let mut indices = Vec::new();
                edits
                    .extend_with_splits_of_symbolic_refs(&store, |idx, e| {
                        indices.push(idx);
                        e
                    })
                    .unwrap();
                assert_eq!(edits.len(), 6, "it follows all symbolic links");
                assert_eq!(
                    indices,
                    vec![0, 1, 2, 3],
                    "the parent index is passed each time there is a split"
                );

                assert_eq!(
                    edits,
                    vec![
                        RefEdit {
                            change: Change::Delete {
                                previous: None,
                                mode: RefLogMode::RefLogOnly,
                            },
                            name: "refs/heads/delete-symbolic-1".try_into().unwrap(),
                            deref: false,
                        },
                        RefEdit {
                            change: Change::Update {
                                previous: None,
                                mode: RefLogMode::RefLogOnly,
                                force_create_reflog: true,
                                new: Target::Peeled(ObjectId::null_sha1()),
                            },
                            name: "refs/heads/update-symbolic-1".try_into().unwrap(),
                            deref: false,
                        },
                        RefEdit {
                            change: Change::Delete {
                                previous: None,
                                mode: RefLogMode::RefLogOnly,
                            },
                            name: "refs/heads/delete-symbolic-2".try_into().unwrap(),
                            deref: false,
                        },
                        RefEdit {
                            change: Change::Update {
                                previous: None,
                                mode: RefLogMode::RefLogOnly,
                                force_create_reflog: true,
                                new: Target::Peeled(ObjectId::null_sha1()),
                            },
                            name: "refs/heads/update-symbolic-2".try_into().unwrap(),
                            deref: false,
                        },
                        RefEdit {
                            change: Change::Delete {
                                previous: None,
                                mode: RefLogMode::RefAndRefLog,
                            },
                            name: "refs/heads/delete-symbolic-3".try_into().unwrap(),
                            deref: false,
                        },
                        RefEdit {
                            change: Change::Update {
                                previous: None,
                                mode: RefLogMode::RefAndRefLog,
                                force_create_reflog: true,
                                new: Target::Peeled(ObjectId::null_sha1()),
                            },
                            name: "refs/heads/update-symbolic-3".try_into().unwrap(),
                            deref: false,
                        },
                    ]
                )
            }
        }
    }
}
