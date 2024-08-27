mod baseline {
    use bstr::ByteSlice;
    use gix_hash::ObjectId;
    use gix_revision::merge_base;
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};

    #[test]
    fn validate() -> crate::Result {
        let root = gix_testtools::scripted_fixture_read_only("make_merge_base_repos.sh")?;
        let mut count = 0;
        let odb = gix_odb::at(root.join(".git/objects"))?;
        for baseline_path in expectation_paths(&root)? {
            count += 1;
            for use_commitgraph in [false, true] {
                let cache = use_commitgraph
                    .then(|| gix_commitgraph::Graph::from_info_dir(&odb.store_ref().path().join("info")).unwrap());
                for expected in parse_expectations(&baseline_path)? {
                    let mut graph = gix_revision::Graph::new(&odb, cache.as_ref());
                    let actual = merge_base(expected.first, &expected.others, &mut graph)?;
                    assert_eq!(
                        actual,
                        expected.bases,
                        "sample {file:?}:{input}",
                        file = baseline_path.with_extension("").file_name(),
                        input = expected.plain_input
                    );
                }
                let mut graph = gix_revision::Graph::new(&odb, cache.as_ref());
                for expected in parse_expectations(&baseline_path)? {
                    let actual = merge_base(expected.first, &expected.others, &mut graph)?;
                    assert_eq!(
                        actual,
                        expected.bases,
                        "sample (reused graph) {file:?}:{input}",
                        file = baseline_path.with_extension("").file_name(),
                        input = expected.plain_input
                    );
                }
            }
        }
        assert_ne!(count, 0, "there must be at least one baseline");
        Ok(())
    }

    /// The expectation as produced by Git itself
    #[derive(Debug)]
    struct Expectation {
        plain_input: String,
        first: ObjectId,
        others: Vec<ObjectId>,
        bases: Option<Vec<ObjectId>>,
    }

    fn parse_expectations(baseline: &Path) -> std::io::Result<Vec<Expectation>> {
        let lines = std::fs::read(baseline)?;
        let mut lines = lines.lines();
        let mut out = Vec::new();
        while let Some(plain_input) = lines.next() {
            let plain_input = plain_input.to_str_lossy().into_owned();
            let mut input = lines
                .next()
                .expect("second line is resolved input objects")
                .split(|b| *b == b' ');
            let first = ObjectId::from_hex(input.next().expect("at least one object")).unwrap();
            let others = input.map(|hex_id| ObjectId::from_hex(hex_id).unwrap()).collect();
            let bases: Vec<_> = lines
                .by_ref()
                .take_while(|l| !l.is_empty())
                .map(|hex_id| ObjectId::from_hex(hex_id).unwrap())
                .collect();
            out.push(Expectation {
                plain_input,
                first,
                others,
                bases: if bases.is_empty() { None } else { Some(bases) },
            });
        }
        Ok(out)
    }

    fn expectation_paths(root: &Path) -> std::io::Result<Vec<PathBuf>> {
        let mut out: Vec<_> = std::fs::read_dir(root)?
            .map(Result::unwrap)
            .filter_map(|e| (e.path().extension() == Some(OsStr::new("baseline"))).then(|| e.path()))
            .collect();
        out.sort();
        Ok(out)
    }
}
