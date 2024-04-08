use gix_hash::{oid, ObjectId};
use gix_object::bstr::ByteSlice;
use gix_traverse::commit::{topo, Parents};
use std::path::PathBuf;

use crate::hex_to_id;

struct TraversalAssertion<'a> {
    init_script: &'a str,
    worktree_dir: PathBuf,
    repo_name: &'a str,
    tips: &'a [&'a str],
    ends: &'a [&'a str],
    expected: &'a [&'a str],
    mode: Parents,
    sorting: topo::Sorting,
}

/// API
impl<'a> TraversalAssertion<'a> {
    fn new(tips: &'a [&'a str], ends: &'a [&'a str], expected: &'a [&'a str]) -> Self {
        Self::new_at("make_repo_for_topo.sh", "", tips, ends, expected)
    }

    fn new_at(
        init_script: &'a str,
        repo_name: &'a str,
        tips: &'a [&'a str],
        ends: &'a [&'a str],
        expected: &'a [&'a str],
    ) -> Self {
        TraversalAssertion {
            init_script,
            worktree_dir: Default::default(),
            repo_name,
            tips,
            ends,
            expected,
            mode: Default::default(),
            sorting: Default::default(),
        }
    }

    fn with_parents(&mut self, mode: Parents) -> &mut Self {
        self.mode = mode;
        self
    }

    fn with_sorting(&mut self, sorting: topo::Sorting) -> &mut Self {
        self.sorting = sorting;
        self
    }

    fn check_with_predicate(&mut self, predicate: impl FnMut(&oid) -> bool + Clone) -> crate::Result<()> {
        let (store, tips, ends, expected) = self.setup()?;

        for use_commitgraph in [false, true] {
            let oids = topo::Builder::from_iters(&store, tips.iter().copied(), Some(ends.iter().copied()))
                .sorting(self.sorting)
                .with_commit_graph(self.setup_commitgraph(store.store_ref(), use_commitgraph))
                .parents(self.mode)
                .with_predicate(predicate.clone())
                .build()?
                .map(|res| res.map(|info| info.id))
                .collect::<Result<Vec<_>, _>>()?;

            assert_eq!(oids, expected);
        }
        Ok(())
    }

    fn assert_baseline(&self, name: &str) {
        let buf = std::fs::read(self.worktree_dir.join(format!("{name}.baseline")))
            .expect("a baseline must be set for each repo");
        let expected: Vec<_> = buf.lines().map(|s| s.to_str().unwrap()).collect();
        assert_eq!(
            self.expected, expected,
            "Baseline must match the expectation we provide here"
        );
    }

    fn check(&mut self) -> crate::Result {
        let (store, tips, ends, expected) = self.setup()?;

        for use_commitgraph in [false, true] {
            let oids = topo::Builder::from_iters(&store, tips.iter().copied(), Some(ends.iter().copied()))
                .sorting(self.sorting)
                .with_commit_graph(self.setup_commitgraph(store.store_ref(), use_commitgraph))
                .parents(self.mode)
                .build()?
                .map(|res| res.map(|info| info.id))
                .collect::<Result<Vec<_>, _>>()?;

            assert_eq!(oids, expected);
        }
        Ok(())
    }
}

impl TraversalAssertion<'_> {
    #[allow(clippy::type_complexity)]
    fn setup(&mut self) -> crate::Result<(gix_odb::Handle, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>)> {
        let dir = gix_testtools::scripted_fixture_read_only_standalone(self.init_script)?;
        let worktree_dir = dir.join(self.repo_name);
        let store = gix_odb::at(worktree_dir.join(".git").join("objects"))?;
        self.worktree_dir = worktree_dir;

        let tips: Vec<_> = self.tips.iter().copied().map(hex_to_id).collect();
        let ends: Vec<_> = self.ends.iter().copied().map(hex_to_id).collect();
        // `tips` is not chained with expected unlike in `commit`'s
        // TraversalAssertion since it's not given that all the tips are
        // shown first.
        let expected: Vec<ObjectId> = self.expected.iter().map(|hex_id| hex_to_id(hex_id)).collect();

        Ok((store, tips, ends, expected))
    }

    fn setup_commitgraph(&self, store: &gix_odb::Store, use_graph: bool) -> Option<gix_commitgraph::Graph> {
        use_graph
            .then(|| gix_commitgraph::at(store.path().join("info")))
            .transpose()
            .expect("graph can be loaded if it exists")
    }
}

