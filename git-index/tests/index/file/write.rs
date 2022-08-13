use filetime::FileTime;
use git_index::verify::extensions::no_find;
use git_index::{decode, write, State, Version};
use std::cmp::{max, min};

#[test]
fn roundtrips() {
    let input = [
        ("v2", write::Options::default()),
        ("V2_empty", write::Options::default()),
        (
            "v2_more_files",
            write::Options {
                end_of_index_entry_extension: false,
                ..write::Options::default()
            },
        ),
    ];

    for (fixture, options) in input {
        let path = crate::fixture_index_path(fixture);
        let expected_index = git_index::File::at(&path, decode::Options::default()).unwrap();
        let expected_bytes = std::fs::read(&path).unwrap();
        let mut out_bytes = Vec::new();

        expected_index.write_to(&mut out_bytes, options).unwrap();
        let (out_index, _) = State::from_bytes(&out_bytes, FileTime::now(), decode::Options::default()).unwrap();

        compare_states(&out_index, &expected_index, options, fixture);
        compare_raw_bytes(&out_bytes, &expected_bytes, fixture);
    }
}

#[test]
fn v2_index_no_extensions() {
    let input = [
        "V2_empty",
        "v2",
        "v2_more_files",
        "v2_split_index",
        "v4_more_files_IEOT",
    ];

    for fixture in input {
        let path = crate::fixture_index_path(fixture);
        let expected = git_index::File::at(&path, decode::Options::default()).unwrap();

        let mut out = Vec::<u8>::new();
        let options = write::Options {
            hash_kind: git_hash::Kind::Sha1,
            version: Version::V2,
            tree_cache_extension: false,
            end_of_index_entry_extension: false,
        };

        expected.write_to(&mut out, options).unwrap();

        let (generated, _) = State::from_bytes(&out, FileTime::now(), decode::Options::default()).unwrap();
        compare_states(&generated, &expected, options, fixture);
    }
}

#[test]
fn v2_index_tree_extensions() {
    let input = [
        "V2_empty",
        "v2",
        "v2_more_files",
        "v2_split_index",
        "v4_more_files_IEOT",
    ];

    for fixture in input {
        let path = crate::fixture_index_path(fixture);
        let expected = git_index::File::at(&path, decode::Options::default()).unwrap();

        let mut out = Vec::<u8>::new();
        let options = write::Options {
            hash_kind: git_hash::Kind::Sha1,
            version: Version::V2,
            tree_cache_extension: true,
            end_of_index_entry_extension: false,
        };

        expected.write_to(&mut out, options).unwrap();

        let (generated, _) = State::from_bytes(&out, FileTime::now(), decode::Options::default()).unwrap();
        compare_states(&generated, &expected, options, fixture);
    }
}

#[test]
fn v2_index_eoie_extensions() {
    let input = [
        "V2_empty",
        "v2",
        "v2_more_files",
        "v2_split_index",
        "v4_more_files_IEOT",
    ];

    for fixture in input {
        let path = crate::fixture_index_path(fixture);
        let expected = git_index::File::at(&path, decode::Options::default()).unwrap();

        let mut out = Vec::<u8>::new();
        let options = write::Options {
            hash_kind: git_hash::Kind::Sha1,
            version: Version::V2,
            tree_cache_extension: false,
            end_of_index_entry_extension: true,
        };

        expected.write_to(&mut out, options).unwrap();

        let (generated, _) = State::from_bytes(&out, FileTime::now(), decode::Options::default()).unwrap();
        compare_states(&generated, &expected, options, fixture);
    }
}

fn compare_states(generated: &State, expected: &State, options: write::Options, fixture: &str) {
    generated.verify_entries().expect("valid");
    generated.verify_extensions(false, no_find).expect("valid");
    assert_eq!(generated.version(), options.version, "version mismatch in {}", fixture);
    assert_eq!(
        generated.tree(),
        match options.tree_cache_extension {
            true => expected.tree(),
            false => None,
        },
        "tree extension mismatch in {}",
        fixture
    );
    assert_eq!(
        generated.entries().len(),
        expected.entries().len(),
        "entry count mismatch in {}",
        fixture
    );
    assert_eq!(
        generated.entries(),
        expected.entries(),
        "entries mismatch in {}",
        fixture
    );
    assert_eq!(
        generated.path_backing(),
        expected.path_backing(),
        "path_backing mismatch in {}",
        fixture
    );
}

fn compare_raw_bytes(generated: &[u8], expected: &[u8], fixture: &str) {
    assert_eq!(generated.len(), expected.len(), "file length mismatch in {}", fixture);

    let print_range = 10;
    for (index, (a, b)) in generated.iter().zip(expected.iter()).enumerate() {
        if a != b {
            let range_left = max(index - print_range, 0);
            let range_right = min(index + print_range, generated.len());
            let generated = &generated[range_left..range_right];
            let expected = &expected[range_left..range_right];

            panic! {"\n\nRoundtrip failed for index in fixture {:?} at position {:?}\n\
            \t   Input: ... {:?} ...\n\
            \tExpected: ... {:?} ...\n\n\
            ", &fixture, index, generated, expected}
        }
    }
}