mod basic {
    use gix_traverse::commit::topo;

    use super::TraversalAssertion;

    use crate::hex_to_id;

    #[test]
    fn simple() -> crate::Result {
        let mut assertion = TraversalAssertion::new(
            &["62ed296d9986f50477e9f7b7e81cd0258939a43d"],
            &[],
            &[
                "62ed296d9986f50477e9f7b7e81cd0258939a43d",
                "722bf6b8c3d9e3a11fa5100a02ed9b140e1d209c",
                "3be0c4c793c634c8fd95054345d4935d10a0879a",
                "2083b02a78e88b747e305b6ed3d5a861cf9fb73f",
                "302a5d0530ec688c241f32c2f2b61b964dd17bee",
                "d09384f312b03e4a1413160739805ff25e8fe99d",
                "22fbc169eeca3c9678fc7028aa80fad5ef49019f",
                "eeab3243aad67bc838fc4425f759453bf0b47785",
                "693c775700cf90bd158ee6e7f14dd1b7bd83a4ce",
                "33eb18340e4eaae3e3dcf80222b02f161cd3f966",
                "1a27cb1a26c9faed9f0d1975326fe51123ab01ed",
                "f1cce1b5c7efcdfa106e95caa6c45a2cae48a481",
                "945d8a360915631ad545e0cf04630d86d3d4eaa1",
                "a863c02247a6c5ba32dff5224459f52aa7f77f7b",
                "2f291881edfb0597493a52d26ea09dd7340ce507",
                "9c46b8765703273feb10a2ebd810e70b8e2ca44a",
                "fb3e21cf45b04b617011d2b30973f3e5ce60d0cd",
            ],
        );
        assertion.with_sorting(topo::Sorting::TopoOrder).check()?;
        assertion.assert_baseline("all-commits");
        Ok(())
    }

    #[test]
    fn one_end() -> crate::Result {
        TraversalAssertion::new(
            &["62ed296d9986f50477e9f7b7e81cd0258939a43d"],
            &["f1cce1b5c7efcdfa106e95caa6c45a2cae48a481"],
            &[
                "62ed296d9986f50477e9f7b7e81cd0258939a43d",
                "722bf6b8c3d9e3a11fa5100a02ed9b140e1d209c",
                "3be0c4c793c634c8fd95054345d4935d10a0879a",
                "2083b02a78e88b747e305b6ed3d5a861cf9fb73f",
                "302a5d0530ec688c241f32c2f2b61b964dd17bee",
                "d09384f312b03e4a1413160739805ff25e8fe99d",
                "22fbc169eeca3c9678fc7028aa80fad5ef49019f",
                "eeab3243aad67bc838fc4425f759453bf0b47785",
                "693c775700cf90bd158ee6e7f14dd1b7bd83a4ce",
                "33eb18340e4eaae3e3dcf80222b02f161cd3f966",
                "1a27cb1a26c9faed9f0d1975326fe51123ab01ed",
            ],
        )
        .with_sorting(topo::Sorting::TopoOrder)
        .check()
    }

    #[test]
    fn empty_range() -> crate::Result {
        TraversalAssertion::new(
            &["f1cce1b5c7efcdfa106e95caa6c45a2cae48a481"],
            &["eeab3243aad67bc838fc4425f759453bf0b47785"],
            &[],
        )
        .with_sorting(topo::Sorting::TopoOrder)
        .check()
    }

    #[test]
    fn two_tips_two_ends() -> crate::Result {
        TraversalAssertion::new(
            &[
                "d09384f312b03e4a1413160739805ff25e8fe99d",
                "3be0c4c793c634c8fd95054345d4935d10a0879a",
            ],
            &[
                "1a27cb1a26c9faed9f0d1975326fe51123ab01ed",
                "22fbc169eeca3c9678fc7028aa80fad5ef49019f",
            ],
            &[
                "3be0c4c793c634c8fd95054345d4935d10a0879a",
                "2083b02a78e88b747e305b6ed3d5a861cf9fb73f",
                "302a5d0530ec688c241f32c2f2b61b964dd17bee",
                "d09384f312b03e4a1413160739805ff25e8fe99d",
                "eeab3243aad67bc838fc4425f759453bf0b47785",
                "693c775700cf90bd158ee6e7f14dd1b7bd83a4ce",
                "33eb18340e4eaae3e3dcf80222b02f161cd3f966",
            ],
        )
        .with_sorting(topo::Sorting::TopoOrder)
        .check()
    }

    #[test]
    fn with_dummy_predicate() -> crate::Result {
        TraversalAssertion::new(
            &["62ed296d9986f50477e9f7b7e81cd0258939a43d"],
            &[],
            &[
                "62ed296d9986f50477e9f7b7e81cd0258939a43d",
                "722bf6b8c3d9e3a11fa5100a02ed9b140e1d209c",
                "3be0c4c793c634c8fd95054345d4935d10a0879a",
                "2083b02a78e88b747e305b6ed3d5a861cf9fb73f",
                "302a5d0530ec688c241f32c2f2b61b964dd17bee",
                "d09384f312b03e4a1413160739805ff25e8fe99d",
                "22fbc169eeca3c9678fc7028aa80fad5ef49019f",
                "693c775700cf90bd158ee6e7f14dd1b7bd83a4ce",
                "33eb18340e4eaae3e3dcf80222b02f161cd3f966",
                "1a27cb1a26c9faed9f0d1975326fe51123ab01ed",
                "f1cce1b5c7efcdfa106e95caa6c45a2cae48a481",
                "945d8a360915631ad545e0cf04630d86d3d4eaa1",
                "a863c02247a6c5ba32dff5224459f52aa7f77f7b",
                "2f291881edfb0597493a52d26ea09dd7340ce507",
                "9c46b8765703273feb10a2ebd810e70b8e2ca44a",
                "fb3e21cf45b04b617011d2b30973f3e5ce60d0cd",
            ],
        )
        .with_sorting(topo::Sorting::TopoOrder)
        .check_with_predicate(|oid| oid != hex_to_id("eeab3243aad67bc838fc4425f759453bf0b47785"))
    }

    #[test]
    fn end_along_first_parent() -> crate::Result {
        TraversalAssertion::new(
            &["d09384f312b03e4a1413160739805ff25e8fe99d"],
            &["33eb18340e4eaae3e3dcf80222b02f161cd3f966"],
            &[
                "d09384f312b03e4a1413160739805ff25e8fe99d",
                "22fbc169eeca3c9678fc7028aa80fad5ef49019f",
                "eeab3243aad67bc838fc4425f759453bf0b47785",
                "693c775700cf90bd158ee6e7f14dd1b7bd83a4ce",
            ],
        )
        .with_sorting(topo::Sorting::TopoOrder)
        .check()
    }
}

mod first_parent {
    use gix_traverse::commit::{topo, Parents};

    use super::TraversalAssertion;

    #[test]
    fn basic() -> crate::Result {
        let mut assertion = TraversalAssertion::new(
            &["62ed296d9986f50477e9f7b7e81cd0258939a43d"],
            &[],
            &[
                "62ed296d9986f50477e9f7b7e81cd0258939a43d",
                "722bf6b8c3d9e3a11fa5100a02ed9b140e1d209c",
                "d09384f312b03e4a1413160739805ff25e8fe99d",
                "eeab3243aad67bc838fc4425f759453bf0b47785",
                "693c775700cf90bd158ee6e7f14dd1b7bd83a4ce",
                "33eb18340e4eaae3e3dcf80222b02f161cd3f966",
                "1a27cb1a26c9faed9f0d1975326fe51123ab01ed",
                "f1cce1b5c7efcdfa106e95caa6c45a2cae48a481",
                "945d8a360915631ad545e0cf04630d86d3d4eaa1",
                "a863c02247a6c5ba32dff5224459f52aa7f77f7b",
                "2f291881edfb0597493a52d26ea09dd7340ce507",
                "9c46b8765703273feb10a2ebd810e70b8e2ca44a",
                "fb3e21cf45b04b617011d2b30973f3e5ce60d0cd",
            ],
        );
        assertion
            .with_parents(Parents::First)
            .with_sorting(topo::Sorting::TopoOrder)
            .check()?;

        assertion.assert_baseline("first-parent");
        Ok(())
    }

    #[test]
    fn with_end() -> crate::Result {
        TraversalAssertion::new(
            &["62ed296d9986f50477e9f7b7e81cd0258939a43d"],
            &["f1cce1b5c7efcdfa106e95caa6c45a2cae48a481"],
            &[
                "62ed296d9986f50477e9f7b7e81cd0258939a43d",
                "722bf6b8c3d9e3a11fa5100a02ed9b140e1d209c",
                "d09384f312b03e4a1413160739805ff25e8fe99d",
                "eeab3243aad67bc838fc4425f759453bf0b47785",
                "693c775700cf90bd158ee6e7f14dd1b7bd83a4ce",
                "33eb18340e4eaae3e3dcf80222b02f161cd3f966",
                "1a27cb1a26c9faed9f0d1975326fe51123ab01ed",
            ],
        )
        .with_parents(Parents::First)
        .with_sorting(topo::Sorting::TopoOrder)
        .check()
    }

    #[test]
    fn end_is_second_parent() -> crate::Result {
        TraversalAssertion::new(
            &["62ed296d9986f50477e9f7b7e81cd0258939a43d"],
            &["3be0c4c793c634c8fd95054345d4935d10a0879a"],
            &[
                "62ed296d9986f50477e9f7b7e81cd0258939a43d",
                "722bf6b8c3d9e3a11fa5100a02ed9b140e1d209c",
                "d09384f312b03e4a1413160739805ff25e8fe99d",
                "eeab3243aad67bc838fc4425f759453bf0b47785",
                "693c775700cf90bd158ee6e7f14dd1b7bd83a4ce",
                "33eb18340e4eaae3e3dcf80222b02f161cd3f966",
                "1a27cb1a26c9faed9f0d1975326fe51123ab01ed",
            ],
        )
        .with_parents(Parents::First)
        .with_sorting(topo::Sorting::TopoOrder)
        .check()
    }
}

mod date_order {
    use gix_traverse::commit::topo;

    use super::TraversalAssertion;

    #[test]
    fn with_ends() -> crate::Result {
        let mut assertion = TraversalAssertion::new(
            // Same tip and end as basic::one_end() but the order should be
            // different.
            &["62ed296d9986f50477e9f7b7e81cd0258939a43d"],
            &["f1cce1b5c7efcdfa106e95caa6c45a2cae48a481"],
            &[
                "62ed296d9986f50477e9f7b7e81cd0258939a43d",
                "722bf6b8c3d9e3a11fa5100a02ed9b140e1d209c",
                "3be0c4c793c634c8fd95054345d4935d10a0879a",
                "2083b02a78e88b747e305b6ed3d5a861cf9fb73f",
                "302a5d0530ec688c241f32c2f2b61b964dd17bee",
                "d09384f312b03e4a1413160739805ff25e8fe99d",
                "eeab3243aad67bc838fc4425f759453bf0b47785",
                "22fbc169eeca3c9678fc7028aa80fad5ef49019f",
                "693c775700cf90bd158ee6e7f14dd1b7bd83a4ce",
                "33eb18340e4eaae3e3dcf80222b02f161cd3f966",
                "1a27cb1a26c9faed9f0d1975326fe51123ab01ed",
            ],
        );
        assertion.with_sorting(topo::Sorting::DateOrder).check()?;
        assertion.assert_baseline("date-order");
        Ok(())
    }
}
